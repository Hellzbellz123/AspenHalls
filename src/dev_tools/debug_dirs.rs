pub fn debugdir() {
    let dir = std::env::current_dir()
        .expect("Couldnt get current working directory, No permissions or doesnt exist");

    println!("Current Working Director is: {:?}", dir);
    run(true, true, 2, &dir).expect("could list directory for some reason");
}

use std::error::Error;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

pub enum ANSIColor {
    GREEN,
    YELLOW,
    MAGENTA,
    RED,
    RESET,
    BLACK,
    BLUE,
    WHITE,
    CYAN,
}

impl ANSIColor {
    #[must_use]
    pub fn as_string(&self) -> &str {
        match &self {
            ANSIColor::RED => "\u{001B}[0;31m",
            ANSIColor::GREEN => "\u{001B}[0;32m",
            ANSIColor::BLACK => "\u{001B}[0;30m",
            ANSIColor::YELLOW => "\u{001B}[0;33m",
            ANSIColor::BLUE => "\u{001B}[0;34m",
            ANSIColor::MAGENTA => "\u{001B}[0;35m",
            ANSIColor::CYAN => "\u{001B}[0;36m",
            ANSIColor::WHITE => "\u{001B}[0;37m",
            ANSIColor::RESET => "\u{001B}[0;0m",
        }
    }
}

fn visit_dirs(
    dir: &Path,
    depth: usize,
    level: usize,
    prefix: &str,
    colorize: bool,
    show_all: bool,
) -> io::Result<()> {
    if (level != 0) & (depth == level) {
        return Ok(());
    }

    if dir.is_dir() {
        let entry_set = fs::read_dir(dir)?; // contains DirEntry
        let mut entries = entry_set
            .filter_map(|v| match v.ok() {
                Some(v) => {
                    if show_all {
                        Some(v)
                    } else if v.file_name().to_str()?.starts_with('.') {
                        None
                    } else {
                        Some(v)
                    }
                }
                None => None,
            })
            .collect::<Vec<_>>();
        entries.sort_by(|a, b| a.path().file_name().cmp(&b.path().file_name()));

        for (index, entry) in entries.iter().enumerate() {
            let path = entry.path();

            if index == entries.len() - 1 {
                println!("{}└── {}", prefix, color_output(colorize, &path));
                if path.is_dir() {
                    let depth = depth + 1;
                    let prefix_new = prefix.to_string().clone() + "    ";
                    visit_dirs(&path, depth, level, &prefix_new, colorize, show_all)?;
                }
            } else {
                println!("{}├── {}", prefix, color_output(colorize, &path));
                if path.is_dir() {
                    let depth = depth + 1;
                    let prefix_new = prefix.to_string() + "│   ";
                    visit_dirs(&path, depth, level, &prefix_new, colorize, show_all)?;
                }
            }
        }
    }
    Ok(())
}

fn color_output(colorize: bool, path: &Path) -> std::string::String {
    let filename = path.file_name().unwrap().to_str().unwrap();
    let symlink = match fs::read_link(path) {
        Ok(v) => v,
        Err(_err) => PathBuf::new(),
    };

    let print_name: String = if symlink.to_str().unwrap().is_empty() {
        filename.to_string()
    } else {
        format!("{} -> {}", filename, symlink.to_str().unwrap())
    };

    if !colorize {
        print_name
    } else if path.is_dir() {
        format!(
            "{}{}{}",
            ANSIColor::BLUE.as_string(),
            print_name,
            ANSIColor::RESET.as_string()
        )
    } else if is_executable(path) {
        format!(
            "{}{}{}",
            ANSIColor::YELLOW.as_string(),
            print_name,
            ANSIColor::RESET.as_string()
        )
    } else if path.is_absolute() {
        format!(
            "{}{}{}",
            ANSIColor::RED.as_string(),
            print_name,
            ANSIColor::RESET.as_string()
        )
    } else if path.is_symlink() {
        format!(
            "{}{}{}",
            ANSIColor::WHITE.as_string(),
            print_name,
            ANSIColor::RESET.as_string()
        )
    } else {
        format!(
            "{}{}{}",
            ANSIColor::CYAN.as_string(),
            print_name,
            ANSIColor::RESET.as_string()
        )
    }
}

