use lexer::lexer::*;
use std::boxed::Box;
use std::collections::vec_deque::Iter;
use std::iter::Peekable;
use either::Either;

type OptionLexClass = Option<LexClass>;

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
      Tp(Vec<Either<Box<LexClass>, Token>>),
}

struct Pair<T> {
      x: T,
      y: T,
}

pub trait Alternative {
      fn empty() -> Self;
      fn add(Self, Self) -> Self;
}

impl Alternative for OptionLexClass{
      fn empty() -> OptionLexClass{
            None
      }

      fn add(x: OptionLexClass, y: OptionLexClass) -> OptionLexClass{
            let pair: Pair<OptionLexClass> = Pair{ x, y };
            match pair{
                  Pair{ x: Some(lc1), y: Some(lc2) } => Some(LexClass::add(lc1, lc2)),
                  Pair{ x: None,      y: Some(lc)  } => Some(lc), 
                  Pair{ x: Some(lc),  y: None      } => Some(lc),
                  Pair{ x: None,      y: None      } => None, 
                  _ => None,

            }
      }
}

impl Alternative for LexClass {
      fn empty() -> LexClass {
            LexClass::Empty
      }

      fn add(x: LexClass, y: LexClass) -> LexClass {
            let pair: Pair<LexClass> = Pair{ x, y };
            match pair {
                  Pair{ x: LexClass::Empty, y: LexClass::Empty }     => LexClass::Empty, 
                  Pair{ x: LexClass::Tp(op1), y: LexClass::Tp(op2) } => {
                        let mut buff = Vec::new();
                        buff.extend(op1);
                        buff.extend(op2);
                        LexClass::Tp(buff)
                  },
                  Pair{ x: LexClass::Tp(op1), y: LexClass::Id(t, ts) } => {
                        let mut buff = Vec::new();
                        buff.extend(op1);
                        let wrapper = vec![Either::Left(Box::new(LexClass::Id(t, ts)))];
                        buff.extend(wrapper);
                        LexClass::Tp(buff)
                  }
                  _ => LexClass::Empty
            }
      }
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

fn peek_non_whitespace(tokens: &mut Peekable<Iter<'_, Token>>) -> Option<Token> {
      loop {
            match tokens.peek() {
                  Some(Token::Undefined(Some(' '))) => (),
                  Some(t) => return Some(**t), 
                  None    => return None,
            }
            tokens.next();
      }
}

// <id> ::= [A-Za-z_][A-Za-z0-9_]*
fn parse_id(tokens: &mut Peekable<Iter<'_, Token>>) -> OptionLexClass{
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

      match peek_non_whitespace(tokens){
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
fn parse_num(tokens: &mut Peekable<Iter<'_, Token>>) -> OptionLexClass{
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

fn parse_strlit(tokens: &mut Peekable<Iter<'_, Token>>) -> OptionLexClass{
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

fn parse_chrlit(tokens: &mut Peekable<Iter<'_, Token>>) -> OptionLexClass{
      let ret: OptionLexClass;
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

fn parse_liblit(tokens: &mut Peekable<Iter<'_, Token>>) -> OptionLexClass{
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

fn parse_sep(tokens: &mut Peekable<Iter<'_, Token>>) -> OptionLexClass{
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

fn parse_unop(tokens: &mut Peekable<Iter<'_, Token>>) -> OptionLexClass{
      match tokens.peek() {
            Some(Token::Not)      
            | Some(Token::BitNot) 
            | Some(Token::Minus)       
            | Some(Token::Mult) => Some(LexClass::Unop(*tokens.next().unwrap())),
            _ => None,
      }
}

fn parse_binop(tokens: &mut Peekable<Iter<'_, Token>>) -> OptionLexClass{
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

fn parse_asnop(tokens: &mut Peekable<Iter<'_, Token>>) -> OptionLexClass{
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

fn parse_postop(tokens: &mut Peekable<Iter<'_, Token>>) -> OptionLexClass{
      match tokens.peek() {
            Some(Token::PostMinusEq)
            | Some(Token::PostPlusEq)  => Some(LexClass::Postop(*tokens.next().unwrap())), 
            _ => None
      }
}


// TODO: Recursing on types
fn parse_tp(tokens: &mut Peekable<Iter<'_, Token>>) -> OptionLexClass{

      let look_ahead = | tokens: &mut Peekable<Iter<'_, Token>>, prev: OptionLexClass| -> OptionLexClass{
            match peek_non_whitespace(tokens) {
                  Some(Token::Mult) => {
                        tokens.next();
                        let parse = Some(LexClass::Tp(vec![Either::Right(Token::Mult)]));
                        return OptionLexClass::add(prev, parse)
                  },
                  Some(Token::LBracket) => {
                        let bracket1 = *tokens.next().unwrap();
                        let bracket2 = *tokens.next().unwrap();
                        assert_eq!(bracket2, Token::RBracket);
                        let parse = Some(LexClass::Tp(vec![Either::Right(bracket1), Either::Right(bracket2)]));
                        OptionLexClass::add(prev, parse)
                  }

                  _ => prev,
            }
      };

      match peek_non_whitespace(tokens) {
            Some(Token::Int) 
            | Some(Token::Bool)
            | Some(Token::String)
            | Some(Token::Char)
            | Some(Token::Void) => {
                  let t = *tokens.next().unwrap();
                  let p = Some(LexClass::Tp( vec![ Either::Right( t ) ] ));
                  look_ahead(tokens, p)
            },
            
            Some(Token::Struct) => {
                  let t = *tokens.next().unwrap();
                  let p = LexClass::Tp(vec![ Either::Right( t ) ]);
                  let sid = parse_sid(tokens);
                  match sid {
                        Some(lc) => OptionLexClass::add(Some(p), Some(lc)),//Some(LexClass::add(p, lc)),
                        None     => None, 
                  }
            }
            Some(Token::Undefined(Some(_))) => OptionLexClass::add(Some(LexClass::Tp(vec![])), parse_id(tokens)),
            _ => None,
      }
}

fn parse_sid(tokens: &mut Peekable<Iter<'_, Token>>) -> OptionLexClass{
      parse_id(tokens)
}

fn parse_aid(tokens: &mut Peekable<Iter<'_, Token>>) -> OptionLexClass{
      parse_id(tokens)
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

      #[test]
      fn test_parsing_tp() {
            let mut src_file = String::from("./src/parser/tests/tp.c0");
            let parser = Parser::new(&mut src_file);
            let mut tokens = parser.lexer().tokens();
            let mut tokens_peekable = tokens.iter().peekable();
            let first_parse = parse_tp(&mut tokens_peekable);
            let expected_first_parse = Some(LexClass::Tp(vec![ Either::Right(Token::Int), Either::Right(Token::Mult) ]));
            let second_parse  = parse_tp(&mut tokens_peekable);
            let expected_second_parse = Some(LexClass::Tp(vec![ Either::Right(Token::Bool), Either::Right(Token::LBracket), Either::Right(Token::RBracket) ]));
            let third_parse = parse_tp(&mut tokens_peekable);
            let expected_third_parse = Some(LexClass::Tp(vec![ Either::Right(Token::Struct), Either::Left(Box::new(LexClass::Id(Token::Undefined(Some('s')), vec![])))]));
            let fourth_parse = parse_tp(&mut tokens_peekable);
            let expected_fourth_parse = Some(LexClass::Tp(vec![Either::Left(Box::new(LexClass::Id(Token::Undefined(Some('c')), vec![Token::Undefined(Some('_'))])))]));
            assert_eq!(first_parse,  expected_first_parse);
            assert_eq!(second_parse, expected_second_parse);
            assert_eq!(third_parse,  expected_third_parse);
            assert_eq!(fourth_parse, expected_fourth_parse);
      }
}