extern crate cc;
extern crate pkg_config;

fn main() {
    if cfg!(feature = "bundled") || pkg_config::find_library("sqlite3").is_err() {
        cc::Build::new()
            .file("source/sqlite3.c")
            .compile("libsqlite3.a");
    }
}
