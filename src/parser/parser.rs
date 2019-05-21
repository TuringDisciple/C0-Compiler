use lexer::lexer::*;
use std::boxed::Box;
use std::collections::{VecDeque};
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
      ChrLit(Vec<Token>       ), 
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

// Returns: Parse tree in the form of a vector of lex class tokens,
// pub fn parse_tokens(tokens: &mut VecDeque<Token>) -> Vec<LexClass> {
//       let mut tokens_peek = tokens.iter().peekable();
//       let mut parse_tree = Vec::new();
//       loop {
//             match tokens_peek.next(){
//                   Some(Token::Undefined(Some(c))) =>
//                         parse_tree.push(parse_id(Token::Undefined(Some(*c)), &mut tokens_peek)),
//                   Some(Token::Num(n)) =>
//                         parse_tree.push(parse_num(Token::Num(*n), &mut tokens_peek)),
//                   Some(Token::LParen)      => parse_tree.push(LexClass::Sep(Token::LParen)),
//                   Some(Token::RParen)      => parse_tree.push(LexClass::Sep(Token::RParen)), 
//                   Some(Token::LBracket)    => parse_tree.push(LexClass::Sep(Token::LBracket)),
//                   Some(Token::RBracket)    => parse_tree.push(LexClass::Sep(Token::RBracket)),
//                   Some(Token::LCurly)      => parse_tree.push(LexClass::Sep(Token::LCurly)),
//                   Some(Token::RCurly)      => parse_tree.push(LexClass::Sep(Token::RCurly)), 
//                   Some(Token::Comma)       => parse_tree.push(LexClass::Sep(Token::Comma)),
//                   Some(Token::SemiColon)   => parse_tree.push(LexClass::Sep(Token::SemiColon)),
//                   Some(Token::Not)         => parse_tree.push(LexClass::Unop(Token::Not)), 
//                   Some(Token::BitNot)      => parse_tree.push(LexClass::Unop(Token::BitNot)),
//                   Some(Token::Minus)       => parse_tree.push(LexClass::Unop(Token::Minus)), 
//                   Some(Token::Mult)        => parse_tree.push(LexClass::Unop(Token::Mult)),
//                   Some(Token::FieldSelect) => parse_tree.push(LexClass::Binop(Token::FieldSelect)),
//                   Some(Token::FieldDeref)  => parse_tree.push(LexClass::Binop(Token::FieldDeref)),
//                   Some(Token::Div)         => parse_tree.push(LexClass::Binop(Token::Div)), 
//                   Some(Token::Mod)         => parse_tree.push(LexClass::Binop(Token::Mod)),
//                   Some(Token::Plus)        => parse_tree.push(LexClass::Binop(Token::Plus)), 
//                   Some(Token::LShift)      => parse_tree.push(LexClass::Binop(Token::LShift)),
//                   Some(Token::RShift)      => parse_tree.push(LexClass::Binop(Token::RShift)),
//                   Some(Token::Lt)          => parse_tree.push(LexClass::Binop(Token::Lt)), 
//                   Some(Token::Lte)         => parse_tree.push(LexClass::Binop(Token::Lte)), 
//                   Some(Token::Gte)         => parse_tree.push(LexClass::Binop(Token::Gte)), 
//                   Some(Token::Gt)          => parse_tree.push(LexClass::Binop(Token::Gt)), 
//                   Some(Token::Equality)    => parse_tree.push(LexClass::Binop(Token::Equality)), 
//                   Some(Token::NotEq)       => parse_tree.push(LexClass::Binop(Token::NotEq)), 
//                   Some(Token::And)         => parse_tree.push(LexClass::Binop(Token::And)), 
//                   Some(Token::Xor)         => parse_tree.push(LexClass::Binop(Token::Xor)), 
//                   Some(Token::Or)          => parse_tree.push(LexClass::Binop(Token::Or)), 
//                   Some(Token::BooleanAnd)  => parse_tree.push(LexClass::Binop(Token::BooleanAnd)),
//                   Some(Token::BooleanOr)   => parse_tree.push(LexClass::Binop(Token::BooleanOr)), 
//                   Some(Token::BooleanNot)  => parse_tree.push(LexClass::Binop(Token::BooleanNot)), 
//                   Some(Token::TernIf)      => parse_tree.push(LexClass::Binop(Token::TernIf)),
//                   Some(Token::TernNot)     => parse_tree.push(LexClass::Binop(Token::TernNot)), 
//                   Some(Token::Equal)       => parse_tree.push(LexClass::Asnop(Token::Equal)), 
//                   Some(Token::PlusEq)      => parse_tree.push(LexClass::Asnop(Token::PlusEq)),
//                   Some(Token::MinusEq)     => parse_tree.push(LexClass::Asnop(Token::MinusEq)),
//                   Some(Token::MultEq)      => parse_tree.push(LexClass::Asnop(Token::MultEq)),
//                   Some(Token::DivEq)       => parse_tree.push(LexClass::Asnop(Token::DivEq)),
//                   Some(Token::ModEq)       => parse_tree.push(LexClass::Asnop(Token::ModEq)),
//                   Some(Token::LShiftEq)    => parse_tree.push(LexClass::Asnop(Token::LShiftEq)),
//                   Some(Token::RShiftEq)    => parse_tree.push(LexClass::Asnop(Token::RShiftEq)),
//                   Some(Token::AndEq)       => parse_tree.push(LexClass::Asnop(Token::AndEq)),
//                   Some(Token::XorEq)       => parse_tree.push(LexClass::Asnop(Token::XorEq)),
//                   Some(Token::OrEq)        => parse_tree.push(LexClass::Asnop(Token::OrEq)),
//                   Some(Token::PostMinusEq) => parse_tree.push(LexClass::Postop(Token::PostMinusEq)), 
//                   Some(Token::PostPlusEq)  => parse_tree.push(LexClass::Postop(Token::PostPlusEq)),
//                   _ => break,
//             }
//       }

