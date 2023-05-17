extern crate cc;
extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:include=DaisySP/Source");
    println!("cargo:rerun-if-changed=wrapper.h");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .clang_arg("-IDaisySP/Source/")
        .clang_arg("-x")
        .clang_arg("c++")
        .clang_arg("-std=c++11")
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    cc::Build::new()
        .cpp(true)
        .includes(&[
                "DaisySP/Source/",
                "DaisySP/Source/Control",
                "DaisySP/Source/Drums",
                "DaisySP/Source/Dynamics",
                "DaisySP/Source/Effects",
                "DaisySP/Source/Filters",
                "DaisySP/Source/Noise",
                "DaisySP/Source/PhysicalModeling",
                "DaisySP/Source/Synthesis",
                "DaisySP/Source/Utility",
        ])
        .files(&[
                "DaisySP/Source/PhysicalModeling/drip.cpp",
                "DaisySP/Source/PhysicalModeling/KarplusString.cpp",
                "DaisySP/Source/PhysicalModeling/modalvoice.cpp",
                "DaisySP/Source/PhysicalModeling/pluck.cpp",
                "DaisySP/Source/PhysicalModeling/resonator.cpp",
                "DaisySP/Source/PhysicalModeling/stringvoice.cpp",
        ])
        .flag_if_supported("-std=c++11")
        .compile("daisysp");
}
