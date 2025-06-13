use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-env-changed=TARGET");
    println!("cargo:rerun-if-env-changed=BINDGEN_EXTRA_CLANG_ARGS");
    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rerun-if-env-changed=BINDGEN_OUTPUT_PATH");

    let out_path = match env::var("BINDGEN_OUTPUT_PATH") {
        Ok(path) => PathBuf::from(path),
        Err(_) => {
            let out_dir = env::var("OUT_DIR").unwrap();
            PathBuf::from(out_dir).join("bindings.rs")
        }
    };

    let header_path: String =
        env::var("PYTHON_HEADER_PATH").expect("PYTHON_HEADER_PATH env var must be set");

    let mut builder = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg(format!("-I{}", header_path))
        .clang_arg(format!("-I{}/../Objects", header_path))
        .clang_arg("-DPy_BUILD_CORE")
        .clang_arg("-D_WIN64")
        .clang_arg("-DMS_WIN64")
        .clang_arg("-D_M_X64")
        .clang_arg("-D_MSC_VER=1900")
        .clang_arg("-DSIZEOF_VOID_P=8")
        .clang_arg("-DSIZEOF_SIZE_T=8")
        .clang_arg("-DSIZEOF_TIME_T=8")
        .clang_arg("--target=x86_64-pc-windows-msvc")
        .clang_arg("-m64")
        .size_t_is_usize(true)
        .derive_default(true)
        .derive_debug(true)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()));

    let types = [
        "PyObject",
        "PyVarObject",
        "PyFrameObject",
        "PyThreadState",
        "PyInterpreterState",
        "PyIntObject",
        "PyLongObject",
        "PyFloatObject",
        "PyComplexObject",
        "PyStringObject",
        "PyUnicodeObject",
        "PyTupleObject",
        "PyDictObject",
        "PyDictEntry",
        "PySetObject",
        "PySetEntry",
        "PyListObject",
        "PyFunctionObject",
        "PyFileObject",
        "PyClassObject",
        "PyInstanceObject",
        "PyMethodObject",
        "PyByteArrayObject",
        "PyCodeObject",
        "PyAddrPair",
        "PyGenObject",
        "PyTaskletObject",
        "PyStacklessState",
        "PyTryBlock",
        "PyTypeObject",
        "PyCFrameObject",
        "PyCFunctionObject",
        "_slp_tasklet",
        "_slp_cstack",
        "_PyErr_StackItem",
        "PyTaskletFlagStruc",
        "Py_complex",
        "PyComplexObject",
    ];

    // for t in &types {
    //     builder = builder.allowlist_type(t);
    // }

    //builder = builder.allowlist_function("Py.*");
    builder = builder.blocklist_function(".*");

    let bindings = builder.generate().expect("Unable to generate bindings");

    println!("Writing bindings to {}", out_path.display());
    bindings
        .write_to_file(&out_path)
        .expect("Couldn't write bindings!");
}
