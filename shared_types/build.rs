use crux_core::typegen::TypeGen;
use shared::{Account, AccountType, Accounts, CrabNews, Subscriptions};
use std::path::PathBuf;

fn main() -> anyhow::Result<()> {
    println!("cargo:rerun-if-changed=../shared");

    let mut gen = TypeGen::new();

    gen.register_app::<CrabNews>()?;
    gen.register_type::<Account>()?;
    gen.register_type::<AccountType>()?;
    gen.register_type::<Accounts>()?;
    gen.register_type::<Subscriptions>()?;

    let output_root = PathBuf::from("./generated");

    gen.swift("SharedTypes", output_root.join("swift"))?;

    gen.java(
        "com.example.simple_counter.shared_types",
        output_root.join("java"),
    )?;

    Ok(())
}
