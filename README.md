# Pcode Parser
Parser for raw (low-level) Pcode generated from Ghidra, written in Rust.

## Example in main.rs
The program parses a Pcode program like this one:
```
0x400f9c
(unique,0x5380,1) = LOAD (const,0x55706c52be80,8) (register,0x0,8)
(register,0x200,1) = INT_CARRY (unique,0x5380,1) (register,0x0,1)
[...]
0x400f9e
```  
as follows:
```
Address: 0x400f9c
Inst { opcode: Load, output: Some(Varnode { var: Unique(0x5380), size: Byte }), inputs: [Varnode { var: Const("0x5643218c0ff0"), size: Quad }, Varnode { var: Register(0), size: Quad }] }
Inst { opcode: IntCarry, output: Some(Varnode { var: Register(512), size: Byte }), inputs: [Varnode { var: Unique(0x5380), size: Byte }, Varnode { var: Register(0), size: Byte }] }
[...]
Address: 0x400f9e
```

## Usage
You can generate the raw Pcode of a binary using [Pcode-generator](https://github.com/kajaaz/pcode-generator/tree/main) and then use Pcode-parser to parse the produced pcode. 

###  Credits and Acknowledgments
This Rust program was developed with the assistance and contributions of the following individuals and organizations:
* @cchr-ledger : For providing the template for this parser, which was crucial for the successful completion of this project.
* Michael Chesser (@mchesser) and Rubens Brandao (@rbran) : For their extensive knowledge in binaries parsing and Rust language, which greatly enhanced the functionality of this program.

