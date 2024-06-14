use std::fs::OpenOptions;
use std::io::{Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};

const UNIT_NAME: [&str; 2] = [
    "osc_waves",
    "osc_dummy",
];

///
/// Copy the compiled unit files to the dist directory and perform necessary operations.
///
pub fn dist() {
    let project_root = project_root();
    let target_dir = project_root.join("target/thumbv7em-none-eabihf/release");
    let dist_dir = project_root.join("dist");

    // Copy unit files to dist
    for unit_name in UNIT_NAME.iter() {
        let src = target_dir.join(unit_name);
        let dst = dist_dir.join(unit_name);
        std::fs::copy(src, dst).unwrap();
    }

    // Patch ELF files
    for unit_name in UNIT_NAME.iter() {
        let dst = dist_dir.join(unit_name);
        patch_elf(dst).unwrap(); // TODO: handle error
    }

    // Rename unit files
    for unit_name in UNIT_NAME.iter() {
        let dst = dist_dir.join(unit_name);
        let dst = dst.with_extension("nts1mkiiunit");
        std::fs::rename(dist_dir.join(unit_name), dst).unwrap(); // TODO: handle error
    }
}

/// Patch the ELF file located at the given path.
///
/// This function opens the file in read-write mode, seeks to the 7th byte,
/// and writes a single byte of value 0. It returns an `std::io::Result<()>`
/// indicating whether the operation was successful or not.
pub fn patch_elf(path: PathBuf) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(path)?;

    file.seek(SeekFrom::Start(7))?;
    file.write_all(&[0])?;

    Ok(())
}

/// Returns the root directory of the project.
///
/// This function uses the `CARGO_MANIFEST_DIR` environment variable to determine the path
/// of the Cargo manifest file. It then goes up one level in the directory hierarchy
/// to get the root directory of the project. The root directory is returned as a `PathBuf`.
fn project_root() -> PathBuf {
    Path::new(&env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(1)
        .unwrap()
        .to_path_buf()
}
