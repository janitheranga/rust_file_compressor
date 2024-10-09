# rust_file_compressor
A simple file compressor written in Rust

## Run

#### Example use case -> `book.pdf`.

### 1. `Compress`
1. Add the `compression traget` to the root of the project. As an example use `book.pdf`.
2. Run `cargo run book.pdf compressed_file C`. In here `compressed_file` is the name of the output file and `C` is the command to perform `compress`.

### 2. `Decompress`
1. Add the `decompression target` to the root of the project. As an example use `compressed_file`.
2. Run `cargo run compressed_file decompressed_file.<extension> D`. Here `<extension>` should be the original file extension and `D` command should use to perform `decompress`. As an example `decompressed_file.pdf`(because `book.pdf` is contains `.pdf` extension).
*  So `decompression command` shoud be `cargo run compressed_file decompressed_file.pdf D`
