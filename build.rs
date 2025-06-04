use std::{env, fs};
use std::path::PathBuf;
use cmake::Config;

fn main() {
    // let dst = cmake::build("bridge");
    let dst = Config::new("bridge")
        .define("POCKET_MAX_BUFFER_RESPONSE_SIZE", "10485760")
        .define("POCKET_ENABLE_LOG", "1")
        .define("POCKET_ENABLE_AES", "1")
        .build();
    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-search={}/build", std::env::var("OUT_DIR").unwrap());
    println!("cargo:rustc-link-lib=static=pocketbridge");
    println!("cargo:rustc-link-lib=static=stdc++");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
   
    println!("cargo:rerun-if-changed=bridge/inc/pocket/constants.h");
    println!("cargo:rerun-if-changed=bridge/inc/pocket/pocket.h");
    println!("cargo:rerun-if-changed=bridge/inc/pocket/field.h");
    println!("cargo:rerun-if-changed=bridge/inc/pocket/field_controller.h");
    
    

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("bridge/inc/pocket/constants.h")
        .header("bridge/inc/pocket/pocket.h")
        .header("bridge/inc/pocket/field.h")
        .header("bridge/inc/pocket/field_controller.h")
        .header("bridge/inc/pocket/group.h")
        .header("bridge/inc/pocket/group_controller.h")
        .header("bridge/inc/pocket/user.h")

        .clang_arg("-Ibridge/inc")
        
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");
    
    // Write the bindings to the src/bindings/[os]/[arch]/bindings.rs file.
    let out_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
        .join("src");

    // If the directory doesn't exist, create it
    fs::create_dir_all(&out_path).expect("Unable to create dir");

    // Write bindings to file
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
