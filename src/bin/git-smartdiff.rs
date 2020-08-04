use std::error::Error;
use std::process::Command;

fn call_difftool() -> Result<(), Box<dyn Error>> {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let output = Command::new("git")
                          .arg("difftool")
                          .arg(format!("--extcmd={}/target/debug/main", manifest_dir))
                          .arg("-y")
                          .output()?;
    println!("{}", String::from_utf8_lossy(&output.stdout));
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    call_difftool()
}