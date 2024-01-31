use std::{collections::HashMap, env::var, str::Chars};
use phf::phf_map;

/// acccccc part
#[rustfmt::skip]
static COMP: phf::Map<&'static str, &'static str> = phf_map! {
    "0"   => "0101010",
    "1"   => "0111111",
    "-1"  => "0111010",
    "D"   => "0001100",
    "A"   => "0110000",
    "M"   => "1110000",
    "!D"  => "0001101",
    "!A"  => "0110001",
    "!M"  => "1110001",
    "-D"  => "0001111",
    "-A"  => "0110011",
    "-M"  => "1110011",
    "D+1" => "0011111",
    "A+1" => "0110111",
    "M+1" => "1110111",
    "D-1" => "0001110",
    "A-1" => "0110010",
    "M-1" => "1110010",
    "D+A" => "0000010",
    "D-A" => "0010011",
    "D-M" => "1010011",
    "A-D" => "0000111",
    "M-D" => "1000111",
    "D&A" => "0000000",
    "D&M" => "1000000",
    "D|A" => "0010101",
    "D|M" => "1010101",
};

static JUMP: phf::Map<&'static str, &'static str> = phf_map! {
    "JGT" => "001",
    "JEQ" => "010",
    "JGE" => "011",
    "JLT" => "100",
    "JNE" => "101",
    "JLE" => "110",
    "JMP" => "111",
};

const NULL_JUMP: &'static str = "000";

#[rustfmt::skip]
static DEST: phf::Map<&'static str, &'static str> = phf_map! {
    "M"   => "001",
    "D"   => "010",
    "DM"  => "011",
    "A"   => "100",
    "AM"  => "101",
    "AD"  => "110",
    "ADM" => "111",
};

#[rustfmt::skip]
static BUILTIN_SYMBOLS: phf::Map<&'static str, u32> = phf_map!{
    "R0" => 0,
    "R1" => 1,
    "R2" => 2,
    "R3" => 3,
    "R4" => 4,
    "R5" => 5,
    "R6" => 6,
    "R7" => 7,
    "R8" => 8,
    "R9" => 9,
    "R10" => 10,
    "R11" => 11,
    "R12" => 12,
    "R13" => 13,
    "R14" => 14,
    "R15" => 15,
    "SP" => 0,
    "LCL" => 1,
    "ARG" => 2,
    "THIS" => 3,
    "THAT" => 4,
    "SCREEN" => 0x4000,
    "KBD" => 0x6000,
};

const NULL_DEST: &'static str = "000";
const EOF: char = '\0';

#[derive(Debug)]
enum Instruction {
    Constant(String),
    AddressVariable(String),
    Label(String),
    Compute(String),
}

struct HackAssembler<'a> {
    cursor: Chars<'a>,
    symbol_table: HashMap<String, Option<u32>>,
    current_instruction_address: u32,
}

impl<'a> HackAssembler<'a> {

    pub fn new(text: &str) -> HackAssembler {
        let cursor = text.chars();
        let mut symbol_table= HashMap::new();
        for entry in BUILTIN_SYMBOLS.entries() {
            symbol_table.insert(String::from(*entry.0), Some(*entry.1));
        }
        HackAssembler {
            cursor,
            symbol_table,
            current_instruction_address: 0,
        }
    }

    pub fn assemble_program(&mut self) -> Vec<String> {
        let mut program = Vec::new();
        while let Some(instruction) = self.emit_instruction() {
            match instruction {
                Instruction::Label(_) => (),
                _ => self.current_instruction_address += 1,
            }
            program.push(instruction);
        }
        self.resolve_symbols(program)
    }
    
    fn resolve_symbols(&mut self, program: Vec<Instruction>) -> Vec<String> {
        let mut resolved_program = Vec::new();
        let mut next_free_address = 16;
        for instruction in program {
            match instruction {
                Instruction::AddressVariable(var_name) => {
                    let address = self.symbol_table.get(&var_name).unwrap();
                    if let Some(addr) = address {
                        // This is for already resolved symbols
                        resolved_program.push(format!("0{addr:015b}"));
                    } else {
                        self.symbol_table.insert(var_name.clone(), Some(next_free_address));
                        resolved_program.push(format!("0{next_free_address:015b}"));
                        next_free_address += 1;
                    }
                    
                }
                Instruction::Label(_) => continue,
                Instruction::Compute(binary_instruction) => {
                    resolved_program.push(binary_instruction);
                }
                Instruction::Constant(binary_instruction) => {
                    resolved_program.push(binary_instruction);
                }
            }
        }
        println!("{:?}", self.symbol_table);
        resolved_program
    }

    fn emit_instruction(&mut self) -> Option<Instruction> {
        self.skip_whitespace();
        match self.peek() {
            '@' => {
                self.advance();
                let symbol_text = self.symbol();
                if let Ok(value) = symbol_text.parse::<u32>() {
                    return Some(Instruction::Constant(format!("0{value:015b}")));
                }
                if !self.symbol_table.contains_key(&symbol_text) {
                    self.symbol_table.insert(symbol_text.clone(), None);
                }
                return Some(Instruction::AddressVariable(symbol_text));
            },
            '(' => {
                self.advance();
                let label = self.symbol();
                self.eat(')');
                self.symbol_table.insert(label.clone(), Some(self.current_instruction_address));
                println!("Assigned {} to {}", label, self.current_instruction_address);

                Some(Instruction::Label(label))
            },
            EOF => None,
            _ => {
                Some(Instruction::Compute(self.compute_instruction()))
            },
        }
    }


