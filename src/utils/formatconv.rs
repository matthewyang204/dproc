// Standard library imports
use std::error::Error;
use std::fs::File;
use std::io::BufRead;
use std::env;
use std::path::Path;
use std::ffi::OsStr;
use std::process::exit;

// Load crates
use csv;

// Load internal modules
use dproc::getArgs;
use dproc::utils::csvext::*;
use dproc::utils::delext::*;
use dproc::getStrFromVec;

fn csvcol2del(options: Vec<String>) -> Vec<String> {
    let (mut optIndex, mut trueOrFalse) = getStrFromVec(options.clone(), "--index".to_string());
    if trueOrFalse == false {
        (optIndex, trueOrFalse) = getStrFromVec(options.clone(), "-i".to_string());
        if trueOrFalse == false {
            println!("Error: -i or --index not passed (UNIMPLEMENTED HANDLER)");
            exit(1);
        }
    }
    let index = optIndex + 1;
    let uIndex = index as usize;
    let col = options.clone()[uIndex].parse::<i64>().unwrap_or(-1);
    if col == -1 {
        println!("Error: Failed to get index, index does not exist, or index is negative (cannot be negative)");
        exit(1);
    }
    println!("{}", col);
    let filename = options[options.len() - 1].clone();
    let file_contents: Vec<String> = read_csv_column(&filename, &(col as f64)).expect("REASON");
    return file_contents;
}

fn csvrow2del(options: Vec<String>)  -> Vec<String> {
    let (mut optIndex, mut trueOrFalse) = getStrFromVec(options.clone(), "--index".to_string());
    if trueOrFalse == false {
        (optIndex, trueOrFalse) = getStrFromVec(options.clone(), "-i".to_string());
        if trueOrFalse == false {
            println!("Error: -i or --index not passed (UNIMPLEMENTED HANDLER)");
            exit(1);
        }
    }
    let index = optIndex + 1;
    let uIndex = index as usize;
    let row = options.clone()[uIndex].parse::<i64>().unwrap_or(-1);
    if row == -1 {
        println!("Error: Failed to get index, index does not exist, or index is negative (cannot be negative)");
        exit(1);
    }
    println!("{}", row);
    let filename = options[options.len() - 1].clone();
    let file_contents: Vec<String> = read_csv_row(&filename, &(row as f64)).expect("REASON");
    return file_contents;
}

fn del2csvcol(options: Vec<String>) -> Vec<String> {
    let (mut optIndex, mut trueOrFalse) = getStrFromVec(options.clone(), "--index".to_string());
    if trueOrFalse == false {
        (optIndex, trueOrFalse) = getStrFromVec(options.clone(), "-i".to_string());
        if trueOrFalse == false {
            println!("Error: -i or --index not passed (UNIMPLEMENTED HANDLER)");
            exit(1);
        }
    }
    let index = optIndex + 1;
    let uIndex = index as usize;
    let col = options.clone()[uIndex].parse::<i64>().unwrap_or(-1);
    if col == -1 {
        println!("Error: Failed to get index, index does not exist, or index is negative (cannot be negative)");
        exit(1);
    }
    let filename = options[options.len() - 1].clone();
    let file_contents: Vec<String> = read_space_delimited_values(&filename).expect("REASON");
    return file_contents;
}

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
        let file_contents = csvcol2del(options);
        for element in file_contents {
            print!("{} ", element);
        }
        println!();
    } else if cmdCall == "csvrow2del" {
        let file_contents = csvrow2del(options);
        for element in file_contents {
            print!("{} ", element);
        }
        println!();
    } else if cmdCall == "del2csvcol" {
        del2csvcol(options);
        for element in file_contents {
            print!("{} ", element);
        }
        println!();
    } else if cmdCall == "del2csvrow" {
        println!("WARNING: Unimplemented function");
    } else {
        println!("WARNING: Unimplemented function");
    }
}
