use std::fs::File;
use std::io::{ BufRead, BufReader };
use std::collections::{VecDeque};

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
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Token {
      Undefined, 
      LCurly,
      RCurly,
      Equal,
      Equality,
      Bool,
      And,
      AndEq,
      Xor,
      XorEq,
      Or,
      OrEq,
      Mult,
      MultEq,
      Not,
      BooleanNot,
      BitNot,
      BitNotEq,
      Mod,
      ModEq,
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
      BooleanAnd, 
      BooleanOr, 
      Num(u32),
}

pub fn open_file(path:&mut String) -> BufReader<File>{
    let f = File::open(path).expect( "Unable to open file" );
    let buffered_reader = BufReader::new(f);
    buffered_reader
}

pub fn print_lines(path: &mut String){
    let file_reader = open_file(path);

    for line in file_reader.lines(){
        let line = line.expect( "Unable to read line" );
        println!( "Line: {}", line )
    }
}

pub struct Lexer {
      Tokens: VecDeque<Token>,
      Chars: VecDeque<char>,
}

impl Lexer {
      pub fn new(mut FilePath: &mut String) -> Lexer {
            Lexer {
                  Tokens: VecDeque::new(), 
                  Chars: chars(&mut FilePath),
            }
      }
}

pub fn lex_tokens(Chars: &mut VecDeque<char>) -> VecDeque<Token> {
      let mut tokens: VecDeque<Token> = VecDeque::new();
      loop {
            println!("{:?}", Chars);
            match Chars.pop_front() {
                  Some(c) => {
                        match c {
                              '(' => tokens.push_back(Token::LParen),
                              ')' => tokens.push_back(Token::RParen),
                              '~' => tokens.push_back(compound_expr(Token::BitNot,   Chars)), 
                              '=' => tokens.push_back(compound_expr(Token::Equal,    Chars)),
                              '!' => tokens.push_back(compound_expr(Token::Not  ,    Chars)),
                              '+' => tokens.push_back(compound_expr(Token::Plus ,    Chars)),
                              '-' => tokens.push_back(compound_expr(Token::Minus,    Chars)),
                              '&' => tokens.push_back(compound_expr(Token::And  ,    Chars)),
                              '%' => tokens.push_back(compound_expr(Token::Mod  ,    Chars)),
                              '/' => tokens.push_back(compound_expr(Token::Div  ,    Chars)),
                              '*' => tokens.push_back(compound_expr(Token::Mult ,    Chars)), 
                              '<' => tokens.push_back(compound_expr(Token::Lt   ,    Chars)), 
                              '>' => tokens.push_back(compound_expr(Token::Gt   ,    Chars)),
                              '^' => tokens.push_back(compound_expr(Token::Xor  ,    Chars)), 
                              '|' => tokens.push_back(compound_expr(Token::Or   ,    Chars)), 
                              '0'|'1'|'2'|
                              '3'|'4'|'5'|
                              '6'|'7'|'8'|'9' => tokens.push_back(numeric(c, Chars)),


                              // TODO: Dealing with remaining tokens that can be something else
                              ' ' => continue,
                              _ => continue,
                        }
                  }
                  _ => break,
            }
      }
      tokens
}
fn chars(FilePath: &mut String) -> VecDeque<char>{
      // Lex simple tokens
      let mut chars: VecDeque<char> = VecDeque::new();
      for line in open_file(FilePath).lines() {
            match line {
                  Ok(value) => for c in value.chars() {
                        chars.push_back(c);
                  },
                  Err(_) => break,
            }
      }
      chars
}

