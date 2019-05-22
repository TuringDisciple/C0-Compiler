use lexer::lexer::*;
use std::boxed::Box;
use std::collections::vec_deque::Iter;
use std::iter::Peekable;

#[derive(Clone, PartialEq, Debug)]
pub enum LexClass {
      Empty,
      Id    (Token, Vec<Token>),
      Num   (Box<LexClass>    ),
      DecNum(Token            ),
      HexNum(Vec<Token>       ), 
      StrLit(Vec<Token>       ),
      ChrLit(Token            ), 
      LibLit(Vec<Token>       ), 
      SChar(Vec<Token>        ), 
      CChar(Token             ),
      NChar(Token             ), 
      LChar(Token             ), 
      Esc(Token               ), 
      Sep(Token               ), 
      Unop(Token              ), 
      Binop(Token             ), 
      Asnop(Token             ), 
      Postop(Token            ),
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

// <id> ::= [A-Za-z_][A-Za-z0-9_]*
fn parse_id(tokens: &mut Peekable<Iter<'_, Token>>) -> Option<LexClass> {
      let parse_id_end = |tokens: &mut Peekable<Iter<'_, Token>>| -> Vec<Token> {
            let mut ret: Vec<Token> = Vec::new();
            loop {
                  match tokens.peek() {
                        Some(Token::Undefined(Some(' '))) => break, 
                        Some(Token::Undefined(Some(_)))
                        | Some(Token::Num(_)) => ret.push(*tokens.next().unwrap()), 
                        _ => break, 
                  }
            }
            ret 
      };

      match tokens.peek() {
            Some(Token::Undefined(Some(_))) => {
                  let t = tokens.next().unwrap();
                  let ts = parse_id_end(tokens);
                  Some(LexClass::Id(*t, ts))
            }
            _ => None
      }
}

// <num> ::= <decnum> | <hexnum>
// <decnum> ::= 0 | [1-9][0-9]*
// <hexnum> ::= 0[xX][0-9a-fA-F]+
fn parse_num(tokens: &mut Peekable<Iter<'_, Token>>) -> Option<LexClass> {
      let parse_hex = |tokens: &mut Peekable<Iter<'_, Token>>| -> LexClass {
            let mut ret: Vec<Token> = Vec::new();
            loop {
                  match tokens.peek() {
                        Some(Token::Undefined(Some('x')))
                        | Some(Token::Undefined(Some('X'))) => {tokens.next().unwrap();},
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
                              => ret.push(*tokens.next().unwrap()),
                        _     => break,
                  }
            }

            LexClass::HexNum(ret)
      };

      match tokens.peek() {
            Some(Token::Num(0)) => {
                  let t = tokens.next().unwrap();
                  match tokens.peek() {
                        Some(Token::Undefined(Some('x')))
                        |Some(Token::Undefined(Some('X'))) 
                              => Some(LexClass::Num(Box::new(parse_hex(tokens)))),
                        _     => Some(LexClass::Num(Box::new(LexClass::DecNum(*t)))),

                  }
            },
            Some(Token::Num(_)) 
                  => Some(LexClass::Num(Box::new(LexClass::DecNum(*tokens.next().unwrap())))),
            _     => None,
      }

}

fn parse_strlit(tokens: &mut Peekable<Iter<'_, Token>>) -> Option<LexClass> {
      let parse_to_end = |tokens: &mut Peekable<Iter<'_, Token>>| -> Vec<Token> {
            let mut ret = Vec::new();
            loop {
                  match tokens.next() {
                        Some(Token::DQuoteMark)
                              => break, 
                        Some(t) 
                              => ret.push(*t),
                        _     => break,
                  }
            }

            ret
      };

      match tokens.peek() {
            Some(Token::DQuoteMark) => {
                  tokens.next();
                  Some(LexClass::StrLit(parse_to_end(tokens)))
            },
            _ => None,
      }
}

fn parse_chrlit(tokens: &mut Peekable<Iter<'_, Token>>) -> Option<LexClass> {
      let ret: Option<LexClass>;
      match tokens.peek() {
            Some(Token::QuoteMark) 
                  => {
                        tokens.next();
                        ret = Some(LexClass::ChrLit(*tokens.next().unwrap()));
                        assert_eq!(Token::QuoteMark, *tokens.next().unwrap());
                  } 
            _     => {ret = None;},
      }

      ret
}

fn parse_liblit(tokens: &mut Peekable<Iter<'_, Token>>) -> Option<LexClass> {
      let parse_to_end = |tokens: &mut Peekable<Iter<'_, Token>>| -> Vec<Token> {
            let mut ret = Vec::new(); 
            loop {
                  match tokens.next() {
                        Some(Token::Gt) => break,
                        Some(t) 
                              => ret.push(*t), 
                        _     => break,
                  }
            }
            ret
      };

      match tokens.peek() {
            Some(Token::Lt) => {
                  tokens.next();
                  Some(LexClass::LibLit(parse_to_end(tokens)))
            },
            _  => None,
      }
}

fn parse_sep(tokens: &mut Peekable<Iter<'_, Token>>) -> Option<LexClass> {
      match tokens.peek() {
            Some(Token::LParen)    
            | Some(Token::RParen)     
            | Some(Token::LBracket)    
            | Some(Token::RBracket)    
            | Some(Token::LCurly)    
            | Some(Token::RCurly)     
            | Some(Token::Comma) => Some(LexClass::Sep(*tokens.next().unwrap())), 
            _ => None, 
      }
}

