/*
    Credits and Acknowledgments
    
    This Rust program was developed with the assistance and contributions of the following individuals and organizations:
    
    - @cchr-ledger : For providing the template for this parser, which was crucial for the successful completion of this project.
    - Michael Chesser and Rubens Brandao : For their extensive knowledge in binaries parsing and Rust language, which greatly enhanced the functionality of this program.

    Their expertise and support have been invaluable.
*/


use crate::parser::{Addr, CodeListing, Inst, Opcode, Size, Value, Var, Varnode};

impl std::str::FromStr for Opcode {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let binding = s.replace("=", "");
        let processed = binding.trim(); // Remove '=' characters and trim
        match processed {
            "BOOL_AND" => Ok(Self::BoolAnd),
            "BOOL_NEGATE" => Ok(Self::BoolNegate),
            "BOOL_OR" => Ok(Self::BoolOr),
            "BOOL_XOR" => Ok(Self::BoolXor),
            "BRANCH" => Ok(Self::Branch),
            "BRANCHIND" => Ok(Self::BranchInd),
            "CALL" => Ok(Self::Call),
            "CALLIND" => Ok(Self::CallInd),
            "CBRANCH" => Ok(Self::CBranch),
            "COPY" => Ok(Self::Copy),
            "INT_2COMP" => Ok(Self::Int2Comp),
            "INT_ADD" => Ok(Self::IntAdd),
            "INT_AND" => Ok(Self::IntAnd),
            "INT_CARRY" => Ok(Self::IntCarry),
            "INT_DIV" => Ok(Self::IntDiv),
            "INT_EQUAL" => Ok(Self::IntEqual),
            "INT_LEFT" => Ok(Self::IntLeft),
            "INT_LESS" => Ok(Self::IntLess),
            "INT_LESSEQUAL" => Ok(Self::IntLessEqual),
            "INT_MULT" => Ok(Self::IntMult),
            "INT_NEGATE" => Ok(Self::IntNegate),
            "INT_NOTEQUAL" => Ok(Self::IntNotEqual),
            "INT_OR" => Ok(Self::IntOr),
            "INT_RIGHT" => Ok(Self::IntRight),
            "INT_SBORROW" => Ok(Self::IntSBorrow),
            "INT_SCARRY" => Ok(Self::IntSCarry),
            "INT_SDIV" => Ok(Self::IntSDiv),
            "INT_SEXT" => Ok(Self::IntSExt),
            "INT_SLESS" => Ok(Self::IntSLess),
            "INT_SLESSEQUAL" => Ok(Self::IntSLessEqual),
            "INT_SUB" => Ok(Self::IntSub),
            "INT_XOR" => Ok(Self::IntXor),
            "INT_ZEXT" => Ok(Self::IntZExt),
            "LOAD" => Ok(Self::Load),
            "RETURN" => Ok(Self::Return),
            "STORE" => Ok(Self::Store),
            "SUBPIECE" => Ok(Self::SubPiece),       
            _ => panic!("Unrecognized opcode \"{}\"", processed),
        }
    }
}

impl std::str::FromStr for Size {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(Self::Byte),
            "2" => Ok(Self::Half),
            "4" => Ok(Self::Word),
            "8" => Ok(Self::Quad),
            _ => panic!("\"{}\" does not correspond to a Size!", s),
        }
    }
}

impl std::str::FromStr for Value {
    type Err = ();

    fn from_str(s_in: &str) -> Result<Self, Self::Err> {
        let s: Vec<&str> = s_in.split(',').map(|s| s.trim()).collect(); // Split and trim each part
        assert_eq!(s.len(), 2);

        let hex_part = s[0].trim_start_matches("0x");

        match (hex_part, s[1]) {
            ("", "8") => Ok(Self::from_quad(0)), // Handle empty hex part as zero
            (s0, "4") => {
                Ok(Self::from_word(u32::from_str_radix(s0, 16).unwrap()))
            }
            (s0, "2") => {
                Ok(Self::from_half(u16::from_str_radix(s0, 16).unwrap()))
            }
            (s0, "1") => {
                Ok(Self::from_byte(u8::from_str_radix(s0, 16).unwrap()))
            }
            _ => panic!("Unable to parse \"{}\" as a Value!", s_in),
        }
    }
}


