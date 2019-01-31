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
      Bool,
      And,
      Xor,
      Or,
      Mult,
      Not,
      Mod,
      Div,
      Plus, 
      Minus,
      Gt, 
      Lt,
      Gte,
      Lte,
      Ne,
      Assign,
      LShift,
      RShift,
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
                                    '+' => tokens.push_back(Token::Plus),
                                    '-' => tokens.push_back(Token::Minus),
                                    '/' => tokens.push_back(Token::Div),
                                    '^' => tokens.push_back(Token::Xor),
                                    '|' => tokens.push_back(Token::Or),
                                    '~' => tokens.push_back(Token::Not),
                                    '%' => tokens.push_back(Token::Mod),
                                    // TODO: Dealing with remaining tokens that can be something else
                                    _ => break,
                              }
                        }
                        _ => break,
                  }
            }
            tokens
      }


}