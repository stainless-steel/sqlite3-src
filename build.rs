extern crate gcc;
extern crate pkg_config;

fn main() {
    if cfg!(feature = "bundled") || pkg_config::find_library("sqlite3").is_err() {
        gcc::compile_library("libsqlite3.a", &["source/sqlite3.c"]);
    }
}
