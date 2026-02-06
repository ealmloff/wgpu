#![cfg_attr(any(target_arch = "wasm32", feature = "wasm-bindgen"), no_main)]
#![cfg(not(any(target_arch = "wasm32", feature = "wasm-bindgen")))]

mod cli;
mod human;
mod report;
#[cfg(test)]
mod tests;
mod texture;

fn main() -> anyhow::Result<()> {
    cli::main()
}
