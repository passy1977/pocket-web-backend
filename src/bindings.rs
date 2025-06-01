// 
// use std::os::raw::c_int;
// 
// // Genera il codice di binding utilizzando bindgen
// include!(concat!(
// env!("OUT_DIR"),
// "/bindings.rs"
// ));
// 
// fn main() {
//     // Genera il file bindings.rs in OUT_DIR
//     let mut builder = bindgen::Builder::default()
//         .header("path/to/your/header/file.h")  // Includi qui il tuo file di intestazione C
//         .derive_default(true)
//         .layout_tests(false);
// 
//     if cfg!(target_os = "windows") {
//         builder = builder.default_abi(bindgen::Abi::Msvc);
//     } else {
//         builder = builder.(bindgen::Abi::SystemV64);
//     }
// 
//     let bindings = builder
//         .generate()
//         .expect("Unable to generate bindings");
// 
//     // Scrivi i bindings al file di destinazione
//     let out_path = std::path::PathBuf::from(env!("OUT_DIR")).join("bindings.rs");
//     bindings.write_to_file(&out_path).expect("Couldn't write bindings!");
// }