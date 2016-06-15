extern crate pkg_config;
extern crate gcc;


#[cfg(not(feature = "bound"))]
fn main() {
    const NAME: &'static str = "sqlite3";
    if pkg_config::find_library(NAME).is_err() {
        gcc::compile_library("libsqlite3.a", &["sqlite/sqlite3.c"]);
    }
}

#[cfg(feature = "bound")]
fn main() {
    gcc::compile_library("libsqlite3.a", &["sqlite/sqlite3.c"]);
}
