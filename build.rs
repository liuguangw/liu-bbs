use std::path::Path;
use std::process::Command;

fn main() {
    commit_info();
    build_info();
}
fn commit_info() {
    if !Path::new(".git").exists() {
        return;
    }
    //git log -1 --date=short --format="%H %h %cd" --abbrev=9
    let output = match Command::new("git")
        .arg("log")
        .arg("-1")
        .arg("--date=short")
        .arg("--format=%H %h %cd")
        .arg("--abbrev=9")
        .output()
    {
        Ok(output) if output.status.success() => output,
        _ => return,
    };
    let stdout = String::from_utf8(output.stdout).unwrap();
    let mut parts = stdout.split_whitespace();
    let mut next = || parts.next().unwrap();
    println!("cargo:rustc-env=LIU_BBS_COMMIT_HASH={}", next());
    println!("cargo:rustc-env=LIU_BBS_COMMIT_SHORT_HASH={}", next());
    println!("cargo:rustc-env=LIU_BBS_COMMIT_DATE={}", next())
}
fn build_info() {
    println!(
        "cargo:rustc-env=LIU_BBS_COMPILER_HOST_OS={}",
        os_info::get()
    );
    println!(
        "cargo:rustc-env=LIU_BBS_HOST_TARGET={}",
        std::env::var("TARGET").unwrap()
    );
    //cargo version
    let output = match Command::new("cargo").arg("-V").output() {
        Ok(output) if output.status.success() => output,
        _ => return,
    };
    let stdout = String::from_utf8(output.stdout).unwrap();
    let mut parts = stdout.split_whitespace();
    let mut next = || parts.next().unwrap();
    next();
    println!("cargo:rustc-env=LIU_BBS_CARGO_VERSION={}", next());
}
