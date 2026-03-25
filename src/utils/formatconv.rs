// Standard library imports
use std::error::Error;
use std::fs::File;
use std::io::BufRead;
use std::env;
use std::path::Path;
use std::ffi::OsStr;

// Load crates
use csv;

// Load internal modules
use dproc::getArgs;
use dproc::utils::csvext::*;

fn main() {
    println!("WARNING: Unimplemented function");

    let args = getArgs();
    let argv0 = &args[0];
    let mut base_argv0 = Path::new(argv0)
        .file_stem()
        .and_then(OsStr::to_str)
        .unwrap_or("");
    println!("argv0: {}, base_argv0: {}", argv0, base_argv0);

    let mut cmdCall = "";
    let mut options: Vec<String>;
    if base_argv0 == "dfmtutils"{
        cmdCall = &args[1];
        options = (&args[2..]).to_vec();
    } else {
        cmdCall = base_argv0;
        options = (&args[1..]).to_vec();
    }

    if let Some(index) = cmdCall.find('2') {
        let from = &cmdCall[..index];
        let to = &cmdCall[index + 1..];
        println!("from: {}, to: {}", from, to);
    } else {
        println!("WARNING: Unimplemented function");
    }

    if cmdCall == "csvcol2del" {
        println!("WARNING: Unimplemented function");
    } else if cmdCall == "csvrow2del" {
        println!("WARNING: Unimplemented function");
    } else if cmdCall == "del2csvcol" {
        println!("WARNING: Unimplemented function");
    } else if cmdCall == "del2csvrow" {
        println!("WARNING: Unimplemented function");
    } else {
        println!("WARNING: Unimplemented function");
    }
}
