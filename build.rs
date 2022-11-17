use std::{env, path::PathBuf};

use glob::glob;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    let mut protos: Vec<PathBuf> = Vec::new();

    for entry in glob("proto/**/*.proto").expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => protos.push(path),
            Err(e) => println!("{:?}", e),
        }
    }

    tonic_build::configure()
        .build_server(true)
        .file_descriptor_set_path(out_dir.join("greeter_descriptor.bin"))
        .out_dir("./generated")
        .compile(&protos, &["."])
        .unwrap_or_else(|e| panic!("protobuf compile error: {}", e));
}
