use pcode_generator::low_pcode_generator::generate_low_pcode;

pub mod parser;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let Some([_arg0, in_file]) = TryInto::<[String; 2]>::try_into(args).ok() else {
        eprintln!("Usage: cargo run <path_to_binary_file>");
        return;
    };

    let pcodes = generate_low_pcode(&in_file).unwrap();
    let lines: Vec<String> = pcodes
        .flat_map(|pcode| {
            pcode
                .lines()
                .map(str::to_string)
                .collect::<Vec<String>>()
                .into_iter()
        })
        .collect();

    for line in lines {
        // Use `parse` method to convert the string into `Inst`
        match line.parse::<parser::Inst>() {
            Ok(inst) => {
                // Successfully parsed the line
                println!("Parsed instruction: {:?}", inst);
            }
            Err(_) => {
                println!("Error parsing the line.");
            }
        }
    }
}
