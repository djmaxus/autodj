// FIXME: place this in "utils" folder, probably organize as a crate

use std::{
    error::Error,
    io::{stdout, Write},
    process::Command,
};

fn main() -> Result<(), Box<dyn Error>> {
    let output = Command::new("cargo").arg("fmt").arg("--check").output()?;
    stdout().write_all(&output.stdout).unwrap();
    let status = output.status;
    println!("{status}");
    let code = status.code().ok_or("empty return code")?;
    if code == 0 {
        return Ok(());
    }
    Err(format!("{status}").into())
}
