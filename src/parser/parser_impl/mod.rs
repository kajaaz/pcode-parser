/*
    Credits and Acknowledgments
    
    This Rust program was developed with the assistance and contributions of the following individuals and organizations:
    
    - @cchr-ledger : For providing the template for this parser, which was crucial for the successful completion of this project.
    - Michael Chesser and Rubens Brandao : For their extensive knowledge in binaries parsing and Rust language, which greatly enhanced the functionality of this program.

    Their expertise and support have been invaluable.
*/

use std::collections::BTreeMap;

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
            "BUILD" => Ok(Self::Build),
            "CALL" => Ok(Self::Call),
            "CALLIND" => Ok(Self::CallInd),
            "CALLOTHER" => Ok(Self::CallOther), 
            "CBRANCH" => Ok(Self::CBranch),
            "CEIL" => Ok(Self::Ceil),
            "COPY" => Ok(Self::Copy),
            "CPOOLREF" => Ok(Self::CPoolRef), 
            "CROSSBUILD" => Ok(Self::CrossBuild), 
            "DELAYSLOT" => Ok(Self::DelaySlot), 
            "FLOAT2FLOAT" => Ok(Self::Float2Float), 
            "FLOAT_ABS" => Ok(Self::FloatAbs), 
            "FLOAT_ADD" => Ok(Self::FloatAdd), 
            "FLOAT_DIV" => Ok(Self::FloatDiv), 
            "FLOAT_EQUAL" => Ok(Self::FloatEqual), 
            "FLOAT_LESS" => Ok(Self::FloatLess), 
            "FLOAT_LESSEQUAL" => Ok(Self::FloatLessEqual), 
            "FLOAT_MULT" => Ok(Self::FloatMult), 
            "FLOAT_NAN" => Ok(Self::FloatNaN), 
            "FLOAT_NEG" => Ok(Self::FloatNeg), 
            "FLOAT_NOTEQUAL" => Ok(Self::FloatNotEqual),
            "FLOAT_SQRT" => Ok(Self::FloatSqrt), 
            "FLOAT_SUB" => Ok(Self::FloatSub), 
            "FLOAT_FLOOR" => Ok(Self::FloatFloor), 
            "INT2FLOAT" => Ok(Self::Int2Float),
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
            "INT_REM" => Ok(Self::IntRem), 
            "INT_RIGHT" => Ok(Self::IntRight),
            "INT_SBORROW" => Ok(Self::IntSBorrow),
            "INT_SCARRY" => Ok(Self::IntSCarry),
            "INT_SDIV" => Ok(Self::IntSDiv),
            "INT_SEXT" => Ok(Self::IntSExt),
            "INT_SLESS" => Ok(Self::IntSLess),
            "INT_SLESSEQUAL" => Ok(Self::IntSLessEqual),
            "INT_SREM" => Ok(Self::IntSRem), 
            "INT_SRIGHT" => Ok(Self::IntSRight), 
            "INT_SUB" => Ok(Self::IntSub),
            "INT_XOR" => Ok(Self::IntXor),
            "INT_ZEXT" => Ok(Self::IntZExt),
            "LABEL" => Ok(Self::Label), 
            "LOAD" => Ok(Self::Load),
            "LZCOUNT" => Ok(Self::LZCount), 
            "NEW" => Ok(Self::New),
            "PIECE" => Ok(Self::Piece),
            "POPCOUNT" => Ok(Self::PopCount), 
            "RETURN" => Ok(Self::Return),
            "ROUND" => Ok(Self::Round), 
            "SEGMENTOP" => Ok(Self::SegmentOp),
            "STORE" => Ok(Self::Store),
            "SUBPIECE" => Ok(Self::SubPiece),
            "TRUNC" => Ok(Self::Trunc), 
            "UNUSED1" => Ok(Self::Unused1),     
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
            "16" => Ok(Self::DoubleQuad),
            "32" => Ok(Self::QuadQuad),
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
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Trim the leading and trailing parentheses before splitting
        let trimmed = s.trim_matches(|p| p == '(' || p == ')');
        let def: Vec<&str> = trimmed.split(',').map(|s| s.trim()).collect();

        // Ensure we have exactly 3 components; otherwise, return an error
        if def.len() != 3 {
            return Err(format!("Unexpected number of components in varnode definition: '{}'. Expected 3, found {}", trimmed, def.len()));
        }

        let var_type = def[0];
        let addr_str = def[1].trim_start_matches("0x");
        let size_str = def[2];

        let size = match size_str {
            "1" => Size::Byte,
            "2" => Size::Half,
            "4" => Size::Word,
            "8" => Size::Quad,
            "16" => Size::DoubleQuad,
            "32" => Size::QuadQuad,
            _ => return Err(format!("Invalid size in varnode definition: '{}'", size_str)),
        };

        let var = match var_type {
            "register" => Var::Register(u64::from_str_radix(addr_str, 16).map_err(|_| format!("Failed to parse register address: '{}'", addr_str))?, size),
            "ram" => Var::Memory(u64::from_str_radix(addr_str, 16).map_err(|_| format!("Failed to parse memory address: '{}'", addr_str))?),
            "unique" => Var::Unique(u64::from_str_radix(addr_str, 16).map_err(|_| format!("Failed to parse unique address: '{}'", addr_str))?),
            "const" => Var::Const(def[1].to_string()),
            _ => return Err(format!("Unknown varnode type '{}'", var_type)),
        };

        Ok(Varnode { var, size })
    }
}

