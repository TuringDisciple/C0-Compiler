use lexer::lexer::*;
use std::boxed::Box;
use std::collections::VecDeque;

#[derive(Clone, PartialEq)]
pub enum Lexemes {
      Empty,
      Id    (Token, Vec<Token>),
      Num   (Box<Lexemes>     ),
      DecNum(Token, Vec<Token>),
      HexNum(Token, Vec<Token>), 
}

pub struct Parser {
      lexer: Lexer,
}

impl Parser {
      pub fn new(filePath: &mut String) -> Parser {
            Parser {
                  lexer: Lexer::new(filePath),
            }
      }

      pub fn parse(&mut self) -> () {
            // let mut tokens: &mut VecDeque<Token> = &mut self.lexer.tokens();
            // loop {
            //       match tokens.pop_front().unwrap() {
            //             Token::Undefined(Some(_)) => parse_id(tokens),
            //             _ => break,
            //       }
            // }
            ()
      }
}

// <id> ::= [A-Za-z_][A-Za-z0-9_]*
fn parse_id(tokens: &mut VecDeque<Token>) -> () {
      ()
}


#[cfg(test)]
mod test {
      use lexer::lexer::*;

      #[test]
      fn test_parsing_ids() {
            
      }
}