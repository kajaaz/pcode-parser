use std::{fs::{File, OpenOptions}, io::{self, BufRead}, path::Path};
use std::io::Write;

use parser::parser::Inst;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: cargo run <path_to_binary_file>");
        return;
    }

    let in_file = &args[1];
    pcode_generator::low_pcode_generator::generate_low_pcode(in_file);

    let output_file_name = Path::new(in_file)
        .file_stem()
        .expect("Failed to extract file stem")
        .to_str()
        .expect("Failed to convert file stem to string");
    
    // MODIFY PATH  
    let output_file_path = format!("/path/to/pcode-parser/results/{}_low_pcode.txt", output_file_name);

    if let Ok(file) = File::open(&output_file_path) {
        let lines = io::BufReader::new(file).lines();
        let mut instructions_with_address = Vec::new();
        let mut last_address: Option<String> = None;

        for line in lines.filter_map(|result| result.ok()) {
            if line.trim_start().starts_with("0x") {
                // Check if this is a new address block
                let current_address = line.trim().to_string();
                if last_address.as_ref() != Some(&current_address) {
                    // New address found, print it and update last_address
                    instructions_with_address.push(format!("Address: {}", current_address));
                    last_address = Some(current_address);
                }
            } else if last_address.is_some() {
                // Parse instruction and add it directly, no need to repeat the address
                match line.parse::<Inst>() {
                    Ok(inst) => {
                        instructions_with_address.push(format!("{:?}", inst));
                    },
                    Err(_) => eprintln!("Error parsing the instruction line: {}", line),
                }
            }
        }

        // Write the collected instructions to the output file
        if let Ok(mut file) = OpenOptions::new().write(true).truncate(true).open(&output_file_path) {
            for line in instructions_with_address {
                if let Err(e) = writeln!(file, "{}", line) {
                    eprintln!("Failed to write to file: {}", e);
                    return;
                }
            }
        } else {
            eprintln!("Failed to open output file for writing: {}", output_file_path);
        }
    } else {
        eprintln!("Failed to open output file: {}", output_file_path);
    }
}
