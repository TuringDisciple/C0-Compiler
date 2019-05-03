use std::fs::File;
use std::io::{ BufRead, BufReader };
use std::collections::{VecDeque};
use std::option:: {Option};

enum Type {
      Int,
      Struct, 
}

enum Command {
      If,
      Else,
      While, 
      For, 
      Assert, 
      Error,
      Return,
}
enum Token {
      Id(String),
      LCurly,
      RCurly,
      Equal,
      Equality,
      Bool,
      And,
      Xor,
      Or,
      Mult,
      MultEq,
      Not,
      Mod,
      Div,
      DivEq,
      Plus,
      PlusEq, 
      Minus,
      MinusEq,
      Gt, 
      Lt,
      Gte,
      Lte,
      NotEq,
      Assign,
      LShift,
      LShiftEq,
      RShift,
      RShiftEq,
      LBracket,
      RBracket,
      Dot,
      SemiColon,
      LParen,
      RParen,
      Command,
      Type,
}

fn open_file(path:String) -> BufReader<File>{
    let f = File::open(path).expect( "Unable to open file" );
    let buffered_reader = BufReader::new(f);
    buffered_reader
}

pub fn print_lines(path:String){
    let file_reader = open_file(path);

    for line in file_reader.lines(){
        let line = line.expect( "Unable to read line" );
        println!( "Line: {}", line )
    }
}

pub struct Lexer {
      FilePath: String, 
      Tokens: VecDeque<Token>,
}

impl Lexer {
      pub fn new(FilePath: String) -> Lexer {
            return Lexer {
                  FilePath, 
                  Tokens: VecDeque::new(),
            }
      }

      fn chars(self) -> VecDeque<char>{
            // Lex simple tokens
            let mut chars: VecDeque<char> = VecDeque::new();
            for line in open_file(self.FilePath).lines() {
                  match line {
                        Ok(value) => for c in value.chars() {
                              chars.push_back(c);
                        },
                        Err(_) => break,
                  }
            }
            chars
      }

      fn lex_tokens(self, ) -> VecDeque<Token> {
            let mut chars: VecDeque<char> = self.chars();
            let mut tokens: VecDeque<Token> = VecDeque::new();
            loop {
                  match chars.pop_front() {
                        Some(c) => {
                              match c {
                                    '~' => tokens.push_back(Token::Not), 
                                    '(' => tokens.push_back(Token::LParen),
                                    ')' => tokens.push_back(Token::RParen),
                                    '=' => tokens.push_back(
                                              self.compound_expression(Token::Equal, chars.copy())
                                          )
                                    // TODO: Dealing with remaining tokens that can be something else
                                    _ => break,
                              }
                        }
                        _ => break,
                  }
            }
            tokens
      }

      fn compound_expression(self, head: Token, chars_copy: VecDeque<char>) -> Token {
            let mut tail = chars_copy.pop_front().unwrap();
            let mut tail2 = chars_copy.pop_front().unwrap();
            match head {
                  Equal => {
                        match tail {
                              '=' => Token::Equality,
                              _ => Token::Equal,
                        }
                  }
                  Lt => {
                        match tail {
                              '=' => Token::Lte,
                              '<' => match tail2 { 
                                   '=' => Token::LShiftEq,
                                    _  => Token::LShift
                                    },
                              _ => Token::Lt,
                        }
                  }
                  Gt => {
                        match tail {
                              '=' => Token::Gte,
                              '>' => match tail2 {
                                    '=' => Token::RShiftEq,
                                    _   => Token::RShift, 
                              }, 
                              _   => Token::Gt,
                        }
                  }
                  Not => {
                        match tail {
                              '=' => Token::NotEq,
                              _   => Token::Not,
                        }
                  }
                  Plus => {
                        match tail {
                              '+' | '=' => Token::PlusEq, 
                              _         => Token::Plus
                        }
                  }
                  Minus => {
                        match tail {
                              '-' | '=' => Token::MinusEq,
                              _         => Token::Minus,
                        }
                  }
                  Div => {
                        match tail {
                              '=' => Token::DivEq,
                              _   => Token::Div,
                        }
                  }
                  Mult => {
                        match tail {
                              '=' => Token::MultEq, 
                              _   => Token::Mult,
                        }
                  }
                  Or => {
                        match tail {
                              '=' => Token::Or
                        }
                  }
                  Mod => {

                  }
                  Xor => {

                  }
                  And => {

                  }
                  // TODO: Finish integer ops
                  // TODO: Boolean expressions
            }
      }


}