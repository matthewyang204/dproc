// Standard library imports
use std::path::Path;
use std::ffi::OsStr;
use std::process::exit;

// Load crates

// Load internal modules
use dproc::getArgs;
use dproc::utils::csvext::*;
use dproc::utils::delext::*;
use dproc::utils::xlsxext::*;
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
    let rows = read_csv_matrix(&filename).expect("REASON");
    let file_contents: Vec<String> = rows.iter().map(|r| r[col as usize].clone()).collect();
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
    let rows = read_csv_matrix(&filename).expect("REASON");
    let file_contents: Vec<String> = rows[row as usize].clone();
    return file_contents;
}


fn del2csvcol(options: Vec<String>) -> (Vec<String>, i64) {
    let (mut optIndex, mut found) = getStrFromVec(options.clone(), "--index".to_string());
    if !found {
        (optIndex, found) = getStrFromVec(options.clone(), "-i".to_string());
        if !found {
            println!("Error: -i or --index not passed");
            exit(1);
        }
    }
    let numeric_index_pos = optIndex + 1;
    let col = options[numeric_index_pos as usize].parse::<i64>().unwrap_or(-1);

    if col < 0 {
        println!("Error: Invalid index");
        exit(1);
    }
    let infile = options[options.len() - 2].clone();
    let raw = read_space_delimited_values(&infile).expect("REASON");
    let file_contents: Vec<String> = raw[0].split_whitespace().map(|s| s.to_string()).collect();
    (file_contents, col)
}

fn del2csvrow(options: Vec<String>) -> (Vec<String>, i64) {
    let (mut optIndex, mut found) = getStrFromVec(options.clone(), "--index".to_string());
    if !found {
        (optIndex, found) = getStrFromVec(options.clone(), "-i".to_string());
        if !found {
            println!("Error: -i or --index not passed");
            exit(1);
        }
    }
    let numeric_index_pos = optIndex + 1;
    let row = options[numeric_index_pos as usize].parse::<i64>().unwrap_or(-1);

    if row < 0 {
        println!("Error: Invalid index");
        exit(1);
    }
    let infile = options[options.len() - 2].clone();
    let raw = read_space_delimited_values(&infile).expect("REASON");
    let file_contents: Vec<String> = raw[0].split_whitespace().map(|s| s.to_string()).collect();
    (file_contents, row)
}

fn singlexlsx2csv(options: Vec<String>) {
    let infile = options[0].clone();
    let outfile = options[1].clone();
    let sheet_name = options[2].clone();

    let mut workbook = load_workbook(&infile).expect("Failed to load workbook");
    write_single_xlsx(&mut workbook, &sheet_name, &outfile).expect("Failed to write CSV");
}

fn xlsxcontainer2csv(options: Vec<String>) {
    let infile = options[0].clone();
    let output_dir = options[1].clone();

    let mut workbook = load_workbook(&infile).expect("Failed to load workbook");
    write_xlsxcontainer(&mut workbook, &output_dir).expect("Failed to write CSV files");
}

fn xlsx2csv(options: Vec<String>){
    let infile = options[0].clone();
    let outfile = options[1].clone();

    let mut workbook = load_workbook(&infile).expect("Failed to load workbook");
    let sheet_names = get_sheet_names(&mut workbook);

    if sheet_names.len() == 1 {
        let sheet_name = sheet_names[0].clone();
        write_single_xlsx(&mut workbook, &sheet_name, &outfile).expect("Failed to write CSV");
    } else {
        write_xlsxcontainer(&mut workbook, &outfile).expect("Failed to write CSV files");
    }
}

fn version() {
    println!("dfmtutils (dproc package utilities), version 1.3.0");
    println!("Copyright (C) 2025-2026 Matthew Yang (杨佳明)");
    println!("Format conversion utility for dproc");
}

fn help() {
    println!("dfmtutils — format conversion utility for dproc");
    println!();
    println!("Usage:");
    println!("  dfmtutils <command> [options] <input> <output>");
    println!();
    println!("Commands:");
    println!("  csvcol2del      Extract a CSV column into space-delimited format");
    println!("  csvrow2del      Extract a CSV row into space-delimited format");
    println!("  del2csvcol      Insert space-delimited values into a CSV column");
    println!("  del2csvrow      Insert space-delimited values into a CSV row");
    println!("  singlexlsx2csv  <standard options> <sheet_name>  Convert a single sheet from an XLSX file to CSV");
    println!("  xlsxcontainer2csv  Convert all sheets from an XLSX file to CSV files in a directory");
    println!("  xlsx2csv        Convert an XLSX file to CSV format (single sheet or multiple sheets)");
    println!();
    println!("Options:");
    println!("  -h, --help      Show this help message and exit");
    println!("  -v, --version   Show version information and exit");
    println!("  -i, --index N   Select the row or column index (0-based)");
    println!();
    println!("Examples:");
    println!("  dfmtutils csvcol2del -i 3 input.csv");
    println!("  dfmtutils csvrow2del -i 0 input.csv");
    println!("  dfmtutils del2csvcol -i 2 values.txt output.csv");
    println!("  dfmtutils del2csvrow -i 1 values.txt output.csv");
    println!("  dfmtutils xlsx2csv input.xlsx output.csv");
}

fn main() {
    let args = getArgs();
    let argv0 = &args[0];
    let base_argv0 = Path::new(argv0)
        .file_stem()
        .and_then(OsStr::to_str)
        .unwrap_or("");
    if args.contains(&"--help".to_string()) || args.contains(&"-h".to_string()) {
        help();
        exit(0);
    } else if args.contains(&"--version".to_string()) || args.contains(&"-v".to_string()) {
        version();
        exit(0);
    }

    let mut cmdCall = "";
    let options: Vec<String>;
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
        help();
        exit(1);
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
        let (file_contents, index) = del2csvcol(options.clone());
        let outfile = options[options.len() - 1].clone();
        let mut rows = read_csv_matrix(&outfile).expect("REASON");
        modify_column(&mut rows, index as usize, &file_contents);
        write_csv_matrix(&outfile, &rows).expect("REASON");
    } else if cmdCall == "del2csvrow" {
        let (file_contents, index) = del2csvrow(options.clone());
        let outfile = options[options.len() - 1].clone();
        let mut rows = read_csv_matrix(&outfile).expect("REASON");
        modify_row(&mut rows, index as usize, &file_contents);
        write_csv_matrix(&outfile, &rows).expect("REASON");
    } else if cmdCall == "singlexlsx2csv" {
        singlexlsx2csv(options);
    } else if cmdCall == "xlsxcontainer2csv" {
        xlsxcontainer2csv(options);
    } else if cmdCall == "xlsx2csv" {
        xlsx2csv(options);
    } else {
        help();
        exit(1);
    }
}
