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
}
