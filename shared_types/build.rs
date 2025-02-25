use crux_core::typegen::TypeGen;
use shared::{AccountType, CrabNews};
use std::path::PathBuf;

fn main() -> anyhow::Result<()> {
    println!("cargo:rerun-if-changed=../shared");

    let mut gen = TypeGen::new();

    gen.register_samples(vec![
        AccountType::Local,
        AccountType::Apple,
        AccountType::Google,
        AccountType::Microsoft,
        AccountType::Canonical,
    ])?;
    gen.register_app::<CrabNews>()?;

    let output_root = PathBuf::from("./generated");

    gen.swift("SharedTypes", output_root.join("swift"))?;

    gen.java(
        "com.example.simple_counter.shared_types",
        output_root.join("java"),
    )?;

    Ok(())
}
