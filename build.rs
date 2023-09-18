use std::env;

fn main() {
    println!("Hello from build.rs");

    println!(
        "CARGO_MANIFEST_DIR is {:?}",
        env::var("CARGO_MANIFEST_DIR").unwrap()
    );

    println!("PROFILE is {:?}", env::var("PROFILE").unwrap());
    embed_window_icons();
    // copy_assets();
}

// fn copy_assets() {
//     let output_path = get_output_path();
//     println!("Calculated build path: {}", output_path.to_str().unwrap());
//
//     let out_dir = env::var("OUT_DIR").unwrap();
//
//     println!("Cargo out dir: {out_dir}");
//
//     let input_path = Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap()).join("gamedata/");
//     let output_path = Path::new(&output_path).join("gamedata/");
//     copy(input_path, output_path).expect("couldn't copy files, maybe the source doesn't exist? {}");
// }

fn embed_window_icons() {
    let target = env::var("TARGET").unwrap();
    if target.contains("windows") {
        println!("embedding icon.rc ");
        embed_resource::compile("gamedata/assets/ico/windows/icon.rc", embed_resource::NONE);
    }
}

// fn get_output_path() -> PathBuf {
//     let target = env::var("TARGET").unwrap();
//     println!("target is: {target}");
//
//     //<root or manifest path>/target/<profile>/
//     let current_working_directory = env::var("CARGO_MANIFEST_DIR").unwrap();
//     let build_type = env::var("PROFILE").unwrap();
//     let path = Path::new(&current_working_directory)
//         .join("target")
//         .join(target)
//         .join(build_type);
//     path
// }

// # Errors
//
// Will return `Err` if `path` does not exist or the user does not have
// permission to read/write.
// pub fn copy<U: AsRef<Path>, V: AsRef<Path>>(from: U, to: V) -> Result<(), std::io::Error> {
//     let mut stack = vec![PathBuf::from(from.as_ref())];
//
//     let output_root = PathBuf::from(to.as_ref());
//     let input_root = PathBuf::from(from.as_ref()).components().count();
//
//     while let Some(working_path) = stack.pop() {
//         println!("process: {:?}", &working_path);
//
//         // Generate a relative path
//         let src: PathBuf = working_path.components().skip(input_root).collect();
//
//         // Create a destination if missing
//         let dest = if src.components().count() == 0 {
//             output_root.clone()
//         } else {
//             output_root.join(&src)
//         };
//         if fs::metadata(&dest).is_err() {
//             println!(" mkdir: {dest:?}");
//             fs::create_dir_all(&dest)?;
//         }
//
//         for entry in fs::read_dir(working_path)? {
//             let entry = entry?;
//             let path = entry.path();
//             if path.is_dir() {
//                 stack.push(path);
//             } else {
//                 match path.file_name() {
//                     Some(filename) => {
//                         let dest_path = dest.join(filename);
//                         println!("  copy: {:?} -> {:?}", &path, &dest_path);
//                         fs::copy(&path, &dest_path)?;
//                     }
//                     None => {
//                         println!("failed: {path:?}");
//                     }
//                 }
//             }
//         }
//     }
//
//     Ok(())
// }
