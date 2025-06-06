use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-env-changed=TARGET");
    println!("cargo:rerun-if-env-changed=BINDGEN_EXTRA_CLANG_ARGS");
    println!("cargo:rerun-if-changed=wrapper.h");

    let target = env::var("TARGET").expect("TARGET must be set");

    if target != env::var("HOST").unwrap() {
        println!("cargo:warning=Cross-compiling ({target}), copying pre-generated bindings.rs into OUT_DIR");

        let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
        std::fs::copy(
            "src/bindings_windows_x86_64.rs",
            out_path.join("bindings.rs"),
        )
        .expect("Couldn't copy pre-generated bindings!");

        return;
    }

    // We are building for the host → run bindgen
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let header_path = env::var("PYTHON_HEADER_PATH")
        .expect("PYTHON_HEADER_PATH must be set (e.g. /nix/store/.../include/python2.7)");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg(format!("-I{}", header_path))
        .clang_arg(format!("-I{}/../Objects", header_path))
        .clang_arg("-DPy_BUILD_CORE")
        .allowlist_type("PyObject")
        .allowlist_type("PyVarObject")
        .allowlist_type("PyFrameObject")
        .allowlist_type("PyThreadState")
        .allowlist_type("PyInterpreterState")
        .allowlist_type("PyDictObject")
        .allowlist_type("PyTypeObject")
        .allowlist_type("PyCFrameObject")
        .allowlist_type("PyCFunctionObject")
        .allowlist_type("_slp_tasklet")
        .allowlist_type("_slp_cstack")
        .allowlist_type("_PyErr_StackItem")
        .allowlist_type("PyTaskletFlagStruc")
        .allowlist_function("Py.*")
        .derive_default(true)
        .derive_debug(true)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
