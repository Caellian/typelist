use variadic_t_codegen::Options;

fn main() {
    let size = std::env::var("VARIADIC_T_SIZE").unwrap_or_else(|_| "12".to_string());
    let size = size.parse().expect("invalid VARIADIC_T_SIZE value");
    // TODO: use OUT_DIR for release if this turns into a crate
    variadic_t_codegen::generate(Options {
        size,
        ..Default::default()
    }, "./src/gen.rs");
}