use std::{env, path::Path};

fn main() {
    let target_family = env::var("CARGO_CFG_TARGET_FAMILY").unwrap();
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();

    if cfg!(feature = "bundled") || pkg_config::find_library("sqlite3").is_err() {
        let mut build = cc::Build::new();
        build.file("source/sqlite3.c");
        for (name, value) in env::vars() {
            if name.starts_with("SQLITE_") {
                build.define(&name, value.as_str());
            }
        }

        if target_family == "wasm" {
            let wasi_sdk_env = env::var("CARGO_CFG_SQLITE3_SRC_WASI_SDK_PATH")
                .or(env::var("WASI_SDK_PATH"))
                .unwrap();
            let wasi_sdk = Path::new(&wasi_sdk_env).canonicalize().unwrap();
            build.compiler(wasi_sdk.join("bin/clang"));
            if target_os != "wasi" {
                let Ok(wasi_env) = env::var("CARGO_CFG_SQLITE3_SRC_WASI_ENV") else {
                    println!("cargo::error=Could not detect WASI environment. Please set the sqlite3_src_wasi_env config flag (e.g. \"p1\").");
                    return;
                };
                build.target(&format!("wasm32-wasi{wasi_env}"));
            }
            build.define("__wasi__", None);
            build.define("SQLITE_OMIT_LOAD_EXTENSION", "1");
            build.define("SQLITE_THREADSAFE", "0");
            build.flag("-Wno-unused");
        }

        build.compile("libsqlite3.a");
    }
}
