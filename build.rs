extern crate gcc;
extern crate pkg_config;

#[cfg(not(feature = "bundled"))]
fn main() {
    if pkg_config::find_library("sqlite3").is_err() {
        gcc::compile_library("libsqlite3.a", &["source/sqlite3.c"]);
    }
}

#[cfg(feature = "bundled")]
fn main() {
    gcc::compile_library("libsqlite3.a", &["source/sqlite3.c"]);
}
