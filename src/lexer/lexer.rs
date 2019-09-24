#![allow(dead_code)]
use std::fs::File;
use std::io::{ BufRead, BufReader };
use std::collections::{VecDeque, HashMap};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Token {
    // Character and operators
    Undefined(Option<char>),
    Comma, 
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
    PostPlusEq,
    Minus,
    MinusEq,
    PostMinusEq,
    Gt, 
    Lt,
    Gte,
    Lte,
    NotEq,
    LShift,
    LShiftEq,
    RShift,
    RShiftEq,
    LBracket,
    RBracket,
    LParen,
    RParen,
    BooleanAnd, 
    BooleanOr, 
    Num(u32),
    FieldSelect, 
    FieldDeref,
    TernIf,
    TernNot,
    QuoteMark,
    DQuoteMark,
    PointerDeref,
    // Types
    Int, 
    Bool, 
    Char,
    String,
    Void,
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
    SemiColon, 
    Use,
    True, 
    False,
    Null,
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

#[derive(Clone)]
pub struct Lexer {
    tokens: VecDeque<Token>,
}

// TODO: Grammar extensions for annotations
impl Lexer {
    pub fn new(mut file_path: &mut String) -> Lexer {
        let tokens = lex_tokens(&mut chars(&mut file_path));
        Lexer {
            tokens,
        }
    }

    // Unintialised lexer object returned, for testing
    pub fn empty() -> Lexer {
        Lexer{
            tokens: VecDeque::new(),
        }
    }

    pub fn tokens(self) -> VecDeque<Token> {
        self.tokens
    }
}

fn lex_tokens(chars: &mut VecDeque<char>) -> VecDeque<Token> {
    let mut tokens: VecDeque<Token> = VecDeque::new();
    loop {
        println!("{:?}", chars);
        match chars.pop_front() {

            Some(c) => {
                match c {
                    ';' => tokens.push_back(Token::SemiColon),
                    '(' => tokens.push_back(Token::LParen),
                    ')' => tokens.push_back(Token::RParen),
                    '~' => tokens.push_back(ops(Token::BitNot,   chars)), 
                    '=' => tokens.push_back(ops(Token::Equal,    chars)),
                    '!' => tokens.push_back(ops(Token::Not  ,    chars)),
                    '+' => tokens.push_back(ops(Token::Plus ,    chars)),
                    '-' => tokens.push_back(ops(Token::Minus,    chars)),
                    '&' => tokens.push_back(ops(Token::And  ,    chars)),
                    '%' => tokens.push_back(ops(Token::Mod  ,    chars)),
                    '/' => tokens.push_back(ops(Token::Div  ,    chars)),
                    '*' => tokens.push_back(ops(Token::Mult ,    chars)), 
                    '<' => tokens.push_back(ops(Token::Lt   ,    chars)), 
                    '>' => tokens.push_back(ops(Token::Gt   ,    chars)),
                    '^' => tokens.push_back(ops(Token::Xor  ,    chars)), 
                    '|' => tokens.push_back(ops(Token::Or   ,    chars)), 
                    '[' => tokens.push_back(Token::LBracket), 
                    ']' => tokens.push_back(Token::RBracket),
                    '{' => tokens.push_back(Token::LCurly), 
                    '}' => tokens.push_back(Token::RCurly),
                    ',' => tokens.push_back(Token::Comma),
                    '.' => tokens.push_back(Token::FieldSelect),
                    '?' => tokens.push_back(Token::TernIf), 
                    ':' => tokens.push_back(Token::TernNot),
                    '"' => tokens.push_back(Token::DQuoteMark),
                    '\''=> tokens.push_back(Token::QuoteMark),
                    '0'|'1'|'2'|
                    '3'|'4'|'5'|
                    '6'|'7'|'8'|
                    '9' => tokens.push_back(numeric(c, chars)),

                    'i' | 'b' | 'c' | 
                    's' | 'v' | 'a' |
                    'e' | 'r' | 'w' |
                    'f' | '_' | 't' |
                    '#' | 'N' => tokens.push_back(keyword(c, chars)),
                     
                    // ' ' => continue,
                    _   => tokens.push_back(Token::Undefined(Some(c))),
                }
            }
            _ => break,
        }
    }
    tokens
}

fn chars(file_path: &mut String) -> VecDeque<char>{
    let mut chars: VecDeque<char> = VecDeque::new();
    for line in open_file(file_path).lines() {
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
                '+' => Token::PostPlusEq, 
                '=' => Token::PlusEq,
                _   => re_insert(tail, tail2, chars, head),
            }
        }
        Token::Minus => {
            match tail {
                '-' => Token::PostMinusEq,
                '=' => Token::MinusEq,
                '>' => Token::FieldDeref,
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
    let mut sum: u32 = head.to_digit(10).unwrap_or(0);
    loop {
        match chars.pop_front() {
            Some(c) => {
                match c {
                    '0'|'1'|'2'|'3'|'4'|'5'|'6'|'7'|'8'|'9' =>{
                        if sum <= (4294967295/10){
                            sum = 10 * sum + c.to_digit(10).unwrap_or(0);
                        };
                    },
                    _        => {
                        chars.push_front(c);
                        return Token::Num(sum);
                    }
                }
            },
            _   =>
                return Token::Num(sum),
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
    
    let next_n: Vec<char> = copy_n(p.len(), chars);

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
                return Some(t.clone());
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
            patterns.insert(Token::False, vec!['a','l','s','e']);
            check_patterns(&patterns, chars)
        }

        'i' => {
            patterns.insert(Token::If, vec!['f']);
            patterns.insert(Token::Int, vec!['n', 't']);
            check_patterns(&patterns, chars)

        }

        'N' => {
            patterns.insert(Token::Null, vec!['U', 'L', 'L']);
            check_patterns(&patterns, chars)
        }

        's' => {
            patterns.insert(Token::String, vec!['t', 'r', 'i', 'n', 'g']);
            patterns.insert(Token::Struct, vec!['t', 'r', 'u', 'c', 't']);
            check_patterns(&patterns, chars)
        }

        't' => {
            patterns.insert(Token::Typedef, vec!['y', 'p', 'e', 'd', 'e', 'f']);
            patterns.insert(Token::True, vec!['r', 'u', 'e']);
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

    fn next_non_space(tokens:&mut VecDeque<Token>) -> Option<Token> {
        loop {
            match tokens.pop_front() {
                Some(Token::Undefined(Some(' '))) => continue, 
                Some(t) => return Some(t), 
                None    => return None,
            }
        }
    }
    #[test]
    pub fn lexing_simple_expressions() {
        let mut src_file = String::from("./src/lexer/tests/simple_expressions.c0");
        let mut lexer = Lexer::new(&mut src_file);
        assert_eq!(Token::Plus,   next_non_space(&mut lexer.tokens).unwrap_or(Token::Undefined(None)));
        assert_eq!(Token::Minus,  next_non_space(&mut lexer.tokens).unwrap_or(Token::Undefined(None)));
        assert_eq!(Token::Mult,   next_non_space(&mut lexer.tokens).unwrap_or(Token::Undefined(None)));
        assert_eq!(Token::Lt,     next_non_space(&mut lexer.tokens).unwrap_or(Token::Undefined(None)));
        assert_eq!(Token::Gt,     next_non_space(&mut lexer.tokens).unwrap_or(Token::Undefined(None)));
        assert_eq!(Token::Or,     next_non_space(&mut lexer.tokens).unwrap_or(Token::Undefined(None)));
        assert_eq!(Token::Xor,    next_non_space(&mut lexer.tokens).unwrap_or(Token::Undefined(None)));
        assert_eq!(Token::BitNot, next_non_space(&mut lexer.tokens).unwrap_or(Token::Undefined(None)));
        assert_eq!(Token::Div,    next_non_space(&mut lexer.tokens).unwrap_or(Token::Undefined(None)));
        assert_eq!(Token::Equal,  next_non_space(&mut lexer.tokens).unwrap_or(Token::Undefined(None)));
    }

    #[test]
    pub fn lexing_expressions() {
        let mut src_file = String::from("./src/lexer/tests/expressions.c0");
        let mut lexer = Lexer::new(&mut src_file);
        assert_eq!(Token::Mult,       next_non_space(&mut lexer.tokens).unwrap_or(Token::Undefined(None)));
        assert_eq!(Token::Minus,      next_non_space(&mut lexer.tokens).unwrap_or(Token::Undefined(None)));
        assert_eq!(Token::Plus,       next_non_space(&mut lexer.tokens).unwrap_or(Token::Undefined(None)));
        assert_eq!(Token::Div,        next_non_space(&mut lexer.tokens).unwrap_or(Token::Undefined(None)));
        assert_eq!(Token::Equal,      next_non_space(&mut lexer.tokens).unwrap_or(Token::Undefined(None)));
        assert_eq!(Token::Equality,   next_non_space(&mut lexer.tokens).unwrap_or(Token::Undefined(None)));
        assert_eq!(Token::Lt,         next_non_space(&mut lexer.tokens).unwrap_or(Token::Undefined(None)));
        assert_eq!(Token::Lte,        next_non_space(&mut lexer.tokens).unwrap_or(Token::Undefined(None)));
        assert_eq!(Token::Gte,        next_non_space(&mut lexer.tokens).unwrap_or(Token::Undefined(None)));
        assert_eq!(Token::Gt,         next_non_space(&mut lexer.tokens).unwrap_or(Token::Undefined(None)));
        assert_eq!(Token::NotEq,      next_non_space(&mut lexer.tokens).unwrap_or(Token::Undefined(None)));
        assert_eq!(Token::Mod,        next_non_space(&mut lexer.tokens).unwrap_or(Token::Undefined(None)));
        assert_eq!(Token::LShift,     next_non_space(&mut lexer.tokens).unwrap_or(Token::Undefined(None)));
        assert_eq!(Token::RShift,     next_non_space(&mut lexer.tokens).unwrap_or(Token::Undefined(None)));
        assert_eq!(Token::And,        next_non_space(&mut lexer.tokens).unwrap_or(Token::Undefined(None)));
        assert_eq!(Token::Xor,        next_non_space(&mut lexer.tokens).unwrap_or(Token::Undefined(None)));
        assert_eq!(Token::Or,         next_non_space(&mut lexer.tokens).unwrap_or(Token::Undefined(None)));
        assert_eq!(Token::BitNot,     next_non_space(&mut lexer.tokens).unwrap_or(Token::Undefined(None)));
        assert_eq!(Token::LParen,     next_non_space(&mut lexer.tokens).unwrap_or(Token::Undefined(None)));
        assert_eq!(Token::RParen,     next_non_space(&mut lexer.tokens).unwrap_or(Token::Undefined(None)));
        assert_eq!(Token::PlusEq,     next_non_space(&mut lexer.tokens).unwrap_or(Token::Undefined(None)));
        assert_eq!(Token::MinusEq,    next_non_space(&mut lexer.tokens).unwrap_or(Token::Undefined(None)));
        assert_eq!(Token::DivEq,      next_non_space(&mut lexer.tokens).unwrap_or(Token::Undefined(None)));
        assert_eq!(Token::MultEq,     next_non_space(&mut lexer.tokens).unwrap_or(Token::Undefined(None)));
        assert_eq!(Token::OrEq,       next_non_space(&mut lexer.tokens).unwrap_or(Token::Undefined(None)));
        assert_eq!(Token::LShiftEq,   next_non_space(&mut lexer.tokens).unwrap_or(Token::Undefined(None)));
        assert_eq!(Token::RShiftEq,   next_non_space(&mut lexer.tokens).unwrap_or(Token::Undefined(None)));
        assert_eq!(Token::ModEq,      next_non_space(&mut lexer.tokens).unwrap_or(Token::Undefined(None)));
        assert_eq!(Token::BitNotEq,   next_non_space(&mut lexer.tokens).unwrap_or(Token::Undefined(None)));
        assert_eq!(Token::AndEq,      next_non_space(&mut lexer.tokens).unwrap_or(Token::Undefined(None)));
        assert_eq!(Token::XorEq,      next_non_space(&mut lexer.tokens).unwrap_or(Token::Undefined(None)));
        assert_eq!(Token::PostPlusEq, next_non_space(&mut lexer.tokens).unwrap_or(Token::Undefined(None)));
        assert_eq!(Token::PostMinusEq,next_non_space(&mut lexer.tokens).unwrap_or(Token::Undefined(None)));
        assert_eq!(Token::BooleanAnd, next_non_space(&mut lexer.tokens).unwrap_or(Token::Undefined(None)));
        assert_eq!(Token::BooleanOr,  next_non_space(&mut lexer.tokens).unwrap_or(Token::Undefined(None)));
        assert_eq!(Token::BooleanNot, next_non_space(&mut lexer.tokens).unwrap_or(Token::Undefined(None)));
    }

    #[test]
    pub fn lexing_numerics() {
        let mut src_file = String::from("./src/lexer/tests/numerics.c0");
        let mut lexer = Lexer::new(&mut src_file);
        assert_eq!(Token::Num(1),         next_non_space(&mut lexer.tokens).unwrap_or(Token::Undefined(None)));
        assert_eq!(Token::Num(12),        next_non_space(&mut lexer.tokens).unwrap_or(Token::Undefined(None)));      
        assert_eq!(Token::Num(123),       next_non_space(&mut lexer.tokens).unwrap_or(Token::Undefined(None)));
        assert_eq!(Token::Num(1234),      next_non_space(&mut lexer.tokens).unwrap_or(Token::Undefined(None)));
        assert_eq!(Token::Num(12345),     next_non_space(&mut lexer.tokens).unwrap_or(Token::Undefined(None)));
        assert_eq!(Token::Num(123456),    next_non_space(&mut lexer.tokens).unwrap_or(Token::Undefined(None)));
        assert_eq!(Token::Num(1234567),   next_non_space(&mut lexer.tokens).unwrap_or(Token::Undefined(None)));
    }

    #[test]
    pub fn lexing_types() {
        let mut src_file = String::from("./src/lexer/tests/types.c0");
        let mut lexer = Lexer::new(&mut src_file);
        assert_eq!(Token::Assert,     next_non_space(&mut lexer.tokens).unwrap_or(Token::Undefined(None)));
        assert_eq!(Token::Alloc,      next_non_space(&mut lexer.tokens).unwrap_or(Token::Undefined(None)));
        assert_eq!(Token::Alloc,      next_non_space(&mut lexer.tokens).unwrap_or(Token::Undefined(None)));
        assert_eq!(Token::AllocArray, next_non_space(&mut lexer.tokens).unwrap_or(Token::Undefined(None)));
        assert_eq!(Token::Bool,       next_non_space(&mut lexer.tokens).unwrap_or(Token::Undefined(None)));
        assert_eq!(Token::Break,      next_non_space(&mut lexer.tokens).unwrap_or(Token::Undefined(None)));
        assert_eq!(Token::Char,       next_non_space(&mut lexer.tokens).unwrap_or(Token::Undefined(None)));
        assert_eq!(Token::Continue,   next_non_space(&mut lexer.tokens).unwrap_or(Token::Undefined(None)));
        assert_eq!(Token::Error,      next_non_space(&mut lexer.tokens).unwrap_or(Token::Undefined(None)));
        assert_eq!(Token::For,        next_non_space(&mut lexer.tokens).unwrap_or(Token::Undefined(None)));
        assert_eq!(Token::If,         next_non_space(&mut lexer.tokens).unwrap_or(Token::Undefined(None)));
        assert_eq!(Token::Int,        next_non_space(&mut lexer.tokens).unwrap_or(Token::Undefined(None)));
        assert_eq!(Token::String,     next_non_space(&mut lexer.tokens).unwrap_or(Token::Undefined(None)));
        assert_eq!(Token::Struct,     next_non_space(&mut lexer.tokens).unwrap_or(Token::Undefined(None)));
        assert_eq!(Token::Typedef,    next_non_space(&mut lexer.tokens).unwrap_or(Token::Undefined(None)));
        assert_eq!(Token::Return,     next_non_space(&mut lexer.tokens).unwrap_or(Token::Undefined(None)));
        assert_eq!(Token::While,      next_non_space(&mut lexer.tokens).unwrap_or(Token::Undefined(None)));
        assert_eq!(Token::Void,       next_non_space(&mut lexer.tokens).unwrap_or(Token::Undefined(None)));
        assert_eq!(Token::Use,        next_non_space(&mut lexer.tokens).unwrap_or(Token::Undefined(None)));
        assert_eq!(Token::SemiColon,  next_non_space(&mut lexer.tokens).unwrap_or(Token::Undefined(None)));
    }

    #[test]
    pub fn lexing_string() {

    }
}
