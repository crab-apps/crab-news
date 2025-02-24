use crux_core::typegen::TypeGen;
use shared::{
    Account, AccountType, Accounts, CrabNews, Feeds, FolderName, NewFolder, NewName, OldFolder,
    OldLink, OldName, OpmlFile, OpmlName, Subscription, SubscriptionLink, SubscriptionTitle,
    Subscriptions,
};
use std::path::PathBuf;

fn main() -> anyhow::Result<()> {
    println!("cargo:rerun-if-changed=../shared");

    let mut gen = TypeGen::new();

    gen.register_type::<Account>()?;
    gen.register_type::<AccountType>()?;
    gen.register_type::<Accounts>()?;
    gen.register_type::<Feeds>()?;
    gen.register_type::<FolderName>()?;
    gen.register_type::<NewFolder>()?;
    gen.register_type::<NewName>()?;
    gen.register_type::<OldFolder>()?;
    gen.register_type::<OldLink>()?;
    gen.register_type::<OldName>()?;
    gen.register_type::<OpmlFile>()?;
    gen.register_type::<OpmlName>()?;
    gen.register_type::<Subscription>()?;
    gen.register_type::<SubscriptionLink>()?;
    gen.register_type::<SubscriptionTitle>()?;
    gen.register_type::<Subscriptions>()?;
    gen.register_app::<CrabNews>()?;

    let output_root = PathBuf::from("./generated");

    gen.swift("SharedTypes", output_root.join("swift"))?;

    gen.java(
        "com.example.simple_counter.shared_types",
        output_root.join("java"),
    )?;

    Ok(())
}
