use std::str::{Chars, FromStr};
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

const NULL_DEST: &'static str = "000";
const EOF: char = '\0';


pub fn assemble_program(text: &str) -> String {
    let mut cursor = text.chars();
    emit_instruction(&mut cursor)
}

fn emit_instruction(cursor: &mut Chars) -> String {
    skip_whitespace(cursor);
    match peek(cursor) {
        '@' => {
            advance(cursor);
            symbol(cursor)
        },
        '(' => {
            advance(cursor);
            let s = symbol(cursor);
            eat(cursor, ')');
            s
        },
        _ => compute_instruction(cursor),
    }
}

fn compute_instruction(cursor: &mut Chars) -> String {
    let dest = parse_dest(cursor);
    println!("Parsed destination: {}", dest);
    let dest_bin;
    if dest.is_empty() {
        dest_bin = NULL_DEST;
    } else if DEST.contains_key(&dest) {
        dest_bin = DEST.get(&dest).unwrap();
    } else {
        panic!("Unknown destination: {}", dest);
    }

    let comp = parse_comp(cursor);
    println!("Parsed computation: {}", comp);
    let comp_bin;
    if comp.is_empty() {
        panic!("Empty computation not allowed");
    } else if COMP.contains_key(&comp) {
        comp_bin = COMP.get(&comp).unwrap();
    } else {
        panic!("Unknown computation: {}", comp);
    }

    let jump = parse_jump(cursor);
    println!("Parsed jump: {}", jump);
    let jump_bin;
    if jump.is_empty() {
        jump_bin = NULL_JUMP;
    } else if JUMP.contains_key(&jump) {
        jump_bin = JUMP.get(&jump).unwrap();
    } else {
        panic!("Unknown jump: {}", jump);
    }

    let mut instruction = String::from(dest_bin);
    instruction.extend(comp_bin.chars().chain(jump_bin.chars()));
    instruction
}

fn parse_comp(cursor: &mut Chars) -> String {
    let mut comp_text = String::new();
    loop {
        match peek(cursor) {
            '0' | '1'| '-' | '+' | '!' | '&' | '|' |'A' | 'D' | 'M'  => {
                comp_text.push(advance(cursor));
            }
            ';' => {
                advance(cursor);
                break;
            }
            _ => {
                break;
            }
        }
    }
    comp_text
}

fn parse_dest(cursor: &mut Chars) -> String {
    let mut dest_text = String::new();
    // Clone the cursor to iterate until we're sure this is a destination, and not a computation
    let mut tmp_cursor = cursor.clone();
    loop {
        match peek(&tmp_cursor) {
            'A' | 'D' | 'M' => {
                dest_text.push(advance(&mut tmp_cursor));
            }
            '=' => {
                advance(&mut tmp_cursor);
                // swap iterators as we're sure this is a destination
                *cursor = tmp_cursor;
                return dest_text;
            }
            _ => {
                // not a destination, return an empty string
                return String::new();
            }
        }
    }
}

fn parse_jump(cursor: &mut Chars) -> String {
    let mut jmp_text = String::new();
    match peek(cursor) {
        'J' => {
            jmp_text.push(advance(cursor));
            jmp_text.push(advance(cursor));
            jmp_text.push(advance(cursor));
        }
        _ => ()
    }
    jmp_text
}

fn eat(cursor: &mut Chars, expect: char) {
    let ch = advance(cursor);
    if ch != expect {
        panic!("Expected {}, but got {}", expect, ch);
    }
}

fn symbol(cursor: &mut Chars) -> String { 
    let mut symbol_text = String::new();
    loop {
        match peek(cursor) {
            'a'..='z' | 'A'..='Z' | '0'..='9' | '_' => {
                symbol_text.push(advance(cursor));
            }
            _ => break,
        }
    }
    symbol_text
}

fn peek(cursor: &Chars) -> char {
    cursor.clone().next().unwrap_or_else(|| EOF)
}

fn peek_next(cursor: &Chars) -> char {
    let mut c = cursor.clone();
    c.next();
    c.next().unwrap_or_else(|| EOF)
}

fn advance(cursor: &mut Chars) -> char {
    cursor.next().unwrap_or_else(|| EOF)
}

fn skip_whitespace(cursor: &mut Chars) {
    loop {
        match peek(cursor) {
            ' ' | '\t' | '\r' | '\n' => {advance(cursor);}
            '/' => {
                if peek_next(cursor) == '/' {
                    loop {
                        match advance(cursor) {
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


fn main() {
    let instruction = assemble_program("@17");
    println!("{}", instruction);

    let instruction = assemble_program("(LOOP)");
    println!("{}", instruction);

    let instruction = assemble_program("0");
    println!("{}", instruction);
}

