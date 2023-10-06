use std::env;

fn main() {
    println!("Hello from build.rs");

    println!(
        "CARGO_MANIFEST_DIR is {:?}",
        env::var("CARGO_MANIFEST_DIR").unwrap()
    );

    println!("PROFILE is {:?}", env::var("PROFILE").unwrap());
    embed_window_icons();
}

fn embed_window_icons() {
    let target = env::var("TARGET").unwrap();
    if target.contains("windows") {
        println!("embedding icon.rc ");
        embed_resource::compile("res/windows/icon.rc", embed_resource::NONE);
    }
}
