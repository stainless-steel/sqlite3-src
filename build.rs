use std::env;
use std::path::PathBuf;

fn main() {
    if !cfg!(feature = "bundled") && pkg_config::find_library("sqlite3").is_ok() {
        return;
    }

    let target_family = env::var("CARGO_CFG_TARGET_FAMILY").unwrap();

    let mut build = cc::Build::new();
    build.file("source/sqlite3.c");

    for (name, value) in env::vars() {
        if name.starts_with("SQLITE_") {
            build.define(&name, value.as_str());
        }
    }

    if target_family == "wasm" {
        let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
        let path = PathBuf::from(
            env::var("CARGO_CFG_SQLITE3_SRC_WASI_SDK_PATH")
                .or(env::var("WASI_SDK_PATH"))
                .expect("sqlite3_src_wasi_sdk_path or WASI_SDK_PATH should be set"),
        );
        let path = path.canonicalize().expect("the SDK path should be valid");
        build.compiler(path.join("bin/clang"));
        if target_os != "wasi" {
            let target_env = env::var("CARGO_CFG_SQLITE3_SRC_WASI_TARGET_ENV")
                .or(env::var("WASI_TARGET_ENV"))
                .expect("sqlite3_src_wasi_target_env or WASI_TARGET_ENV should be set");
            build.target(&format!("wasm32-wasi{target_env}"));
        }
        build.define("__wasi__", None);
        build.define("SQLITE_OMIT_LOAD_EXTENSION", "1");
        build.define("SQLITE_THREADSAFE", "0");
        build.flag("-Wno-unused");
        build.flag("-Wno-unused-parameter");
    }

    build.compile("libsqlite3.a");
}