fn compound_expr(head: Token, chars: &mut VecDeque<char>) -> Token {
      let tail  = chars.pop_front().unwrap_or(' ');
      let tail2 = chars.pop_front().unwrap_or(' ');
      let re_insert = 
      | t1: char, t2: char, cs: &mut VecDeque<char>, ret: Token| -> Token {
            cs.push_front(t2); 
            cs.push_front(t1);
            ret
      };

      let re_insert2 = | t1: char, cs: &mut VecDeque<char>, ret: Token| -> Token {
            cs.push_back(t1);
            ret
      };

      match head {
            Token::Equal => {
                  match tail {
                        '=' => Token::Equality,
                        _   => re_insert(tail, tail2, chars, head),
                  }
            }
            Token::Lt => {
                  match tail {
                        '=' => Token::Lte,
                        '<' => match tail2 { 
                              '=' => Token::LShiftEq,
                              _  => re_insert2(tail2, chars, Token::LShift),
                              },
                        _ => re_insert(tail, tail2, chars, head),
                  }
            }
            Token::Gt => {
                  match tail {
                        '=' => Token::Gte,
                        '>' => match tail2 {
                              '=' => Token::RShiftEq,
                              _   => re_insert2(tail2, chars, Token::RShift), 
                        }, 
                        _   => re_insert(tail, tail2, chars, head),
                  }
            }
            Token::Not => {
                  match tail {
                        '=' => Token::NotEq,
                        '!' => Token::BooleanNot,
                        _   => re_insert(tail, tail2, chars, head),
                  }
            }
            Token::Plus => {
                  match tail {
                        '+' | '=' => Token::PlusEq, 
                        _         => re_insert(tail, tail2, chars, head),
                  }
            }
            Token::Minus => {
                  match tail {
                        '-' | '=' => Token::MinusEq,
                        _         => re_insert(tail, tail2, chars, head)
                  }
            }
            Token::Div => {
                  match tail {
                        '=' => Token::DivEq,
                        _   => re_insert(tail, tail2, chars, head),
                  }
            }
            Token::Mult => {
                  match tail {
                        '=' => Token::MultEq, 
                        _   => re_insert(tail, tail2, chars, head),
                  }
            }
            Token::Or => {
                  match tail {
                        '=' => Token::OrEq, 
                        '|' => Token::BooleanOr,
                        _   => re_insert(tail, tail2, chars, head),
                  }
            }
            Token::Mod => {
                  match tail {
                        '=' => Token::ModEq,
                        _   => re_insert(tail, tail2, chars, head)
                  }
            }
            Token::Xor => {
                  match tail {
                        '=' => Token::XorEq,
                        _   => re_insert(tail, tail2, chars, head)
                  }
            }
            Token::And => {
                  match tail {
                        '=' => Token::AndEq,
                        '&' => Token::BooleanAnd, 
                        _   => re_insert(tail, tail2, chars, head),
                  }
            }
            Token::BitNot => {
                  match tail {
                        '=' => Token::BitNotEq,
                        _   => re_insert(tail, tail2, chars, head),
                  }
            }
            _ => re_insert(tail, tail2, chars, head)
            // TODO: Finish integer ops
            // TODO: Boolean expressions
      }
}

fn numeric(head: char, chars: &mut VecDeque<char>) -> Token {
      let mut sum: u32 =0;
      sum = head.to_digit(10).unwrap_or(0);
      loop {
            match chars.pop_front() {
                  Some(c) => {
                        match c {
                              '0'|'1'|'2'|'3'|'4'|'5'|'6'|'7'|'8'|'9' =>{
                                    if sum <= (4294967295/10){
                                          sum = 10 * sum + c.to_digit(10).unwrap_or(0);
                                    };
                              },
                              _        => return Token::Num(sum), 
                        }
                  },
                  _   => return Token::Num(sum),
            }
      }
}

