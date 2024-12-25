const APP_NAME: &str = concat!(env!("CARGO_PKG_NAME"), " (version ", env!("CARGO_PKG_VERSION"), ")");

fn main() {
    println!("This is {APP_NAME}.");
}