    fn compute_instruction(&mut self) -> String {
        let dest = self.parse_dest();
        let dest_bin;
        if dest.is_empty() {
            dest_bin = NULL_DEST;
        } else if DEST.contains_key(&dest) {
            dest_bin = DEST.get(&dest).unwrap();
        } else {
            panic!("Unknown destination: {}", dest);
        }

        let comp = self.parse_comp();
        let comp_bin;
        if comp.is_empty() {
            panic!("Empty computation not allowed");
        } else if COMP.contains_key(&comp) {
            comp_bin = COMP.get(&comp).unwrap();
        } else {
            panic!("Unknown computation: {}", comp);
        }

        let jump = self.parse_jump();
        let jump_bin;
        if jump.is_empty() {
            jump_bin = NULL_JUMP;
        } else if JUMP.contains_key(&jump) {
            jump_bin = JUMP.get(&jump).unwrap();
        } else {
            panic!("Unknown jump: {}", jump);
        }
        
        println!("Parsed C-instruction: comp={}, dest={}, jmp={}", comp, dest, jump);

        let mut instruction = String::from("111");
        instruction.extend(
                comp_bin.chars()
                .chain(dest_bin.chars())
                .chain(jump_bin.chars())
        );
        instruction
    }

    fn parse_comp(&mut self) -> String {
        let mut comp_text = String::new();
        loop {
            match self.peek() {
                '0' | '1'| '-' | '+' | '!' | '&' | '|' |'A' | 'D' | 'M'  => {
                    comp_text.push(self.advance());
                }
                ';' => {
                    self.advance();
                    break;
                }
                _ => {
                    break;
                }
            }
        }
        comp_text
    }

    fn parse_dest(&mut self) -> String {
        let mut dest_text = String::new();
        // Save the cursor and iterate until we're sure this is a destination, and not a computation
        // If this is not the destination, swap back the old cursor
        let unchanged_cursor = self.cursor.clone();
        loop {
            match self.peek() {
                'A' | 'D' | 'M' => {
                    dest_text.push(self.advance());
                }
                '=' => {
                    self.advance();
                    return dest_text;
                }
                _ => {
                    // not a destination, return an empty string
                    self.cursor = unchanged_cursor;
                    return String::new();
                }
            }
        }
    }

    fn parse_jump(&mut self) -> String {
        let mut jmp_text = String::new();
        match self.peek() {
            'J' => {
                jmp_text.push(self.advance());
                jmp_text.push(self.advance());
                jmp_text.push(self.advance());
            }
            _ => ()
        }
        jmp_text
    }

    fn eat(&mut self, expect: char) {
        let ch = self.advance();
        if ch != expect {
            panic!("Expected {}, but got {}", expect, ch);
        }
    }

    fn symbol(&mut self) -> String { 
        let mut symbol_text = String::new();
        loop {
            match self.peek() {
                'a'..='z' | 'A'..='Z' | '0'..='9' | '_' => {
                    symbol_text.push(self.advance());
                }
                _ => break,
            }
        }
        symbol_text
    }

    fn peek(&self) -> char {
        self.cursor.clone().next().unwrap_or_else(|| EOF)
    }

    fn peek_next(&self) -> char {
        let mut c = self.cursor.clone();
        c.next();
        c.next().unwrap_or_else(|| EOF)
    }

    fn advance(&mut self) -> char {
        self.cursor.next().unwrap_or_else(|| EOF)
    }

    fn skip_whitespace(&mut self) {
        loop {
            match self.peek() {
                ' ' | '\t' | '\r' | '\n' => {self.advance();}
                '/' => {
                    if self.peek_next() == '/' {
                        loop {
                            match self.advance() {
                                EOF | '\n' => break,
                                _ => continue,
                            }
                        }
                    } else {
                        break
                    }
                },
                _ => break,
            };
        }
    }
}


fn main() {
    let mut assembler = HackAssembler::new(
"
// This file is part of www.nand2tetris.org
// and the book \"The Elements of Computing Systems\"
// by Nisan and Schocken, MIT Press.
// File name: projects/06/rect/Rect.asm

// Draws a rectangle at the top-left corner of the screen.
// The rectangle is 16 pixels wide and R0 pixels high.

   // If (R0 <= 0) goto END else n = R0
   @R0
   D=M
   @END
   D;JLE 
   @n
   M=D
   // addr = base address of first screen row
   @SCREEN
   D=A
   @addr
   M=D
(LOOP)
   // RAM[addr] = -1
   @addr
   A=M
   M=-1
   // addr = base address of next screen row
   @addr
   D=M
   @32
   D=D+A
   @addr
   M=D
   // decrements n and loops
   @n
   M=M-1
   D=M
   @LOOP
   D;JGT
(END)
   @END
   0;JMP
");

    let expected = vec![
        "0000000000000000",
        "1111110000010000",
        "0000000000011000",
        "1110001100000110",
        "0000000000010000",
        "1110001100001000",
        "0100000000000000",
        "1110110000010000",
        "0000000000010001",
        "1110001100001000",
        "0000000000010001",
        "1111110000100000",
        "1110111010001000",
        "0000000000010001",
        "1111110000010000",
        "0000000000100000",
        "1110000010010000",
        "0000000000010001",
        "1110001100001000",
        "0000000000010000",
        "1111110010001000",
        "1111110000010000",
        "0000000000001010",
        "1110001100000001",
        "0000000000011000",
        "1110101010000111",
    ];
    let program = assembler.assemble_program();
    assert_eq!(program, expected);
    for i in program {
        println!("{}", i);
    }
    
}

