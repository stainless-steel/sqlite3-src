use std::env;

fn main() {
    if cfg!(feature = "bundled") || pkg_config::find_library("sqlite3").is_err() {
        let mut build = cc::Build::new();
        build.file("source/sqlite3.c");
        for (key, value) in env::vars() {
            if key.starts_with("SQLITE_") {
                // Forward SQLITE options and features
                build.define(&key, value.as_str());
            }
        }
        build.compile("libsqlite3.a");
    }
}
