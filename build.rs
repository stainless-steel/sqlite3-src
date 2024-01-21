use std::env;

fn main() {
    if cfg!(feature = "bundled") || pkg_config::find_library("sqlite3").is_err() {
        let mut build = cc::Build::new();
        build.file("source/sqlite3.c");
        for (name, value) in env::vars() {
            if name.starts_with("SQLITE_") {
                build.define(&name, value.as_str());
            }
        }
        build.compile("libsqlite3.a");
    }
}
