pub mod parser;

fn main() {
    let lines = vec![
        "STORE (const,0x5636d09d0d50,8) (register,0x0,4) (unique,0x3b80,1)",
        //"(register,0x200,1) = INT_CARRY (unique,0x5380,1) (register,0x0,1)",
    ];

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