#[cfg(test)]
mod test {
      use lexer::lexer::*;
      #[test]
      pub fn lexing_simple_expressions() {
            let mut src_file = String::from("./src/lexer/tests/simple_expressions.c0");
            let mut lexer = Lexer::new(&mut src_file);
            let mut tokens = lex_tokens(&mut lexer.Chars);
            assert_eq!(Token::Plus,   tokens.pop_front().unwrap_or(Token::Undefined));
            assert_eq!(Token::Minus,  tokens.pop_front().unwrap_or(Token::Undefined));
            assert_eq!(Token::Mult,   tokens.pop_front().unwrap_or(Token::Undefined));
            assert_eq!(Token::Lt,     tokens.pop_front().unwrap_or(Token::Undefined));
            assert_eq!(Token::Gt,     tokens.pop_front().unwrap_or(Token::Undefined));
            assert_eq!(Token::Or,     tokens.pop_front().unwrap_or(Token::Undefined));
            assert_eq!(Token::Xor,    tokens.pop_front().unwrap_or(Token::Undefined));
            assert_eq!(Token::BitNot, tokens.pop_front().unwrap_or(Token::Undefined));
            assert_eq!(Token::Div,    tokens.pop_front().unwrap_or(Token::Undefined));
            assert_eq!(Token::Equal,  tokens.pop_front().unwrap_or(Token::Undefined));

      }
      #[test]
      pub fn lexing_expressions() {
            let mut src_file = String::from("./src/lexer/tests/expressions.c0");
            let mut lexer = Lexer::new(&mut src_file);
            let mut tokens = lex_tokens(&mut lexer.Chars);
            // println!("{:?}", tokens.len());
            assert_eq!(Token::Mult,       tokens.pop_front().unwrap_or(Token::Undefined));
            assert_eq!(Token::Minus,      tokens.pop_front().unwrap_or(Token::Undefined));
            assert_eq!(Token::Plus,       tokens.pop_front().unwrap_or(Token::Undefined));
            assert_eq!(Token::Div,        tokens.pop_front().unwrap_or(Token::Undefined));
            assert_eq!(Token::Equal,      tokens.pop_front().unwrap_or(Token::Undefined));
            assert_eq!(Token::Equality,   tokens.pop_front().unwrap_or(Token::Undefined));
            assert_eq!(Token::Lt,         tokens.pop_front().unwrap_or(Token::Undefined));
            assert_eq!(Token::Lte,        tokens.pop_front().unwrap_or(Token::Undefined));
            assert_eq!(Token::Gte,        tokens.pop_front().unwrap_or(Token::Undefined));
            assert_eq!(Token::Gt,         tokens.pop_front().unwrap_or(Token::Undefined));
            assert_eq!(Token::NotEq,      tokens.pop_front().unwrap_or(Token::Undefined));
            assert_eq!(Token::Mod,        tokens.pop_front().unwrap_or(Token::Undefined));
            assert_eq!(Token::LShift,     tokens.pop_front().unwrap_or(Token::Undefined));
            assert_eq!(Token::RShift,     tokens.pop_front().unwrap_or(Token::Undefined));
            assert_eq!(Token::And,        tokens.pop_front().unwrap_or(Token::Undefined));
            assert_eq!(Token::Xor,        tokens.pop_front().unwrap_or(Token::Undefined));
            assert_eq!(Token::Or,         tokens.pop_front().unwrap_or(Token::Undefined));
            assert_eq!(Token::BitNot,     tokens.pop_front().unwrap_or(Token::Undefined));
            assert_eq!(Token::LParen,     tokens.pop_front().unwrap_or(Token::Undefined));
            assert_eq!(Token::RParen,     tokens.pop_front().unwrap_or(Token::Undefined));
            assert_eq!(Token::PlusEq,     tokens.pop_front().unwrap_or(Token::Undefined));
            assert_eq!(Token::MinusEq,    tokens.pop_front().unwrap_or(Token::Undefined));
            assert_eq!(Token::DivEq,      tokens.pop_front().unwrap_or(Token::Undefined));
            assert_eq!(Token::MultEq,     tokens.pop_front().unwrap_or(Token::Undefined));
            assert_eq!(Token::OrEq,       tokens.pop_front().unwrap_or(Token::Undefined));
            assert_eq!(Token::LShiftEq,   tokens.pop_front().unwrap_or(Token::Undefined));
            assert_eq!(Token::RShiftEq,   tokens.pop_front().unwrap_or(Token::Undefined));
            assert_eq!(Token::ModEq,      tokens.pop_front().unwrap_or(Token::Undefined));
            assert_eq!(Token::BitNotEq,   tokens.pop_front().unwrap_or(Token::Undefined));
            assert_eq!(Token::AndEq,      tokens.pop_front().unwrap_or(Token::Undefined));
            assert_eq!(Token::XorEq,      tokens.pop_front().unwrap_or(Token::Undefined));
            assert_eq!(Token::PlusEq,     tokens.pop_front().unwrap_or(Token::Undefined));
            assert_eq!(Token::MinusEq,    tokens.pop_front().unwrap_or(Token::Undefined));
            assert_eq!(Token::BooleanAnd, tokens.pop_front().unwrap_or(Token::Undefined));
            assert_eq!(Token::BooleanOr,  tokens.pop_front().unwrap_or(Token::Undefined));
            assert_eq!(Token::BooleanNot, tokens.pop_front().unwrap_or(Token::Undefined));
      }

      #[test]
      pub fn lexing_numerics() {
            let mut src_file = String::from("./src/lexer/tests/numerics.c0");
            let mut lexer = Lexer::new(&mut src_file);
            let mut tokens = lex_tokens(&mut lexer.Chars);

            assert_eq!(Token::Num(1),         tokens.pop_front().unwrap_or(Token::Undefined));
            assert_eq!(Token::Num(12),        tokens.pop_front().unwrap_or(Token::Undefined));      
            assert_eq!(Token::Num(123),       tokens.pop_front().unwrap_or(Token::Undefined));
            assert_eq!(Token::Num(1234),      tokens.pop_front().unwrap_or(Token::Undefined));
            assert_eq!(Token::Num(12345),     tokens.pop_front().unwrap_or(Token::Undefined));
            assert_eq!(Token::Num(123456),    tokens.pop_front().unwrap_or(Token::Undefined));
            assert_eq!(Token::Num(1234567),   tokens.pop_front().unwrap_or(Token::Undefined));
      }
}
