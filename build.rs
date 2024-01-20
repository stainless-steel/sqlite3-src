fn main() {
    if cfg!(feature = "bundled") || pkg_config::find_library("sqlite3").is_err() {
        let mut build = cc::Build::new();
        build.file("source/sqlite3.c");
        if cfg!(feature = "sqlite-ext-fts5") {
            build.define("SQLITE_ENABLE_FTS5", "");
        }
        build.compile("libsqlite3.a");
    }
}
