use std::process::Command;

fn main() {
    let version = match Command::new("git").args(["describe", "--always"]).output() {
        Ok(version) => version,
        Err(err) => panic!("Failed to run `git describe --always`\n Error: {}", err),
    };
    let git_hash = String::from_utf8(version.stdout).unwrap();
    println!("cargo:rustc-env=GIT_VERSION={}", git_hash);
}
