#![allow(non_snake_case)]
use lexer::lexer::*;
use std::vec::Vec;
#[derive(Clone)]
pub struct Parser {
    lexer: Lexer,
    ast: Vec<()>,
}
/*
The purpose of the parser is to apply semantic meaning to our language lexemes.
Our parser must therefore perform the following functions

1. Construct semantically correct structures from lexems following the language grammar
2. Identify syntactic errors in source


Fortunately there is a formal grammar specification
for the C0 language. The problems with this grammar is that
it's left recursive which will introduce complications with parsing down the line.
 */

impl Parser {
    pub fn new(file_path: &mut String) -> Parser {
        Parser {
            lexer: Lexer::new(file_path),
            ast: Vec::new(),
        }
    }

    pub fn lexer(self) -> Lexer {
        self.lexer
    }
    // <id> ::= [A-Za-z_][A-Za-z0-9_]*
    pub fn parseId(&mut self){
    }
}




