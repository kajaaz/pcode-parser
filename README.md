# Pcode Parser
Parser for raw (low-level) Pcode generated from Ghidra, written in Rust.

## Example in main.rs
The program parses this Pcode line
```
(unique,0x5380,1) = LOAD (const,0x55e4a78f0330,8) (register,0x0,8)
```  
as follows:
```
Parsed instruction: Inst { opcode: Load, output: Some(Varnode { var: Unique(21376), size: Byte }), inputs: [Varnode { var: Const("0x55e4a78f0330"), size: Quad }, Varnode { var: Register(0), size: Quad }] }
```

## Usage
You can generate the raw Pcode of a binary using [Pcode-generator](https://github.com/kajaaz/pcode-generator/tree/main) and then use Pcode-parser to parse the produced pcode. 

###  Credits and Acknowledgments
This Rust program was developed with the assistance and contributions of the following individuals and organizations:
* @cchr-ledger : For providing the template for this parser, which was crucial for the successful completion of this project.
* Michael Chesser (@mchesser) and Rubens Brandao (@rbran) : For their extensive knowledge in binaries parsing and Rust language, which greatly enhanced the functionality of this program.

