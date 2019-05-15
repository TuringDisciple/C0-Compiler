use std::fs::File;
use std::io::{ BufRead, BufReader };
use std::collections::{VecDeque, LinkedList, HashMap};

#[derive(Copy, Clone, Debug, PartialEq)]
enum Collection<T> {
      Array(T), 
      Pointer(T),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Token {
      // Character and operators
      Undefined(Option<char>), 
      LCurly,
      RCurly,
      Equal,
      Equality,
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

      // Types
      Int, 
      Bool, 
      Char,
      String_, 
      Void, 
      Func(), 
      Collection,
      // TODO: how to represent structs
      // TODO: How to represent collection of arguments for function types 
      // TODO: 
      // Keywords
      If, 
      While, 
      For, 
      Return, 
      Assert, 
      Error, 
      Alloc, 
      AllocArray,
      Typedef, 
      Break, 
      Continue, 
      Semicolon, 

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
                              ';' => tokens.push_back(Token::SemiColon),
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

                              'i' | 'b' | 'c' | 
                              's' | 'v' | 'a' |
                              'e' | 'r' | 'w' |
                              'f'  => tokens.push_back(keyword(c, Chars)),
                              
                              // TODO: Types
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

fn re_insert(t1: char, t2: char, cs: &mut VecDeque<char>, ret: Token) -> Token {
      cs.push_front(t2); 
      cs.push_front(t1);
      ret
}

fn re_insert2 (t1: char, cs: &mut VecDeque<char>, ret: Token) -> Token {
      cs.push_back(t1);
      ret
}

fn re_insert3(t1: Vec<char>, cs: &mut VecDeque<char>) -> () {
      for i in t1.len()..0 {
            cs.push_front(t1[i]);
      }
}

fn compound_expr(head: Token, chars: &mut VecDeque<char>) -> Token {
      let tail  = chars.pop_front().unwrap_or(' ');
      let tail2 = chars.pop_front().unwrap_or(' ');
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

fn pattern_check(p: Vec<char>, chars: &mut VecDeque<char>) -> Option<Token> {
      let mut buff: Vec<char> = Vec::new();
      for i in 0..p.len() {
            match chars.pop_front() {
                  Some(c) => buff.push(c), 
                  None    => {
                              re_insert3(buff, chars); 
                              return None;
                        }
            }

            if p[i] != buff[i] {
                  re_insert3(buff, chars);
                  return None;
            }
      }

      Some(Token::Undefined(None))
}

fn match_keyword(patterns: &HashMap<Token, Vec<char>>, chars: &mut VecDeque<char>) -> Option<Token>{

      for (t, p) in patterns.iter() {
            match pattern_check(p.to_vec(), chars) {
                  Some(_) => return Some(*t),
                  None    => (),
            }

      }

      return None;
      
}

fn keyword(head: char, chars: &mut VecDeque<char>) -> Token {

      let mut patterns = HashMap::new();
      let check_patterns = |patterns: &HashMap<Token, Vec<char>>, chars: &mut VecDeque<char>| -> Token {
            match match_keyword(patterns, chars) {
                  Some(token) => token, 
                  _           => Token::Undefined(Some(head)),
            }
      };

      match head { 
            'a' => {
                  patterns.insert(Token::Assert, vec!['s', 's', 'e', 'r', 't']);
                  patterns.insert(Token::Alloc, vec!['l', 'l', 'o', 'c']);
                  patterns.insert(Token::AllocArray, vec!['l', 'l', 'o', 'c', '_', 'a', 'r', 'r', 'a','y']);
                  check_patterns(&patterns, chars)
            }

            // 'b' => {
            //       patterns.insert(Token::Bool, vec!['o', 'o', 'l']);
            //       patterns.insert(Token::Break, vec!['r', 'e', 'a', 'k']);
            //       check_patterns(&patterns, chars)
            // }

            // 'c' => {
            //       patterns.insert(Token::Char, vec!['h', 'a', 'r']);
            //       patterns.insert(Token::Continue, vec!['o', 'n', 't', 'i', 'n', 'u', 'e']);
            //       check_patterns(&patterns, chars)
            // }

            // 'e' => {
            //       patterns.insert(Token::Error, vec!['r', 'r', 'o', 'r']);
            //       check_patterns(&patterns, chars)
            // }

            // 'i' => {
            //       patterns.insert(Token::If, vec!['f']);
            //       patterns.insert(Token::Int, vec!['n', 't']);
            //       check_patterns(&patterns, chars)

            // }

            _ => Token::Undefined(Some(head)), 
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
            assert_eq!(Token::Plus,   tokens.pop_front().unwrap_or(Token::Undefined(None)));
            assert_eq!(Token::Minus,  tokens.pop_front().unwrap_or(Token::Undefined(None)));
            assert_eq!(Token::Mult,   tokens.pop_front().unwrap_or(Token::Undefined(None)));
            assert_eq!(Token::Lt,     tokens.pop_front().unwrap_or(Token::Undefined(None)));
            assert_eq!(Token::Gt,     tokens.pop_front().unwrap_or(Token::Undefined(None)));
            assert_eq!(Token::Or,     tokens.pop_front().unwrap_or(Token::Undefined(None)));
            assert_eq!(Token::Xor,    tokens.pop_front().unwrap_or(Token::Undefined(None)));
            assert_eq!(Token::BitNot, tokens.pop_front().unwrap_or(Token::Undefined(None)));
            assert_eq!(Token::Div,    tokens.pop_front().unwrap_or(Token::Undefined(None)));
            assert_eq!(Token::Equal,  tokens.pop_front().unwrap_or(Token::Undefined(None)));
      }

      #[test]
      pub fn lexing_expressions() {
            let mut src_file = String::from("./src/lexer/tests/expressions.c0");
            let mut lexer = Lexer::new(&mut src_file);
            let mut tokens = lex_tokens(&mut lexer.Chars);
            // println!("{:?}", tokens.len());
            assert_eq!(Token::Mult,       tokens.pop_front().unwrap_or(Token::Undefined(None)));
            assert_eq!(Token::Minus,      tokens.pop_front().unwrap_or(Token::Undefined(None)));
            assert_eq!(Token::Plus,       tokens.pop_front().unwrap_or(Token::Undefined(None)));
            assert_eq!(Token::Div,        tokens.pop_front().unwrap_or(Token::Undefined(None)));
            assert_eq!(Token::Equal,      tokens.pop_front().unwrap_or(Token::Undefined(None)));
            assert_eq!(Token::Equality,   tokens.pop_front().unwrap_or(Token::Undefined(None)));
            assert_eq!(Token::Lt,         tokens.pop_front().unwrap_or(Token::Undefined(None)));
            assert_eq!(Token::Lte,        tokens.pop_front().unwrap_or(Token::Undefined(None)));
            assert_eq!(Token::Gte,        tokens.pop_front().unwrap_or(Token::Undefined(None)));
            assert_eq!(Token::Gt,         tokens.pop_front().unwrap_or(Token::Undefined(None)));
            assert_eq!(Token::NotEq,      tokens.pop_front().unwrap_or(Token::Undefined(None)));
            assert_eq!(Token::Mod,        tokens.pop_front().unwrap_or(Token::Undefined(None)));
            assert_eq!(Token::LShift,     tokens.pop_front().unwrap_or(Token::Undefined(None)));
            assert_eq!(Token::RShift,     tokens.pop_front().unwrap_or(Token::Undefined(None)));
            assert_eq!(Token::And,        tokens.pop_front().unwrap_or(Token::Undefined(None)));
            assert_eq!(Token::Xor,        tokens.pop_front().unwrap_or(Token::Undefined(None)));
            assert_eq!(Token::Or,         tokens.pop_front().unwrap_or(Token::Undefined(None)));
            assert_eq!(Token::BitNot,     tokens.pop_front().unwrap_or(Token::Undefined(None)));
            assert_eq!(Token::LParen,     tokens.pop_front().unwrap_or(Token::Undefined(None)));
            assert_eq!(Token::RParen,     tokens.pop_front().unwrap_or(Token::Undefined(None)));
            assert_eq!(Token::PlusEq,     tokens.pop_front().unwrap_or(Token::Undefined(None)));
            assert_eq!(Token::MinusEq,    tokens.pop_front().unwrap_or(Token::Undefined(None)));
            assert_eq!(Token::DivEq,      tokens.pop_front().unwrap_or(Token::Undefined(None)));
            assert_eq!(Token::MultEq,     tokens.pop_front().unwrap_or(Token::Undefined(None)));
            assert_eq!(Token::OrEq,       tokens.pop_front().unwrap_or(Token::Undefined(None)));
            assert_eq!(Token::LShiftEq,   tokens.pop_front().unwrap_or(Token::Undefined(None)));
            assert_eq!(Token::RShiftEq,   tokens.pop_front().unwrap_or(Token::Undefined(None)));
            assert_eq!(Token::ModEq,      tokens.pop_front().unwrap_or(Token::Undefined(None)));
            assert_eq!(Token::BitNotEq,   tokens.pop_front().unwrap_or(Token::Undefined(None)));
            assert_eq!(Token::AndEq,      tokens.pop_front().unwrap_or(Token::Undefined(None)));
            assert_eq!(Token::XorEq,      tokens.pop_front().unwrap_or(Token::Undefined(None)));
            assert_eq!(Token::PlusEq,     tokens.pop_front().unwrap_or(Token::Undefined(None)));
            assert_eq!(Token::MinusEq,    tokens.pop_front().unwrap_or(Token::Undefined(None)));
            assert_eq!(Token::BooleanAnd, tokens.pop_front().unwrap_or(Token::Undefined(None)));
            assert_eq!(Token::BooleanOr,  tokens.pop_front().unwrap_or(Token::Undefined(None)));
            assert_eq!(Token::BooleanNot, tokens.pop_front().unwrap_or(Token::Undefined(None)));
      }

      #[test]
      pub fn lexing_numerics() {
            let mut src_file = String::from("./src/lexer/tests/numerics.c0");
            let mut lexer = Lexer::new(&mut src_file);
            let mut tokens = lex_tokens(&mut lexer.Chars);

            assert_eq!(Token::Num(1),         tokens.pop_front().unwrap_or(Token::Undefined(None)));
            assert_eq!(Token::Num(12),        tokens.pop_front().unwrap_or(Token::Undefined(None)));      
            assert_eq!(Token::Num(123),       tokens.pop_front().unwrap_or(Token::Undefined(None)));
            assert_eq!(Token::Num(1234),      tokens.pop_front().unwrap_or(Token::Undefined(None)));
            assert_eq!(Token::Num(12345),     tokens.pop_front().unwrap_or(Token::Undefined(None)));
            assert_eq!(Token::Num(123456),    tokens.pop_front().unwrap_or(Token::Undefined(None)));
            assert_eq!(Token::Num(1234567),   tokens.pop_front().unwrap_or(Token::Undefined(None)));
      }

      #[test]
      pub fn lexing_types() {
            let mut src_file = String::from("./src/lexer/tests/types.c0");
            let mut lexer = Lexer::new(&mut src_file);
            let mut tokens = lex_tokens(&mut lexer.Chars);

            assert_eq!(Token::Assert,     tokens.pop_front().unwrap_or(Token::Undefined(None)));
            // assert_eq!(Token::Alloc,      tokens.pop_front().unwrap_or(Token::Undefined(None)));
            // assert_eq!(Token::AllocArray, tokens.pop_front().unwrap_or(Token::Undefined(None)));
            // assert_eq!(Token::Bool,       tokens.pop_front().unwrap_or(Token::Undefined(None)));
            // assert_eq!(Token::Break,      tokens.pop_front().unwrap_or(Token::Undefined(None)));
            // assert_eq!(Token::Char,       tokens.pop_front().unwrap_or(Token::Undefined(None)));
            // assert_eq!(Token::Continue,   tokens.pop_front().unwrap_or(Token::Undefined(None)));
      }
}
