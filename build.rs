extern crate pkg_config;

const NAME: &'static str = "sqlite3";

fn main() {
    if pkg_config::find_library(NAME).is_err() {
        println!("cargo:rustc-link-lib=dylib={}", NAME);
    }
}
