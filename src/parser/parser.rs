use lexer::lexer::*;
use std::boxed::Box;
use std::collections::{VecDeque};
use std::collections::vec_deque::Iter;
use std::iter::Peekable;

#[derive(Clone, PartialEq, Debug)]
pub enum LexClass {
      Empty,
      Id    (Token, Vec<Token>),
      Num   (Box<LexClass>     ),
      DecNum(Token),
      HexNum(Token, Vec<Token>), 
}

#[derive(Clone)]
pub struct Parser {
      lexer: Lexer,
      parseTree: Vec<LexClass>,
}

impl Parser {
      pub fn new(filePath: &mut String) -> Parser {
            Parser {
                  lexer: Lexer::new(filePath),
                  parseTree: Vec::new(),
            }
      }

      pub fn empty() -> Parser {
            Parser{
                  lexer: Lexer::empty(), 
                  parseTree: Vec::new(),
            }
      }

      pub fn lexer(self) -> Lexer {
            self.lexer
      }
}

// Returns: Parse tree in the form of a vector of lexclasses,
pub fn parse(tokens: &mut VecDeque<Token>) -> Vec<LexClass> {
      let mut tokens_peek = tokens.iter().peekable();
      let mut parse_tree = Vec::new();
      loop {
            match tokens_peek.next(){
                  Some(Token::Undefined(Some(c))) =>
                        parse_tree.push(parse_id(Token::Undefined(Some(*c)), &mut tokens_peek)),
                  Some(Token::Num(n)) =>
                        parse_tree.push(parse_num(Token::Num(*n), &mut tokens_peek)), 

                  _ => break,
            }
      }

      parse_tree
}

// <id> ::= [A-Za-z_][A-Za-z0-9_]*
fn parse_id(t: Token, tokens: &mut Peekable<Iter<'_, Token>>) -> LexClass {
      let mut ts: Vec<Token> = Vec::new();
      loop {
            match tokens.peek() {
                  Some(Token::Undefined(Some(_))) |
                  Some(Token::Num(_)) => {
                        ts.push(*tokens.next().unwrap())
                  }
                  _ => break,
            }
      }
      LexClass::Id(t, ts)      
}

// <num> ::= <decnum> | <hexnum>
// <decnum> ::= 0 | [1-9][0-9]*
// <hexnum> ::= 0[xX][0-9a-fA-F]+
fn parse_num(t: Token, tokens: &mut Peekable<Iter<'_, Token>>) -> LexClass {
      let mut parse_hex = |tokens: &mut Peekable<Iter<'_, Token>>| -> LexClass {
            let mut ret: Vec<Token> = Vec::new();
            loop {
                  match tokens.peek() {
                        Some(Token::Num(_))
                        | Some(Token::Undefined(Some('a')))
                        | Some(Token::Undefined(Some('b')))
                        | Some(Token::Undefined(Some('c'))) 
                        | Some(Token::Undefined(Some('d')))
                        | Some(Token::Undefined(Some('e'))) 
                        | Some(Token::Undefined(Some('f')))
                        | Some(Token::Undefined(Some('A')))
                        | Some(Token::Undefined(Some('B')))
                        | Some(Token::Undefined(Some('C'))) 
                        | Some(Token::Undefined(Some('D')))
                        | Some(Token::Undefined(Some('E'))) 
                        | Some(Token::Undefined(Some('F')))
                        | Some(Token::Undefined(Some('x')))
                        | Some(Token::Undefined(Some('X')))
                              => ret.push(*tokens.next().unwrap()),
                        _     => break,
                  }
            }

            LexClass::HexNum(t, ret)
      };

      let lex_class = match t {
            Token::Num(0) => {
                  match tokens.peek() {
                        Some(Token::Undefined(Some('x')))|
                        Some(Token::Undefined(Some('X'))) 
                              => parse_hex(tokens),
                        _     => LexClass::DecNum(t), 

                  }
            }, 
            Token::Num(_) => LexClass::DecNum(t), 
            _             => LexClass::Empty,
      };

      LexClass::Num(Box::new(lex_class))
}


#[cfg(test)]
mod test {
      use lexer::lexer::*;
      use parser::parser::*;

      #[test]
      fn test_parsing_ids() {
            let mut tokens: VecDeque<Token> = VecDeque::new();
            tokens.push_back(Token::Undefined(Some('a')));
            tokens.push_back(Token::Undefined(Some('Z')));
            tokens.push_back(Token::Num(0));
            tokens.push_back(Token::Num(9));

            let parse_tree = parse(&mut tokens);
            let expected_parse_tree = vec![
                  LexClass::Id(
                        Token::Undefined(Some('a')),
                        vec![
                              Token::Undefined(Some('Z')), 
                              Token::Num(0),
                              Token::Num(9)])];

            assert_eq!(expected_parse_tree, parse_tree);
      }

      #[test]
      fn test_parsing_ids_from_file() {
            let mut src_file = String::from("./src/parser/tests/ids.c0");
            let mut parser = Parser::new(&mut src_file);
            let mut tokens = parser.lexer().tokens();
            let mut parse_tree = parse(&mut tokens);
            let expected_parse_tree = vec![
                  LexClass::Id(
                        Token::Undefined(Some('A')),
                        vec![
                              Token::Undefined(Some('a')),
                              Token::Num(123),
                        ]
                  )];
            
            assert_eq!(expected_parse_tree, parse_tree);
      }

      #[test]
      fn test_parsing_hex() {
            let mut src_file = String::from("./src/parser/tests/hex.c0");
            let mut parser = Parser::new(&mut src_file); 
            let mut tokens = parser.lexer().tokens();
            println!("{:?}", tokens);
            let mut parse_tree = parse(&mut tokens);
            let expected_parse_tree = vec![
                  LexClass::Num(
                        Box::new(
                              LexClass::HexNum(
                                    Token::Num(0),
                                    vec![
                                          Token::Undefined(Some('x')),
                                          Token::Num(19), 
                                          Token::Undefined(Some('a')), 
                                          Token::Undefined(Some('F'))]
                              )
                        ))
            ];

            assert_eq!(expected_parse_tree, parse_tree);
      }
}