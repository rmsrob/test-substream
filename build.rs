use anyhow::{Ok, Result};
use substreams_ethereum::Abigen;

fn main() -> Result<(), anyhow::Error> {
    Abigen::new("STAKINGV1", "abis/stakingv1.json")?
        .generate()?
        .write_to_file("src/abis/stakingv1.rs")?;

    Ok(())
}