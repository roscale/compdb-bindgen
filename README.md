# compdb-bindgen

Easily acquire the required clang flags for bindgen from the compilation database.

You don't have to mess anymore with CMake or other build systems to collect the appropriate flags and pass them to the build script.
Let your build system generate a compilation database and this crate will extract the flags from it.

## Example

1. Add this crate under `[build-dependencies]`.
2. Create a `build.rs`. Here's an example:

```rs
use std::env;
use std::fs::read_to_string;
use std::path::PathBuf;

fn main() {
    // Read the compilation database.
    let comp_db = read_to_string("build/compile_commands.json")
        .expect("Failed to read the compilation database");

    // Can be any source file from your project, or an empty one just for this purpose.
    let flags = compdb_bindgen::get_bindgen_flags(&comp_db, |file| file.ends_with("bindgen.c"))
        .expect("Failed to get the bindgen flags");

    // Generate bindings using bindgen.
    let bindings = bindgen::Builder::default()
        .header("bindings.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .clang_args(flags) // Add the flags like this.
        .generate()
        .expect("Failed to generate bindings");

    // Write the bindings to disk.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Failed to write bindings to disk");
}
```
