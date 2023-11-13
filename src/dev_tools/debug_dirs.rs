use crate::ahp::engine::warn;
use std::{
    error::Error,
    fs, io,
    path::{Path, PathBuf},
};

/// prints current working directory too console
pub fn debug_directory() {
    let dir = match std::env::current_dir() {
        Ok(path) => path,
        Err(e) => {
            warn!("Could not get working directory: {}", e);
            return;
        }
    };

    println!("Current Working Director is: {dir:?}");
    match run(true, true, 2, &dir) {
        Ok(()) => {}
        Err(e) => warn!("{}", e),
    }
}

/// Term Colors that can be used in output
pub enum ANSIColor {
    /// is executable
    Yellow,
    /// is absolute
    Red,
    /// resets terminal color
    Reset,
    /// is directory
    Blue,
    /// is symlink
    White,
    /// everything else
    Cyan,
}

impl ANSIColor {
    /// converts `ANSIColor` enum too string
    #[must_use]
    pub const fn as_string(&self) -> &str {
        match &self {
            Self::Red => "\u{001B}[0;31m",
            Self::Yellow => "\u{001B}[0;33m",
            Self::Blue => "\u{001B}[0;34m",
            Self::Cyan => "\u{001B}[0;36m",
            Self::White => "\u{001B}[0;37m",
            Self::Reset => "\u{001B}[0;0m",
        }
    }
}

/// walk through dirs in path
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
            .filter_map(|v| v.ok().filter(|_v| show_all))
            .collect::<Vec<_>>();
        entries.sort_by(|a, b| {
            a.path().file_name().cmp(&b.path().file_name())
        });

        for (index, entry) in entries.iter().enumerate() {
            let path = entry.path();

            if index == entries.len() - 1 {
                println!("{prefix}└── {}", color_output(colorize, &path));
                if path.is_dir() {
                    let depth = depth + 1;
                    let prefix_new = prefix.to_string().clone() + "    ";
                    visit_dirs(
                        &path,
                        depth,
                        level,
                        &prefix_new,
                        colorize,
                        show_all,
                    )?;
                }
            } else {
                println!("{prefix}├── {}", color_output(colorize, &path));
                if path.is_dir() {
                    let depth = depth + 1;
                    let prefix_new = prefix.to_string() + "│   ";
                    visit_dirs(
                        &path,
                        depth,
                        level,
                        &prefix_new,
                        colorize,
                        show_all,
                    )?;
                }
            }
        }
    }
    Ok(())
}

/// checks and colorizes path based on type
fn color_output(colorize: bool, path: &Path) -> std::string::String {
    let filename = path.file_name().unwrap().to_str().unwrap();
    let symlink = match fs::read_link(path) {
        Ok(v) => v,
        Err(_err) => PathBuf::new(),
    };

    let print_name: String = if symlink.to_str().unwrap().is_empty() {
        filename.to_string()
    } else {
        format!("{filename} -> {}", symlink.to_str().unwrap())
    };

    if !colorize {
        print_name
    } else if path.is_dir() {
        format!(
            "{}{}{}",
            ANSIColor::Blue.as_string(),
            print_name,
            ANSIColor::Reset.as_string()
        )
    } else if is_executable(path) {
        format!(
            "{}{}{}",
            ANSIColor::Yellow.as_string(),
            print_name,
            ANSIColor::Reset.as_string()
        )
    } else if path.is_absolute() {
        format!(
            "{}{}{}",
            ANSIColor::Red.as_string(),
            print_name,
            ANSIColor::Reset.as_string()
        )
    } else if path.is_symlink() {
        format!(
            "{}{}{}",
            ANSIColor::White.as_string(),
            print_name,
            ANSIColor::Reset.as_string()
        )
    } else {
        format!(
            "{}{}{}",
            ANSIColor::Cyan.as_string(),
            print_name,
            ANSIColor::Reset.as_string()
        )
    }
}

/// Walks through given file tree and outputs all entries in the tree
///
/// # Parameters
/// - `show_all`: If set to true, show all entries in the tree
/// - `colorize`: output terminal escape codes for color
/// - `level`: ...
/// - `dir`: root of directory tree
/// # Errors
/// Will return `Error` if `path` does not exist or the user does not have
/// permission to read it, may also error if theres no where too output
pub fn run(
    show_all: bool,
    colorize: bool,
    level: usize,
    dir: &Path,
) -> Result<(), Box<dyn Error>> {
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

/// check if path is executable on linux
#[cfg(target_os = "linux")]
mod linux {
    use std::os::unix::fs::PermissionsExt;
    use std::path::Path;

    use super::IsExecutable;

    impl IsExecutable for Path {
        fn is_executable(&self) -> bool {
            let Ok(metadata) = self.metadata() else {
                return false;
            };
            let permissions = metadata.permissions();
            metadata.is_file() && permissions.mode() & 0o111 != 0
        }
    }
}

/// check if path is executable on windows
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

            let ret = unsafe {
                GetBinaryTypeW(windows_string_ptr, binary_type_ptr)
            };
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
                    unknown_type => warn!("found unknown executable type on windows platform: {}", unknown_type),
                }
            }

            false
        }
    }
}
