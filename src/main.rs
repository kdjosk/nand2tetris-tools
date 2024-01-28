use std::str::Chars;
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

pub enum InstructionType {
    Address,
    Compute,
    Label,
}

pub struct HackParser<'a> {
    chars: Chars<'a>,
    text: &'a str,
    // current position in the text
    current: usize,
    // start of current token
    start: usize,
    // current line
    line: usize,
}

impl<'a> HackParser<'a> {
    pub fn new(input: &'a str) -> HackParser<'a> {
        HackParser {
            chars: input.chars(),
            text: input,
            current: 0,
            start: 0,
            line: 1,
        }
    }
    
    /// Are there more lines in the input?
    pub fn has_more_lines(&self) -> bool {
        todo!()
    }

    /// Skips over white space and comments, if necessary.
    /// Reads the next instruction from the input.
    /// Should be called only if has_more_lines is true.
    /// Initially there's no current instruction.
    pub fn advance() {
        todo!()
    }

    /// Returns the type of the current instruction
    pub fn instruction_type() -> InstructionType {
        todo!()
    }

    /// If the current instruction is (xxx), returns the symbol xxx.
    /// If the current instruction is @xxx, returns the symbol or decimal xxx (as a string)
    pub fn symbol() -> String {
        todo!()
    }

    /// Returns the symbolic dest part of the current C-instruction (8 possibilities)
    pub fn dest() -> String {
        todo!()
    }

    /// Returns the symbolic comp part of the current C-instruction (28 possibilities)
    pub fn comp() -> String {
        todo!()
    }

    /// Returns the symbolic jump part of the current C-instruction (8 possibilities)
    pub fn jump() -> String {
        todo!()
    }


}

fn main() {
    let scanner = HackParser::new("@17");

}

