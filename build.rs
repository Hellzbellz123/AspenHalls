use std::env;
use std::path::Path;
use std::path::PathBuf;

fn main() {
    println!("cargo:warning=Hello from build.rs");

    println!(
        "cargo:warning=CARGO_MANIFEST_DIR is {:?}",
        env::var("CARGO_MANIFEST_DIR").unwrap()
    );

    println!(
        "cargo:warning=PROFILE is {:?}",
        env::var("PROFILE").unwrap()
    );

    compilewindowicons();
    copyassets();
}

fn copyassets() {
    let output_path = get_output_path();
    println!(
        "cargo:warning=Calculated build path: {}",
        output_path.to_str().unwrap()
    );

    let input_path = Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap()).join("assets/*");
    let output_path = Path::new(&output_path).join("assets/*");
    let res = std::fs::copy(input_path, output_path);
    println!("cargo:info={:#?}", res)
}

fn get_output_path() -> PathBuf {
    let target = env::var("TARGET").unwrap();
    println!("cargo:warning=target is: {}", target);

    //<root or manifest path>/target/<profile>/
    let currentworkingdirectory = env::var("CARGO_MANIFEST_DIR").unwrap();
    let build_type = env::var("PROFILE").unwrap();
    let path = Path::new(&currentworkingdirectory)
        .join("target")
        .join(target)
        .join(build_type);
    path
}

fn compilewindowicons() {
    let target = env::var("TARGET").unwrap();
    if target.contains("windows") {
        embed_resource::compile("assets/icons/windows/icon.rc");
    }
}
