use std::env;
use std::path::PathBuf;

const SELF_PROTO: &str = "../../specs/self/{{ project_name }}.proto";
const SELF_DIR: &str = "../../specs/self";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    println!("cargo:rerun-if-changed={}", SELF_DIR);

    tonic_build::configure()
        .file_descriptor_set_path(out_dir.join("{{ project_name }}.bin"))
        .build_server(false)
        .build_client(true)
        .compile(&[SELF_PROTO],
                 &[SELF_DIR]
        )
        .unwrap();

    Ok(())
}
