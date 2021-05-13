mod lexer;
mod token;
mod position_container;
mod position_reader;
mod error;
mod ast;
mod parser;

use std::io::{stdin, stdout, Write};

struct StdinReader{
    line_nr: usize,
}

impl StdinReader {
    fn new() -> Self {
        Self {
            line_nr: 1,
        }
    }
}

impl Iterator for StdinReader {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        print!("mylang [{}]: ", self.line_nr);
        stdout().flush().unwrap();
        let mut line = String::new();
        stdin().read_line(&mut line).unwrap();
        self.line_nr += 1;
        Some(line)
    }
}

fn main() {
    let stdin_reader = StdinReader::new();
    let lexer = lexer::Lexer::new(stdin_reader);
    let parser = parser::Parser::new(lexer);
    for parse_result in parser {
        println!("Parse Result: {:#?}", parse_result);
    }
}
