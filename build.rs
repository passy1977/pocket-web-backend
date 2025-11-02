use std::{env, fs};
use std::path::PathBuf;
use cmake::Config;

fn main() {
    // Detect build profile
    let profile = env::var("PROFILE").unwrap_or_else(|_| "debug".to_string());
    let is_release = profile == "release";
    
    // Check if logging should be enabled via environment variable
    let enable_logs = env::var("POCKET_ENABLE_LOGS")
        .unwrap_or_else(|_| "0".to_string())
        .parse::<i32>()
        .unwrap_or(0) != 0;
    
    println!("cargo:warning=Building with profile: {} (CMake build type: {})", 
             profile, if is_release { "Release" } else { "Debug" });
    
    if enable_logs {
        println!("cargo:warning=Logging ENABLED via POCKET_ENABLE_LOGS environment variable");
    }

    let mut config = Config::new("bridge");
    
    // Set CMake build type based on Rust profile
    if is_release {
        // Use RelWithDebInfo instead of Release to preserve symbols
        config.define("CMAKE_BUILD_TYPE", "RelWithDebInfo");
        
        // RelWithDebInfo flags with symbol preservation
        config.define("CMAKE_CXX_FLAGS_RELWITHDEBINFO", "-O2 -g -DNDEBUG -fPIC -fno-eliminate-unused-debug-types");
        config.define("CMAKE_C_FLAGS_RELWITHDEBINFO", "-O2 -g -DNDEBUG -fPIC -fno-eliminate-unused-debug-types");
        
        // Safe linker flags
        config.define("CMAKE_EXE_LINKER_FLAGS_RELWITHDEBINFO", "-Wl,--no-gc-sections");
        config.define("CMAKE_SHARED_LINKER_FLAGS_RELWITHDEBINFO", "-Wl,--no-gc-sections");
        
        // Logging control: Enable if POCKET_ENABLE_LOGS is set, otherwise OFF in release
        config.define("POCKET_ENABLE_LOG", if enable_logs { "ON" } else { "OFF" });
        config.define("POCKET_DISABLE_LOCK", "ON");
        config.define("POCKET_ENABLE_TEST", "OFF");
        config.define("POCKET_ENABLE_AES", "ON");
        
        // Disable verbose output in release
        config.very_verbose(false);
        
    } else {
        config.define("CMAKE_BUILD_TYPE", "Debug");
        
        // Debug-specific flags WITHOUT AddressSanitizer
        config.define("CMAKE_CXX_FLAGS_DEBUG", "-g3 -O0 -DDEBUG -Wall -Wextra -fno-omit-frame-pointer");
        config.define("CMAKE_C_FLAGS_DEBUG", "-g3 -O0 -DDEBUG -Wall -Wextra -fno-omit-frame-pointer");
        
        // Enable all logging and debugging features
        config.define("POCKET_ENABLE_LOG", "ON");
        config.define("POCKET_DISABLE_LOCK", "ON");
        config.define("POCKET_ENABLE_TEST", "OFF");
        config.define("POCKET_ENABLE_AES", "OFF");

        // Enable verbose output only in debug mode
        config.define("CMAKE_VERBOSE_MAKEFILE", "ON");
        config.very_verbose(true);
    }

    let dst = config
        .define("POCKET_MAX_BUFFER_RESPONSE_SIZE", "10485760")
        .define("POCKET_FORCE_TIMESTAMP_LAST_UPDATE", "0")
        .build();

    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("{}", env::var("OUT_DIR").unwrap());
    println!("cargo:rustc-link-search={}/build", env::var("OUT_DIR").unwrap());
    println!("cargo:rustc-link-search=native={}/build/pocket-lib/", dst.display());
    
    // Explicit library search paths for release mode
    if is_release {
        println!("cargo:rustc-link-search=native={}/lib", dst.display());
        println!("cargo:rustc-link-search=native={}/build", dst.display());
        println!("cargo:rustc-link-search=native={}/build/pocket-lib", dst.display());
    }
    
    println!("cargo:rustc-link-lib=static=pocketbridge");
    println!("cargo:rustc-link-lib=static=pocket");
    println!("cargo:rustc-link-lib=stdc++");
    println!("cargo:rustc-link-lib=curl");
    println!("cargo:rustc-link-lib=ssl");
    println!("cargo:rustc-link-lib=crypto");
    println!("cargo:rustc-link-lib=sqlite3");
    println!("cargo:rustc-link-lib=tinyxml2");

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
        .header("bridge/inc/pocket-bridge/constants.h")
        .header("bridge/inc/pocket-bridge/field.h")
        .header("bridge/inc/pocket-bridge/field_controller.h")
        .header("bridge/inc/pocket-bridge/group.h")
        .header("bridge/inc/pocket-bridge/group_controller.h")
        .header("bridge/inc/pocket-bridge/group_field.h")
        .header("bridge/inc/pocket-bridge/group_field_controller.h")
        .header("bridge/inc/pocket-bridge/pocket.h")
        .header("bridge/inc/pocket-bridge/user.h")
        .clang_arg("-Ibridge/inc")
        .clang_arg("-Ibridge/pocket-lib/inc")
        .clang_arg("-std=c17")
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