impl std::str::FromStr for Varnode {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Trim the leading and trailing parentheses before splitting
        let trimmed = s.trim_matches(|p| p == '(' || p == ')');
        let def: Vec<&str> = trimmed.split(',').map(|s| s.trim()).collect(); // Split and trim each part
        assert_eq!(def.len(), 3, "Unexpected number of components in varnode definition: {}", s);

        let var_type = def[0];
        let addr_str = def[1].trim_start_matches("0x");
        let size = def[2].parse().unwrap();

        let var = match var_type {
            "register" => Var::Register(u64::from_str_radix(addr_str, 16).expect("Failed to parse register address")),
            "ram" => Var::Memory(u64::from_str_radix(addr_str, 16).expect("Failed to parse memory address")),
            "unique" => Var::Unique(u64::from_str_radix(addr_str, 16).expect("Failed to parse unique address")),
            "const" => Var::Const(def[1].to_string()), // Directly use the string for const varnodes
            _ => panic!("Unknown varnode type \"{}\"", var_type),
        };

        Ok(Varnode { var, size })
    }
}


impl std::str::FromStr for Inst {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Check if the line is for a STORE operation
        if s.starts_with("STORE") {
            let parts: Vec<&str> = s.split_whitespace().collect(); // Split the line by spaces
            let opcode_str = parts[0]; // The first part is the opcode
            let inputs = parts[1..] // The remaining parts are the inputs
                .iter()
                .map(|&input_str| input_str.parse().unwrap())
                .collect();

            return Ok(Inst {
                output: None, // STORE operations do not have an output in the conventional sense
                opcode: opcode_str.parse().unwrap(),
                inputs,
            });
        }

        // Handle other operations as before
        let parts: Vec<&str> = s.split('=').collect();
        let output_str = parts[0].trim();
        let rest = parts[1].trim();

        let opcode_and_inputs = rest.split_whitespace().collect::<Vec<&str>>();
        let opcode_str = opcode_and_inputs[0]; // First element is the opcode
        let input_strs = opcode_and_inputs[1..].to_vec(); // Remaining elements are the inputs

        let output = if output_str != "-" {
            Some(output_str.trim_matches(|p| p == '(' || p == ')').parse().unwrap())
        } else {
            None
        };

        let opcode = opcode_str.parse().unwrap();
        let inputs = input_strs
            .iter()
            .map(|s| s.trim_matches(|p| p == '(' || p == ')').parse().unwrap())
            .collect();

        Ok(Inst {
            output,
            opcode,
            inputs,
        })
    }
}


impl std::str::FromStr for CodeListing {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut res = CodeListing::new();
        let mut curr_addr: Option<Addr> = None;
        let mut curr_vec: Vec<Inst> = Vec::new();

        for line in s.lines() {
            let line = line.trim();

            match line.chars().nth(0) {
                Some('(') | Some('-') => {
                    curr_vec.push(line.parse().unwrap());
                }
                Some(_) => {
                    let addr = Addr::from_str_radix(&line, 16)
                        .expect("Failed to parse instruction address!");

                    match curr_addr {
                        None => {
                            assert!(curr_vec.is_empty());
                            curr_addr = Some(addr);
                        }
                        Some(_) => {
                            res.0.insert(curr_addr.unwrap(), curr_vec.clone());
                            curr_vec = Vec::new();
                            curr_addr = Some(addr);
                        }
                    }
                }
                _ => panic!("Could not parse input line \"{}\"", line),
            }
        }

        res.0.insert(curr_addr.unwrap(), curr_vec.clone());
        Ok(res)
    }
}