impl std::str::FromStr for Inst {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("0x") {
            return Err("Address line encountered in instruction parsing context".to_string());
        }

        // Determine if there's an output by checking if there's an '=' in the instruction
        let has_output = s.contains("=");
        let parts = s.splitn(2, '=').map(str::trim).collect::<Vec<&str>>();

        if has_output && parts.len() != 2 {
            return Err("Malformed instruction with '='".to_string());
        }

        let output = if has_output {
            Some(parts[0].parse::<Varnode>().map_err(|e| format!("Error parsing output varnode: '{}', error: {}", parts[0], e))?)
        } else {
            None
        };

        let rest = if has_output { parts[1] } else { parts[0] };
        let opcode_and_inputs: Vec<&str> = rest.split_whitespace().collect();
        if opcode_and_inputs.is_empty() {
            return Err("No opcode found in instruction".to_string());
        }

        let opcode_str = opcode_and_inputs[0];
        let inputs_strs = &opcode_and_inputs[1..];

        let opcode = opcode_str.parse::<Opcode>().map_err(|_| format!("Unrecognized opcode: '{}'", opcode_str))?;
        let inputs = inputs_strs.iter()
            .map(|&input_str| input_str.parse::<Varnode>().map_err(|e| format!("Error parsing input varnode: '{}', error: {}", input_str, e)))
            .collect::<Result<Vec<Varnode>, String>>()?;

        Ok(Inst { output, opcode, inputs })
    }
}


impl std::str::FromStr for CodeListing {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut listing = CodeListing(BTreeMap::new());
        let mut current_addr: Option<Addr> = None;

        for line in s.lines() {
            let trimmed_line = line.trim();

            // Check if the line starts with "0x", indicating an address marker
            if trimmed_line.starts_with("0x") {
                match Addr::from_str_radix(&trimmed_line[2..], 16) {
                    Ok(addr) => {
                        // Successfully parsed the address, update current_addr
                        current_addr = Some(addr);
                    },
                    Err(_) => {
                        // Log the error and continue to the next line
                        eprintln!("Invalid address format: {}", trimmed_line);
                        continue;
                    }
                }
            } else if let Some(addr) = current_addr {
                // Process the instruction line and associate it with the current address
                match trimmed_line.parse::<Inst>() {
                    Ok(inst) => {
                        // Add the instruction to the listing under the current address
                        listing.0.entry(addr).or_insert_with(Vec::new).push(inst);
                    },
                    Err(_) => {
                        // Log the error and potentially skip this line
                        eprintln!("Error parsing the instruction line: {}", trimmed_line);
                    }
                }
            } else {
                // Handle the case where an instruction line appears before any address marker
                eprintln!("Instruction line found without preceding address: {}", trimmed_line);
            }
        }

        if listing.0.is_empty() {
            Err("No valid instructions found.".to_string())
        } else {
            Ok(listing)
        }
    }
}





