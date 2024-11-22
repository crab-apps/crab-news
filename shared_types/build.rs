use crux_core::typegen::TypeGen;
use shared::AccountType;
use shared::CrabNews;
use std::path::PathBuf;

fn main() -> anyhow::Result<()> {
    println!("cargo:rerun-if-changed=../shared");

    let mut gen = TypeGen::new();

    gen.register_app::<CrabNews>()?;

    gen.register_type::<AccountType>()?;

    let output_root = PathBuf::from("./generated");

    gen.swift("SharedTypes", output_root.join("swift"))?;

    gen.java("shared_types", output_root.join("java"))?;

    Ok(())
}
