#![allow(proc_macro_derive_resolution_fallback)]

#[get("/")]
pub fn index() -> &'static str {
    "Hello, world!"
}
#[get("/myrocket")]
pub fn myrocket() -> String {
    "My 🚀 server".to_string()
}
#[get("/login")]
pub fn login() -> String {
    "login".to_string()
}