/// # Errors
/// Will return `Err` if `path` does not exist or the user does not have
/// permission to read it, may also error if theres no where to println too

pub fn run(show_all: bool, colorize: bool, level: usize, dir: &Path) -> Result<(), Box<dyn Error>> {
    visit_dirs(dir, 0, level, "", colorize, show_all)?;
    Ok(())
}

/// Returns `true` if there is a file at the given path and it is
/// executable. Returns `false` otherwise.
///
/// See the module documentation for details.
pub fn is_executable<P>(path: P) -> bool
where
    P: AsRef<Path>,
{
    path.as_ref().is_executable()
}

/// An extension trait for `std::fs::Path` providing an `is_executable` method.
///
/// See the module documentation for examples.
pub trait IsExecutable {
    /// Returns `true` if there is a file at the given path and it is
    /// executable. Returns `false` otherwise.
    ///
    /// See the module documentation for details.
    fn is_executable(&self) -> bool;
}

#[cfg(target_os = "linux")]
mod unix {
    use std::os::unix::fs::PermissionsExt;
    use std::path::Path;

    use super::IsExecutable;

    impl IsExecutable for Path {
        fn is_executable(&self) -> bool {
            let metadata = match self.metadata() {
                Ok(metadata) => metadata,
                Err(_) => return false,
            };
            let permissions = metadata.permissions();
            metadata.is_file() && permissions.mode() & 0o111 != 0
        }
    }
}

#[cfg(target_os = "windows")]
mod windows {
    use super::IsExecutable;
    use std::{os::windows::ffi::OsStrExt, path::Path};
    use winapi::ctypes::{c_ulong, wchar_t};
    use winapi::um::winbase::GetBinaryTypeW;

    impl IsExecutable for Path {
        fn is_executable(&self) -> bool {
            // Check using file extension
            if let Some(pathext) = std::env::var_os("PATHEXT") {
                if let Some(extension) = self.extension() {
                    let extension = extension.to_string_lossy();

                    // Originally taken from:
                    // https://github.com/nushell/nushell/blob/93e8f6c05e1e1187d5b674d6b633deb839c84899/crates/nu-cli/src/completion/command.rs#L64-L74
                    return pathext
                        .to_string_lossy()
                        .split(';')
                        // Filter out empty tokens and ';' at the end
                        .filter(|f| f.len() > 1)
                        .any(|ext| {
                            // Cut off the leading '.' character
                            let ext = &ext[1..];
                            extension.eq_ignore_ascii_case(ext)
                        });
                }
            }

            // Check using file properties
            // This code is only reached if there is no file extension or retrieving PATHEXT fails
            let windows_string = self
                .as_os_str()
                .encode_wide()
                .chain(Some(0))
                .collect::<Vec<wchar_t>>();
            let windows_string_ptr = windows_string.as_ptr();

            let mut binary_type: c_ulong = 42;
            let binary_type_ptr = std::ptr::addr_of_mut!(binary_type);

            let ret = unsafe { GetBinaryTypeW(windows_string_ptr, binary_type_ptr) };
            if binary_type_ptr.is_null() {
                return false;
            }
            if ret != 0 {
                let binary_type = unsafe { *binary_type_ptr };
                match binary_type {
                    0   // A 32-bit Windows-based application
                    | 1 // An MS-DOS-based application
                    | 2 // A 16-bit Windows-based application
                    | 3 // A PIF file that executes an MS-DOS-based application
                    | 4 // A POSIX – based application
                    | 5 // A 16-bit OS/2-based application
                    | 6 // A 64-bit Windows-based application
                    => return true,
                    _ => (),
                }
            }

            false
        }
    }
}