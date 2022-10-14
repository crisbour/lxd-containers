use lxd::{Container, Location};
use anyhow::Result;

fn main() -> Result<()> {
    let mut container = Container::new(Location::Local, "test-new", "ubuntu:16.04")?;
    println!("Container {} created", container.name());
    Ok(())
}
