use serde::Deserialize;

mod errors;

pub use errors::*;

type CompDb = Vec<CommandObject>;

#[derive(Debug, Deserialize)]
struct CommandObject {
    file: String,
    #[serde(flatten)]
    command: Command,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum Command {
    Command { command: String },
    Arguments { arguments: Vec<String> },
}

pub fn get_bindgen_flags<P: FnMut(&str) -> bool>(
    comp_db: &str,
    mut file_predicate: P,
) -> Result<Vec<String>, Error> {
    let mut comp_db: CompDb = serde_json::from_str(comp_db)?;

    let command_object = comp_db
        .drain(..)
        .find(|command_object| file_predicate(&command_object.file))
        .ok_or(Error::FileNotFound)?;

    match command_object.command {
        Command::Command { command } => get_file_flags(command.split_whitespace()),
        Command::Arguments { arguments } => get_file_flags(arguments.into_iter()),
    }
}

pub fn get_file_flags(
    mut iter: impl Iterator<Item = impl AsRef<str> + Into<String>>,
) -> Result<Vec<String>, Error> {
    let mut args: Vec<String> = vec![];

    let compiler_path = iter.next().ok_or(Error::NotEnoughArguments)?.into();
    args.extend(get_compiler_includes(&compiler_path)?);

    while let Some(arg) = iter.next() {
        let arg = arg.as_ref();

        for &flag in &["-D", "-I", "-std"] {
            if arg == flag {
                // -I path
                args.push(flag.into());
                args.push(iter.next().ok_or(Error::NotEnoughArguments)?.into());
                break;
            } else if arg.starts_with(flag) {
                // -Ipath
                args.push(arg.into());
                break;
            }
        }
    }

    Ok(args)
}

pub fn get_compiler_includes(compiler_path: &str) -> Result<Vec<String>, GetCompilerIncludesError> {
    let command = format!("echo | {compiler_path} -v -E -x c -");

    let output = std::process::Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()?;

    let output = String::from_utf8(output.stderr)?;

    let start_pattern = "#include <...> search starts here:\n";
    let end_pattern = "\nEnd of search list.";

    let start = output
        .find(start_pattern)
        .ok_or(GetCompilerIncludesError::MatchNotFound(
            start_pattern.to_string(),
        ))?
        + start_pattern.len();

    let end = output
        .find(end_pattern)
        .ok_or(GetCompilerIncludesError::MatchNotFound(
            end_pattern.to_string(),
        ))?;

    let includes = &output[start..end];

    Ok(includes
        .split('\n')
        .map(|include| format!("-I{}", include.trim()))
        .collect())
}
