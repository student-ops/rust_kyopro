use std::fs::OpenOptions;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("./Cargo.toml")?;

    writeln!(file, "\n新たに追加する内容")?;

    Ok(())
}