use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    println!("cargo:rustc-link-search=native={}/dist", dir);
    println!("cargo:rustc-link-lib=nts1mkii");

    if std::env::var("WITH_LOGUE_SDK_BINDGEN").map(|s| s == "true").unwrap_or(false) {
        bindgen_patch();
        bindgen();
    }
}

#[allow(dead_code)]
fn bindgen() {
    let bindings = bindgen::Builder::default()
        .header("src/bindings_libnts1mkii.c")
        .clang_arg("-Icomponents/logue-sdk/platform/nts-1_mkii/common")
        .clang_arg("-Itoolchain/gcc-arm-none-eabi/arm-none-eabi/include")
        .clang_arg("-Icomponents/logue-sdk/platform/ext/CMSIS/CMSIS/Include")
        .clang_arg("-Wno-unknown-attributes")
        .clang_arg("-Wno-implicit-function-declaration")
        .clang_arg("-DSTM32H725xE")
        .clang_arg("-DARM_MATH_CM7")
        .clang_arg("-D__FPU_PRESENT")
        .clang_arg("-DCORTEX_USE_FPU=TRUE")
        .clang_arg("-fkeep-inline-functions")
        .generate_inline_functions(true)
        .blocklist_function("ampdbf")// TODO: log10f
        .blocklist_function("dbampf")// TODO: powf
        .blocklist_function("log10f")
        .blocklist_function("powf")
        .blocklist_function("sqrtf")
        .use_core()
        .ctypes_prefix("core::ffi")
        .blocklist_function("^unit_.+$")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("src/bindings_libnts1mkii.rs"))
        .expect("Couldn't write bindings!");
}

#[allow(dead_code)]
fn bindgen_patch() {
    let dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let cmsis_dir = dir.join("components/logue-sdk/platform/ext/CMSIS");

    let result = Command::new("git")
        .arg("apply")
        .arg(dir.join("script/cmsis_gcc-for-bindgen-clang.patch"))
        .current_dir(&cmsis_dir)
        .output();

    if let Err(err) = result {
        eprintln!("Failed to apply patch: {:?}", err);
    }
}
