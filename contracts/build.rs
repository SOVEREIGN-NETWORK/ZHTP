use std::process::Command;
use std::env;
use std::path::Path;

fn main() {
    // Only run wasm-bindgen if targeting wasm32
    if env::var("TARGET").unwrap().contains("wasm32") {
        // Ensure wasm-bindgen-cli is installed
        Command::new("cargo")
            .args(&["install", "wasm-bindgen-cli"])
            .status()
            .expect("Failed to install wasm-bindgen-cli");

        println!("cargo:rerun-if-changed=src/lib.rs");
        
        // Configure wasm-bindgen output
        let out_dir = env::var("OUT_DIR").unwrap();
        let wasm_bindgen_out_dir = Path::new(&out_dir).join("wasm-bindgen");
        
        // Create output directory
        std::fs::create_dir_all(&wasm_bindgen_out_dir)
            .expect("Failed to create wasm-bindgen output directory");

        // Run wasm-bindgen
        Command::new("wasm-bindgen")
            .args(&[
                "--out-dir", wasm_bindgen_out_dir.to_str().unwrap(),
                "--target", "web",
                "--no-typescript",
            ])
            .status()
            .expect("Failed to run wasm-bindgen");

        // Add link to wasm-bindgen runtime
        println!("cargo:rustc-link-lib=wasm-bindgen-runtime");
    }
}