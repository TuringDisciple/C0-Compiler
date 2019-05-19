use std::fs::File;
use std::io::{ BufRead, BufReader };
use std::collections::{VecDeque, HashMap};

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
      String, 
      Void,
      Collection,
      Struct,
      // TODO: how to represent structs
      // TODO: How to represent collection of arguments for function types 
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
      Use,

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
      tokens: VecDeque<Token>,
}

// TODO: Grammar extensions for annotations
impl Lexer {
      pub fn new(mut FilePath: &mut String) -> Lexer {
            let tokens = lex_tokens(&mut chars(&mut FilePath));
            Lexer {
                  tokens,
            }
      }

      pub fn tokens(self) -> VecDeque<Token> {
            self.tokens
      }
}

fn lex_tokens(Chars: &mut VecDeque<char>) -> VecDeque<Token> {
      let mut tokens: VecDeque<Token> = VecDeque::new();
      loop {
            // println!("{:?}", Chars);
            match Chars.pop_front() {

                  Some(c) => {
                        match c {
                              ';' => tokens.push_back(Token::SemiColon),
                              '(' => tokens.push_back(Token::LParen),
                              ')' => tokens.push_back(Token::RParen),
                              '~' => tokens.push_back(ops(Token::BitNot,   Chars)), 
                              '=' => tokens.push_back(ops(Token::Equal,    Chars)),
                              '!' => tokens.push_back(ops(Token::Not  ,    Chars)),
                              '+' => tokens.push_back(ops(Token::Plus ,    Chars)),
                              '-' => tokens.push_back(ops(Token::Minus,    Chars)),
                              '&' => tokens.push_back(ops(Token::And  ,    Chars)),
                              '%' => tokens.push_back(ops(Token::Mod  ,    Chars)),
                              '/' => tokens.push_back(ops(Token::Div  ,    Chars)),
                              '*' => tokens.push_back(ops(Token::Mult ,    Chars)), 
                              '<' => tokens.push_back(ops(Token::Lt   ,    Chars)), 
                              '>' => tokens.push_back(ops(Token::Gt   ,    Chars)),
                              '^' => tokens.push_back(ops(Token::Xor  ,    Chars)), 
                              '|' => tokens.push_back(ops(Token::Or   ,    Chars)), 
                              '0'|'1'|'2'|
                              '3'|'4'|'5'|
                              '6'|'7'|'8'|
                              '9' => tokens.push_back(numeric(c, Chars)),

                              'i' | 'b' | 'c' | 
                              's' | 'v' | 'a' |
                              'e' | 'r' | 'w' |
                              'f' | '_' | 't' |
                              '#' => tokens.push_back(keyword(c, Chars)),
                              
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

fn ops(head: Token, chars: &mut VecDeque<char>) -> Token {
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

fn copy_n(n: usize, vec: &mut VecDeque<char>) -> Vec<char>{
      let mut ret: Vec<char> = Vec::new();

      for i in 0..n {
            match vec.get(i) {
                  Some(c) => ret.push(*c),
                  None    => return ret,
            }
      }

      ret
}

fn pattern_check(p: Vec<char>, chars: &mut VecDeque<char>) -> Option<Token> {
      
      let mut next_n: Vec<char> = copy_n(p.len(), chars);

      if next_n == p {
            return Some(Token::Undefined(None));
      }

      None
}

fn match_keyword(patterns: &HashMap<Token, Vec<char>>, chars: &mut VecDeque<char>) -> Option<Token>{

      for (t, p) in patterns.iter() {
            match pattern_check(p.to_vec(), chars) {
                  Some(_) => {
                        chars.drain(0..p.len());
                        return Some(*t);
                  }
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
                  check_patterns(&patterns, chars)
            }

            'b' => {
                  patterns.insert(Token::Bool, vec!['o', 'o', 'l']);
                  patterns.insert(Token::Break, vec!['r', 'e', 'a', 'k']);
                  check_patterns(&patterns, chars)
            }

            'c' => {
                  patterns.insert(Token::Char, vec!['h', 'a', 'r']);
                  patterns.insert(Token::Continue, vec!['o', 'n', 't', 'i', 'n', 'u', 'e']);
                  check_patterns(&patterns, chars)
            }

            'e' => {
                  patterns.insert(Token::Error, vec!['r', 'r', 'o', 'r']);
                  check_patterns(&patterns, chars)
            }

            'f' => {
                  patterns.insert(Token::For, vec!['o', 'r']);
                  check_patterns(&patterns, chars)
            }

            'i' => {
                  patterns.insert(Token::If, vec!['f']);
                  patterns.insert(Token::Int, vec!['n', 't']);
                  check_patterns(&patterns, chars)

            }

            's' => {
                  patterns.insert(Token::String, vec!['t', 'r', 'i', 'n', 'g']);
                  patterns.insert(Token::Struct, vec!['t', 'r', 'u', 'c', 't']);
                  check_patterns(&patterns, chars)
            }

            't' => {
                  patterns.insert(Token::Typedef, vec!['y', 'p', 'e', 'd', 'e', 'f']);
                  check_patterns(&patterns, chars)
            }

            'r' => {
                  patterns.insert(Token::Return, vec!['e', 't', 'u', 'r', 'n']);
                  check_patterns(&patterns, chars) 
            }

            'w' => {
                  patterns.insert(Token::While, vec!['h', 'i', 'l', 'e']);
                  check_patterns(&patterns, chars)
            }

            'v' => {
                  patterns.insert(Token::Void, vec!['o', 'i', 'd']);
                  check_patterns(&patterns, chars)
            }

            '_' => {
                  patterns.insert(Token::AllocArray, vec!['a', 'r', 'r', 'a', 'y']);
                  check_patterns(&patterns, chars)
            }

            '#' => {
                  patterns.insert(Token::Use, vec!['u', 's', 'e']);
                  check_patterns(&patterns, chars)
            }

            _   => Token::Undefined(Some(head)), 
      }
}


#[cfg(test)]
mod test {
      use lexer::lexer::*;
      #[test]
      pub fn lexing_simple_expressions() {
            let mut src_file = String::from("./src/lexer/tests/simple_expressions.c0");
            let mut lexer = Lexer::new(&mut src_file);
            assert_eq!(Token::Plus,   lexer.tokens.pop_front().unwrap_or(Token::Undefined(None)));
            assert_eq!(Token::Minus,  lexer.tokens.pop_front().unwrap_or(Token::Undefined(None)));
            assert_eq!(Token::Mult,   lexer.tokens.pop_front().unwrap_or(Token::Undefined(None)));
            assert_eq!(Token::Lt,     lexer.tokens.pop_front().unwrap_or(Token::Undefined(None)));
            assert_eq!(Token::Gt,     lexer.tokens.pop_front().unwrap_or(Token::Undefined(None)));
            assert_eq!(Token::Or,     lexer.tokens.pop_front().unwrap_or(Token::Undefined(None)));
            assert_eq!(Token::Xor,    lexer.tokens.pop_front().unwrap_or(Token::Undefined(None)));
            assert_eq!(Token::BitNot, lexer.tokens.pop_front().unwrap_or(Token::Undefined(None)));
            assert_eq!(Token::Div,    lexer.tokens.pop_front().unwrap_or(Token::Undefined(None)));
            assert_eq!(Token::Equal,  lexer.tokens.pop_front().unwrap_or(Token::Undefined(None)));
      }

      #[test]
      pub fn lexing_expressions() {
            let mut src_file = String::from("./src/lexer/tests/expressions.c0");
            let mut lexer = Lexer::new(&mut src_file);
            assert_eq!(Token::Mult,       lexer.tokens.pop_front().unwrap_or(Token::Undefined(None)));
            assert_eq!(Token::Minus,      lexer.tokens.pop_front().unwrap_or(Token::Undefined(None)));
            assert_eq!(Token::Plus,       lexer.tokens.pop_front().unwrap_or(Token::Undefined(None)));
            assert_eq!(Token::Div,        lexer.tokens.pop_front().unwrap_or(Token::Undefined(None)));
            assert_eq!(Token::Equal,      lexer.tokens.pop_front().unwrap_or(Token::Undefined(None)));
            assert_eq!(Token::Equality,   lexer.tokens.pop_front().unwrap_or(Token::Undefined(None)));
            assert_eq!(Token::Lt,         lexer.tokens.pop_front().unwrap_or(Token::Undefined(None)));
            assert_eq!(Token::Lte,        lexer.tokens.pop_front().unwrap_or(Token::Undefined(None)));
            assert_eq!(Token::Gte,        lexer.tokens.pop_front().unwrap_or(Token::Undefined(None)));
            assert_eq!(Token::Gt,         lexer.tokens.pop_front().unwrap_or(Token::Undefined(None)));
            assert_eq!(Token::NotEq,      lexer.tokens.pop_front().unwrap_or(Token::Undefined(None)));
            assert_eq!(Token::Mod,        lexer.tokens.pop_front().unwrap_or(Token::Undefined(None)));
            assert_eq!(Token::LShift,     lexer.tokens.pop_front().unwrap_or(Token::Undefined(None)));
            assert_eq!(Token::RShift,     lexer.tokens.pop_front().unwrap_or(Token::Undefined(None)));
            assert_eq!(Token::And,        lexer.tokens.pop_front().unwrap_or(Token::Undefined(None)));
            assert_eq!(Token::Xor,        lexer.tokens.pop_front().unwrap_or(Token::Undefined(None)));
            assert_eq!(Token::Or,         lexer.tokens.pop_front().unwrap_or(Token::Undefined(None)));
            assert_eq!(Token::BitNot,     lexer.tokens.pop_front().unwrap_or(Token::Undefined(None)));
            assert_eq!(Token::LParen,     lexer.tokens.pop_front().unwrap_or(Token::Undefined(None)));
            assert_eq!(Token::RParen,     lexer.tokens.pop_front().unwrap_or(Token::Undefined(None)));
            assert_eq!(Token::PlusEq,     lexer.tokens.pop_front().unwrap_or(Token::Undefined(None)));
            assert_eq!(Token::MinusEq,    lexer.tokens.pop_front().unwrap_or(Token::Undefined(None)));
            assert_eq!(Token::DivEq,      lexer.tokens.pop_front().unwrap_or(Token::Undefined(None)));
            assert_eq!(Token::MultEq,     lexer.tokens.pop_front().unwrap_or(Token::Undefined(None)));
            assert_eq!(Token::OrEq,       lexer.tokens.pop_front().unwrap_or(Token::Undefined(None)));
            assert_eq!(Token::LShiftEq,   lexer.tokens.pop_front().unwrap_or(Token::Undefined(None)));
            assert_eq!(Token::RShiftEq,   lexer.tokens.pop_front().unwrap_or(Token::Undefined(None)));
            assert_eq!(Token::ModEq,      lexer.tokens.pop_front().unwrap_or(Token::Undefined(None)));
            assert_eq!(Token::BitNotEq,   lexer.tokens.pop_front().unwrap_or(Token::Undefined(None)));
            assert_eq!(Token::AndEq,      lexer.tokens.pop_front().unwrap_or(Token::Undefined(None)));
            assert_eq!(Token::XorEq,      lexer.tokens.pop_front().unwrap_or(Token::Undefined(None)));
            assert_eq!(Token::PlusEq,     lexer.tokens.pop_front().unwrap_or(Token::Undefined(None)));
            assert_eq!(Token::MinusEq,    lexer.tokens.pop_front().unwrap_or(Token::Undefined(None)));
            assert_eq!(Token::BooleanAnd, lexer.tokens.pop_front().unwrap_or(Token::Undefined(None)));
            assert_eq!(Token::BooleanOr,  lexer.tokens.pop_front().unwrap_or(Token::Undefined(None)));
            assert_eq!(Token::BooleanNot, lexer.tokens.pop_front().unwrap_or(Token::Undefined(None)));
      }

      #[test]
      pub fn lexing_numerics() {
            let mut src_file = String::from("./src/lexer/tests/numerics.c0");
            let mut lexer = Lexer::new(&mut src_file);
            assert_eq!(Token::Num(1),         lexer.tokens.pop_front().unwrap_or(Token::Undefined(None)));
            assert_eq!(Token::Num(12),        lexer.tokens.pop_front().unwrap_or(Token::Undefined(None)));      
            assert_eq!(Token::Num(123),       lexer.tokens.pop_front().unwrap_or(Token::Undefined(None)));
            assert_eq!(Token::Num(1234),      lexer.tokens.pop_front().unwrap_or(Token::Undefined(None)));
            assert_eq!(Token::Num(12345),     lexer.tokens.pop_front().unwrap_or(Token::Undefined(None)));
            assert_eq!(Token::Num(123456),    lexer.tokens.pop_front().unwrap_or(Token::Undefined(None)));
            assert_eq!(Token::Num(1234567),   lexer.tokens.pop_front().unwrap_or(Token::Undefined(None)));
      }

      #[test]
      pub fn lexing_types() {
            let mut src_file = String::from("./src/lexer/tests/types.c0");
            let mut lexer = Lexer::new(&mut src_file);
            assert_eq!(Token::Assert,     lexer.tokens.pop_front().unwrap_or(Token::Undefined(None)));
            assert_eq!(Token::Alloc,      lexer.tokens.pop_front().unwrap_or(Token::Undefined(None)));
            assert_eq!(Token::Alloc,      lexer.tokens.pop_front().unwrap_or(Token::Undefined(None)));
            assert_eq!(Token::AllocArray, lexer.tokens.pop_front().unwrap_or(Token::Undefined(None)));
            assert_eq!(Token::Bool,       lexer.tokens.pop_front().unwrap_or(Token::Undefined(None)));
            assert_eq!(Token::Break,      lexer.tokens.pop_front().unwrap_or(Token::Undefined(None)));
            assert_eq!(Token::Char,       lexer.tokens.pop_front().unwrap_or(Token::Undefined(None)));
            assert_eq!(Token::Continue,   lexer.tokens.pop_front().unwrap_or(Token::Undefined(None)));
            assert_eq!(Token::Error,      lexer.tokens.pop_front().unwrap_or(Token::Undefined(None)));
            assert_eq!(Token::For,        lexer.tokens.pop_front().unwrap_or(Token::Undefined(None)));
            assert_eq!(Token::If,         lexer.tokens.pop_front().unwrap_or(Token::Undefined(None)));
            assert_eq!(Token::Int,        lexer.tokens.pop_front().unwrap_or(Token::Undefined(None)));
            assert_eq!(Token::String,     lexer.tokens.pop_front().unwrap_or(Token::Undefined(None)));
            assert_eq!(Token::Struct,     lexer.tokens.pop_front().unwrap_or(Token::Undefined(None)));
            assert_eq!(Token::Typedef,    lexer.tokens.pop_front().unwrap_or(Token::Undefined(None)));
            assert_eq!(Token::Return,     lexer.tokens.pop_front().unwrap_or(Token::Undefined(None)));
            assert_eq!(Token::While,      lexer.tokens.pop_front().unwrap_or(Token::Undefined(None)));
            assert_eq!(Token::Void,       lexer.tokens.pop_front().unwrap_or(Token::Undefined(None)));
            assert_eq!(Token::Use,        lexer.tokens.pop_front().unwrap_or(Token::Undefined(None)));
            assert_eq!(Token::SemiColon,  lexer.tokens.pop_front().unwrap_or(Token::Undefined(None)));
      }
}
