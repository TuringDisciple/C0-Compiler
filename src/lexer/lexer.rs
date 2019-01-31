use std::fs::File;
use std::io::{ BufRead, BufReader };
use std::vec;
enum Token {
      Id(String),
      Int,
      Bool,
      Char,
      Str,
      Star, 
      Struct,
      LBracket,
      RBracket,
      Equal, 
      Dot,
      And,
      Xor,
      Or,
      LShift,
      RShift,
      Div,
      Plus,
      Minus,
      SemiColon,
      Assert,
      Error,
      LCurly,
      RCurly,
      LParen,
      RParen,
      If,
      Else,
      While,
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
      Tokens: vec::Vec<Token>,
}

impl Lexer {
      pub fn new(FilePath: String) -> Lexer {
            return Lexer {
                  FilePath, 
                  Tokens: vec::Vec::new(),
            }
      }

      pub fn lex(&mut self) {
            
      }


}