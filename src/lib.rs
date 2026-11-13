pub use clir_core;
pub use clir_macro::cli;

pub fn args() -> Vec<String> {
    std::env::args().skip(1).collect()
}