//       parse_tree
// }

// <id> ::= [A-Za-z_][A-Za-z0-9_]*
fn parse_id(tokens: &mut Peekable<Iter<'_, Token>>) -> Option<LexClass> {
      let parse_id_end = |tokens: &mut Peekable<Iter<'_, Token>>| -> Vec<Token> {
            let ret: Vec<Token>;
            loop {
                  match tokens.peek() {
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
      let mut parse_hex = |tokens: &mut Peekable<Iter<'_, Token>>| -> LexClass {
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
                        | Some(Token::Undefined(Some('X'))) => Some(parse_hex(tokens)),
                        _ => Some(LexClass::DecNum(*t)),

                  }
            },
            Some(Token::Num(_)) => Some(LexClass::DecNum(*tokens.next().unwrap())),
            _ => None,
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
      use lexer::lexer::*;
      use parser::parser::*;

      // #[test]
      // fn test_parsing_ids() {
      //       let mut tokens: VecDeque<Token> = VecDeque::new();
      //       tokens.push_back(Token::Undefined(Some('a')));
      //       tokens.push_back(Token::Undefined(Some('Z')));
      //       tokens.push_back(Token::Num(0));
      //       tokens.push_back(Token::Num(9));

      //       let parse_tree = parse_tokens(&mut tokens);
      //       let expected_parse_tree = vec![
      //             LexClass::Id(
      //                   Token::Undefined(Some('a')),
      //                   vec![
      //                         Token::Undefined(Some('Z')), 
      //                         Token::Num(0),
      //                         Token::Num(9)])];

      //       assert_eq!(expected_parse_tree, parse_tree);
      // }

      // #[test]
      // fn test_parsing_ids_from_file() {
      //       let mut src_file = String::from("./src/parser/tests/ids.c0");
      //       let mut parser = Parser::new(&mut src_file);
      //       let mut tokens = parser.lexer().tokens();
      //       let mut parse_tree = parse_tokens(&mut tokens);
      //       let expected_parse_tree = vec![
      //             LexClass::Id(
      //                   Token::Undefined(Some('A')),
      //                   vec![
      //                         Token::Undefined(Some('a')),
      //                         Token::Num(123),
      //                   ]
      //             )];
            
      //       assert_eq!(expected_parse_tree, parse_tree);
      // }

      // #[test]
      // fn test_parsing_hex() {
      //       let mut src_file = String::from("./src/parser/tests/hex.c0");
      //       let mut parser = Parser::new(&mut src_file); 
      //       let mut tokens = parser.lexer().tokens();
      //       println!("{:?}", tokens);
      //       let mut parse_tree = parse_tokens(&mut tokens);
      //       let expected_parse_tree = vec![
      //             LexClass::Num(
      //                   Box::new(
      //                         LexClass::HexNum(
      //                               vec![
      //                                     Token::Num(0), 
      //                                     Token::Undefined(Some('x')),
      //                                     Token::Num(19), 
      //                                     Token::Undefined(Some('a')), 
      //                                     Token::Undefined(Some('F'))]
      //                         )
      //                   ))
      //       ];

      //       assert_eq!(expected_parse_tree, parse_tree);
      // }
}