fn parse_unop(tokens: &mut Peekable<Iter<'_, Token>>) -> Option<LexClass> {
      match tokens.peek() {
            Some(Token::Not)      
            | Some(Token::BitNot) 
            | Some(Token::Minus)       
            | Some(Token::Mult) => Some(LexClass::Unop(*tokens.next().unwrap())),
            _ => None,
      }
}

fn parse_binop(tokens: &mut Peekable<Iter<'_, Token>>) -> Option<LexClass> {
      match tokens.peek() {
            Some(Token::FieldSelect) 
            | Some(Token::FieldDeref) 
            | Some(Token::Div)  
            | Some(Token::Mod) 
            | Some(Token::Plus)  
            | Some(Token::LShift) 
            | Some(Token::RShift) 
            | Some(Token::Lt)  
            | Some(Token::Lte)  
            | Some(Token::Gte)  
            | Some(Token::Gt)  
            | Some(Token::Equality)  
            | Some(Token::NotEq)  
            | Some(Token::And)  
            | Some(Token::Xor)  
            | Some(Token::Or)  
            | Some(Token::BooleanAnd) 
            | Some(Token::BooleanOr)  
            | Some(Token::BooleanNot)  
            | Some(Token::TernIf) 
            | Some(Token::TernNot) => Some(LexClass::Binop(*tokens.next().unwrap())), 
            _ => None, 
      }
}

fn parse_asnop(tokens: &mut Peekable<Iter<'_, Token>>) -> Option<LexClass> {
      match tokens.peek() {
            Some(Token::Equal)   
            | Some(Token::PlusEq)    
            | Some(Token::MinusEq)   
            | Some(Token::MultEq)    
            | Some(Token::DivEq)     
            | Some(Token::ModEq)     
            | Some(Token::LShiftEq)  
            | Some(Token::RShiftEq)  
            | Some(Token::AndEq)     
            | Some(Token::XorEq)     
            | Some(Token::OrEq)  => Some(LexClass::Asnop(*tokens.next().unwrap())), 
            _ => None  
      }
}

fn parse_postop(tokens: &mut Peekable<Iter<'_, Token>>) -> Option<LexClass> {
      match tokens.peek() {
            Some(Token::PostMinusEq)
            | Some(Token::PostPlusEq)  => Some(LexClass::Postop(*tokens.next().unwrap())), 
            _ => None
      }
}

#[cfg(test)]
mod test {
      use parser::parser::*;

      #[test]
      fn test_parsing_ids() {
            let mut src_file = String::from("./src/parser/tests/ids.c0");
            let mut parser = Parser::new(&mut src_file);
            let mut tokens = parser.lexer().tokens();
            let mut parse_output = parse_id(&mut tokens.iter().peekable());
            let expected_parse = Some(
                  LexClass::Id(
                        Token::Undefined(Some('A')), 
                        vec![
                              Token::Undefined(Some('a')), 
                              Token::Num(123),
                        ]
                  )
            );
            assert_eq!(parse_output, expected_parse);
      }

      #[test]
      fn test_parsing_hex() {
            let mut src_file = String::from("./src/parser/tests/hex.c0");
            let mut parser = Parser::new(&mut src_file); 
            let mut tokens = parser.lexer().tokens();
            let mut parse_output = parse_num(&mut tokens.iter().peekable());
            let expected_parse =
                  Some(LexClass::Num(
                        Box::new(
                              LexClass::HexNum(
                                    vec![
                                          Token::Num(19), 
                                          Token::Undefined(Some('a')), 
                                          Token::Undefined(Some('F'))]
                              )
                        ))
                  );

            assert_eq!(parse_output, expected_parse);
      }

      #[test]
      fn test_parsing_strlit() { 
            let mut src_file = String::from("./src/parser/tests/string.c0");
            let parser = Parser::new(&mut src_file);
            let mut tokens = parser.lexer().tokens();
            let mut parse_output = parse_strlit(&mut tokens.iter().peekable());
            let expected_parse = Some(
                  LexClass::StrLit(
                        vec![
                              Token::Undefined(Some('H')), 
                              Token::Undefined(Some('e')),
                              Token::Undefined(Some('l')),
                              Token::Undefined(Some('l')),
                              Token::Undefined(Some('o')), 
                              Token::Undefined(Some(' ')), 
                              Token::Undefined(Some('w')), 
                              Token::Undefined(Some('o')),
                              Token::Undefined(Some('r')),
                              Token::Undefined(Some('l')),
                              Token::Undefined(Some('d')), 
                        ]
                  )
            );

            assert_eq!(parse_output, expected_parse);
      }

      #[test]
      fn test_parsing_chrlit() {
            let mut src_file = String::from("./src/parser/tests/char.c0");
            let parser = Parser::new(&mut src_file);
            let mut tokens = parser.lexer().tokens();
            let mut parse_output = parse_chrlit(&mut tokens.iter().peekable());
            let expected_parse = Some(
                  LexClass::ChrLit(Token::Undefined(Some('a'))),
            );

            assert_eq!(parse_output, expected_parse);
      }
      
      #[test]
      fn test_parsing_liblit() {
            let mut src_file = String::from("./src/parser/tests/lib.c0");
            let parser = Parser::new(&mut src_file);
            let mut tokens = parser.lexer().tokens();
            let mut parse_output = parse_liblit(&mut tokens.iter().peekable()); 
            let expected_parse = Some(
                  LexClass::LibLit(
                        vec![
                              Token::Undefined(Some('s')), 
                              Token::Undefined(Some('t')), 
                              Token::Undefined(Some('d')), 
                        ]
                  )
            );

            assert_eq!(parse_output, expected_parse);
      }
}