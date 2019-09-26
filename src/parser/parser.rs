#![allow(dead_code)]
use lexer::lexer::*;
use std::boxed::Box;
use std::collections::vec_deque::Iter;
use std::iter::Peekable;
use either::Either;

type OptionLexClass = Option<LexClass>;
type VecToken = Vec<Token>;
type VecEither = Vec<Either<Box<LexClass>, Token>>;
// type PeekableTokens = Peekable<Iter<'a, Token>>;
type Combinator = fn(Peekable<Iter<'_, Token>>) -> OptionLexClass;

#[derive(Clone, PartialEq, Debug)]
pub enum LexClass {
    Empty,
    Id(VecToken             ),
    Num(Box<LexClass>       ),
    DecNum(Token            ),
    HexNum(VecToken         ), 
    StrLit(VecToken         ),
    ChrLit(Token            ), 
    LibLit(VecToken         ), 
    SChar(VecToken          ), 
    CChar(Token             ),
    NChar(Token             ), 
    LChar(Token             ), 
    Esc(Token               ), 
    Sep(Token               ), 
    Unop(Token              ), 
    Binop(Token             ), 
    Asnop(Token             ), 
    Postop(Token            ),
    Tp(VecEither            ),
    Lv(VecEither            ),
    Simple(VecEither        ),
    Exp(VecEither           ),
    Keyword(Token        )
}

pub trait ToEither {
    fn to_either(Self) -> VecEither;
}

impl ToEither for VecToken {
    fn to_either(ts: VecToken) -> VecEither {
        let mut accum: VecEither = Vec::new();
        for t in ts {
            accum.push(Either::Right(t));
        }
        accum
    }
}

struct Pair<T>(T, T);

pub trait Alternative {
    fn empty() -> Self;
    fn add(Self, Self) -> Self;
}

impl Alternative for OptionLexClass{
    fn empty() -> OptionLexClass{
        None
    }

    fn add(x: OptionLexClass, y: OptionLexClass) -> OptionLexClass{
        let pair: Pair<OptionLexClass> = Pair(x, y);
        match pair {
            Pair( Some(lc1), Some(lc2) ) => Some(LexClass::add(lc1, lc2)),
            Pair( None,      Some(lc)  ) => Some(lc), 
            Pair( Some(lc),  None      ) => Some(lc),
            _ => None,
        }
    }
}

impl Alternative for LexClass {
    fn empty() -> LexClass {
        LexClass::Empty
    }

    fn add(x: LexClass, y: LexClass) -> LexClass {
        let pair: Pair<LexClass> = Pair(x, y );
        let mut buff: VecEither = Vec::new();
        match pair {
            Pair(LexClass::Empty, LexClass::Empty)   => LexClass::Empty, 
            Pair(LexClass::Tp(op1), LexClass::Tp(op2)) => {
                buff.extend(op1);
                buff.extend(op2);
                LexClass::Tp(buff)
            }
            Pair(LexClass::Tp(op1), lc) => {
                buff.extend(op1);
                buff.extend(vec![Either::Left(Box::new(lc))]);
                LexClass::Tp(buff)
            },
            Pair(LexClass::Lv(op1), LexClass::Lv(op2)) => {
                buff.extend(op1);
                buff.extend(op2);
                LexClass::Lv(buff)
            }
            Pair(lc, LexClass::Lv(l)) => {
                buff.extend(vec![Either::Left(Box::new(lc))]);
                buff.extend(l);
                LexClass::Lv(buff)
            },
            Pair(LexClass::Lv(l), lc) => {
                buff.extend(l);
                buff.extend(vec![Either::Left(Box::new(lc))]);
                LexClass::Lv(buff)
            },
            Pair(LexClass::Simple(l), lc) =>{
                buff.extend(l);
                buff.extend(vec![Either::Left(Box::new(lc))]);
                LexClass::Simple(buff)
            },
            Pair(LexClass::Exp(l), lc) => {
                buff.extend(l);
                buff.extend(vec![Either::Left(Box::new(lc))]);
                LexClass::Exp(buff)
            },
            _ => panic!("No existing combination for add({:?}, {:?})", pair.0, pair.1)
        }
    }
}

#[derive(Clone)]
pub struct Parser {
    lexer: Lexer,
    parse_tree: Vec<LexClass>,
}

impl Parser {
    pub fn new(file_path: &mut String) -> Parser {
        Parser {
            lexer: Lexer::new(file_path),
            parse_tree: Vec::new(),
        }
    }

    pub fn empty() -> Parser {
        Parser{
            lexer: Lexer::empty(), 
            parse_tree: Vec::new(),
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
            let mut buff = vec![ *tokens.next().unwrap()];
            buff.extend(parse_id_end(tokens));
            
            Some(LexClass::Id(buff))
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
            match peek_non_whitespace(tokens) {
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

    match peek_non_whitespace(tokens) {
        Some(Token::Num(0)) => {
            let t = tokens.next().unwrap();
            match peek_non_whitespace(tokens) {
                Some(Token::Undefined(Some('x')))
                |Some(Token::Undefined(Some('X'))) 
                    => Some(LexClass::Num(Box::new(parse_hex(tokens)))),
                _     => Some(LexClass::Num(Box::new(LexClass::DecNum(*t)))),

            }
        },
        Some(Token::Num(_))  => 
            Some(LexClass::Num(Box::new(LexClass::DecNum(*tokens.next().unwrap())))),
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
    match peek_non_whitespace(tokens) {
        Some(Token::LParen)    
        | Some(Token::RParen)     
        | Some(Token::LBracket)    
        | Some(Token::RBracket)    
        | Some(Token::LCurly)    
        | Some(Token::RCurly)     
        | Some(Token::FieldSelect)
        | Some(Token::TernIf)
        | Some(Token::TernNot)
        | Some(Token::FieldDeref)
        | Some(Token::Comma) => Some(LexClass::Sep(*tokens.next().unwrap())), 
        _ => None, 
    }
}

fn parse_unop(tokens: &mut Peekable<Iter<'_, Token>>) -> OptionLexClass{
    match peek_non_whitespace(tokens) {
        Some(Token::Not)      
        | Some(Token::BitNot) 
        | Some(Token::Minus) => Some(LexClass::Unop(*tokens.next().unwrap())),
        Some(Token::Mult) => {
            tokens.next();
            Some(LexClass::Unop(Token::PointerDeref))
        },
        _ => None,
    }
}

fn parse_binop(tokens: &mut Peekable<Iter<'_, Token>>) -> OptionLexClass{
    match peek_non_whitespace(tokens) {
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
    match peek_non_whitespace(tokens) {
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

fn parse_postop(tokens: &mut Peekable<Iter<'_, Token>>) -> OptionLexClass {
    match peek_non_whitespace(tokens) {
        Some(Token::PostMinusEq)
        | Some(Token::PostPlusEq)  => Some(LexClass::Postop(*tokens.next().unwrap())), 
        _ => None
    }
}


fn parse_tp(tokens: &mut Peekable<Iter<'_, Token>>) -> OptionLexClass {

    let look_ahead = | tokens: &mut Peekable<Iter<'_, Token>>, prev: OptionLexClass| -> OptionLexClass{
        match peek_non_whitespace(tokens) {
            Some(Token::Mult) => {
                tokens.next();
                let parse = Some(LexClass::Tp(vec![Either::Right(Token::Mult)]));
                return OptionLexClass::add(prev, parse)
            },
            Some(Token::LBracket) => {
                let _ = parse_sep(tokens);
                let bracket_2 = parse_sep(tokens);
                assert_eq!(bracket_2, Some(LexClass::Sep(Token::RBracket)));
                let parse = Some(LexClass::Tp(vec![
                    Either::Right(Token::LBracket), 
                    Either::Right(Token::RBracket)
                    ]));
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
            let keyword = *tokens.next().unwrap();
            let p = Some(LexClass::Tp( vec![ Either::Right(keyword) ] ));
            look_ahead(tokens, p)
        },
        
        Some(Token::Struct) => {
            let _ = parse_keyword(tokens);
            let p = Some(LexClass::Tp(vec![ Either::Right(Token::Struct)]));
            OptionLexClass::add(p, parse_sid(tokens))
        }
        Some(Token::Undefined(Some(_))) => OptionLexClass::add(Some(LexClass::Tp(vec![])), parse_aid(tokens)),
        _ => None,
    }
}

fn parse_sid(tokens: &mut Peekable<Iter<'_, Token>>) -> OptionLexClass {
    parse_id(tokens)
}

fn parse_aid(tokens: &mut Peekable<Iter<'_, Token>>) -> OptionLexClass {
    parse_id(tokens)
}

fn parse_lv(tokens: &mut Peekable<Iter<'_, Token>>) -> OptionLexClass {
    let look_ahead = | parse: OptionLexClass, tokens: &mut Peekable<Iter<'_, Token>> | -> OptionLexClass {
        let mut ret = Some(LexClass::Lv(vec![Either::Left(Box::new(parse.unwrap()))]));
        match peek_non_whitespace(tokens) {
            Some(Token::FieldSelect) 
            | Some(Token::FieldDeref)=> {
                let t = *tokens.next().unwrap();
                ret = OptionLexClass::add(ret, Some(LexClass::Lv(vec![Either::Right(t)])));
                OptionLexClass::add(ret, parse_fid(tokens))
            },
            Some(Token::LBracket) => {
                OptionLexClass::add(ret, parse_exp(tokens))
            }
            _ => ret,
        }
    };

    match peek_non_whitespace(tokens) {
        Some(Token::Undefined(Some(_))) =>
            look_ahead(parse_vid(tokens), tokens),
        Some(Token::Mult) =>{
            tokens.next();
            OptionLexClass::add(Some(LexClass::Lv(vec![
                Either::Right(Token::PointerDeref)])), 
                parse_lv(tokens))
        },
        Some(Token::LParen) => {
            let buff = OptionLexClass::add(Some(LexClass::Lv(vec![
                Either::Right(*tokens.next().unwrap())])), 
                parse_lv(tokens));
            OptionLexClass::add(
                buff, 
                Some(LexClass::Lv(vec![Either::Right(*tokens.next().unwrap())])))
        },
        _ => None,
    }
}

fn parse_vid(tokens: &mut Peekable<Iter<'_, Token>>) -> OptionLexClass {
    parse_id(tokens)
}

fn parse_fid(tokens: &mut Peekable<Iter<'_, Token>>) -> OptionLexClass {
    parse_id(tokens)
}

fn parse_keyword(tokens: &mut Peekable<Iter<'_, Token>>) -> OptionLexClass {
    match peek_non_whitespace(tokens) {
        Some(Token::True)
        | Some(Token:: False)
        | Some(Token::Null)
        | Some(Token::Alloc)
        | Some(Token::AllocArray)
        | Some(Token::If)
        | Some(Token::Else)
        | Some(Token::While)
        | Some(Token::For)
        | Some(Token::Return) 
        | Some(Token::Assert) 
        | Some(Token::Error)
        | Some(Token::Break)
        | Some(Token::Continue)
        | Some(Token::Int)
        | Some(Token::Char)
        | Some(Token::Bool)
        | Some(Token::String)
        | Some(Token::Void)
        | Some(Token::Struct)
        | Some(Token::Typedef)
         => Some(LexClass::Keyword(*tokens.next().unwrap())),
        _ => None,
    }
}

fn parse_exp(tokens: &mut Peekable<Iter<'_, Token>>) -> OptionLexClass {
    
    let wrap_exp_parse = |parse_option: OptionLexClass| -> OptionLexClass {
        OptionLexClass::add(
            Some(LexClass::Exp(vec![])), 
            parse_option
        )
    };
    
    //<num>
    let mut parse: OptionLexClass;
    parse = parse_num(tokens);
    if parse != None {
        return wrap_exp_parse(parse);
    }

    //<strlit>
    parse = parse_strlit(tokens);
    if parse != None {
        return wrap_exp_parse(parse);
    }
    
    // <chrlit> 
    parse = parse_chrlit(tokens);
    if parse != None {
        return wrap_exp_parse(parse);
    }

    // true | false | NULL | alloc (<tp>) | alloc_array (<tp>, <exp>)
    parse = parse_keyword(tokens);
    if parse != None {
        parse = match parse {
            Some(LexClass::Keyword(v)) => {
                let mut parse_mov = Some(LexClass::Keyword(v));
                if v == Token::Alloc {
                    parse_mov = OptionLexClass::add(
                        parse_mov, 
                        parse_sep(tokens)
                    );
                    parse_mov = OptionLexClass::add(
                        parse_mov, 
                        parse_tp(tokens)
                    );
                    OptionLexClass::add(
                        parse_mov, 
                        parse_sep(tokens)
                    )
                }
                else if v == Token::AllocArray {
                    parse_mov = OptionLexClass::add(
                        parse_mov, 
                        parse_sep(tokens)
                    );
                    parse_mov = OptionLexClass::add(
                        parse_mov, 
                        parse_tp(tokens)
                    ); 
                    parse_mov = OptionLexClass::add(
                        parse_mov, 
                        parse_sep(tokens)
                    );
                    parse_mov = OptionLexClass::add(
                        parse_mov, 
                        parse_exp(tokens)
                    );
                    OptionLexClass::add(
                        parse_mov, 
                        parse_sep(tokens)
                    )
                }
                else {
                    parse_mov
                } 
             }
            _     => parse,
        };

        return wrap_exp_parse(parse);
    }

    // <vid> | <vid> ([<exp>(, <exp>)*])
    parse = parse_vid(tokens);
    if parse != None {
        let option_parse = match peek_non_whitespace(tokens) {
            Some(Token::LBracket) => {
                let mut acc = parse_sep(tokens);
                let maybe_exp = parse_exp(tokens);
                match maybe_exp {
                    None => OptionLexClass::add(
                        acc,
                        parse_sep(tokens),
                    ),
                    Some(_) => {
                        acc = OptionLexClass::add(
                            acc, 
                            maybe_exp
                        ); 
                        if peek_non_whitespace(tokens) == Some(Token::Comma){
                            acc = OptionLexClass::add(
                                acc, 
                                parse_sep(tokens)
                            );
                            acc = OptionLexClass::add(
                                acc, 
                                parse_exp(tokens) 
                            );
                        }
                        acc
                    }
                }

            }
            _ => None
        };

        return wrap_exp_parse(
            OptionLexClass::add(
                parse, 
                option_parse
            )
        )
    }

    // <unop><exp>
    parse = parse_unop(tokens);
    if parse != None {
        return wrap_exp_parse(            
            OptionLexClass::add(
                parse, 
                parse_exp(tokens)
            )
        );
    }

    // <exp> <binop> <exp> | <exp> [<exp>] | <exp> ? <exp> : <exp>|
    // <exp> . <fid> | <exp> -> <fid> 
    parse = parse_exp(tokens);
    if parse != None {
        let maybe_binop = parse_binop(tokens);
        match maybe_binop {
            Some(LexClass::Binop(Token::TernIf))
            | Some(LexClass::Binop(Token::TernNot)) => {
                parse = OptionLexClass::add(
                    parse, 
                    maybe_binop
                );
                return wrap_exp_parse(
                    OptionLexClass::add(
                        parse, 
                        parse_exp(tokens)
                    )
                )
            },
            Some(LexClass::Binop(Token::FieldDeref))
            | Some(LexClass::Binop(Token::FieldSelect)) => {
                parse = OptionLexClass::add(
                    parse, 
                    maybe_binop
                );
                return wrap_exp_parse (
                    OptionLexClass::add(
                        parse, 
                        parse_fid(tokens)
                    )
                )
            }
            Some(_) => {
                let p = OptionLexClass::add(
                    parse,
                    maybe_binop
                );
                return wrap_exp_parse(
                    OptionLexClass::add(
                        p, 
                        parse_exp(tokens)
                    )
                )
            }
            _ => ()
        }

        return wrap_exp_parse(
            OptionLexClass::add(
                parse, 
                parse_exp(tokens) 
            )
        )

    }

    None
}

fn parse_simple(tokens: &mut Peekable<Iter<'_, Token>>) -> OptionLexClass {
    let mut parse = parse_lv(tokens);
    if parse != None {
        let asnop = parse_asnop(tokens);
        if asnop != None {
            parse = OptionLexClass::add(parse, asnop);
            parse = OptionLexClass::add(parse, parse_exp(tokens));
        } else {
            parse = OptionLexClass::add(parse, parse_postop(tokens));
        }
        return OptionLexClass::add(Some(LexClass::Simple(Vec::new())), parse);
    }
    
    parse = parse_exp(tokens);
    if parse != None {
        return OptionLexClass::add(Some(LexClass::Simple(Vec::new())), parse);
    }

    parse = parse_tp(tokens);
    if parse != None {
        parse = OptionLexClass::add(parse, parse_vid(tokens));
        let asnop =  parse_asnop(tokens);
        if asnop != None {
            parse = OptionLexClass::add(parse, asnop);
            parse = OptionLexClass::add(parse, parse_exp(tokens));
        } 
        return OptionLexClass::add(Some(LexClass::Simple(Vec::new())), parse);
    } 
    None
}

// TODO: Parse <stmt>

#[cfg(test)]
mod test {
    use parser::parser::*;

    #[test]
    fn test_parsing_ids() {
        let mut src_file = String::from("./src/parser/tests/ids.c0");
        let parser = Parser::new(&mut src_file);
        let tokens = parser.lexer().tokens();
        let parse_output = parse_id(&mut tokens.iter().peekable());
        let expected_parse = Some(
            LexClass::Id(
                vec![
                    Token::Undefined(Some('A')),
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
        let parser = Parser::new(&mut src_file); 
        let tokens = parser.lexer().tokens();
        let parse_output = parse_num(&mut tokens.iter().peekable());
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
        let tokens = parser.lexer().tokens();
        let parse_output = parse_chrlit(&mut tokens.iter().peekable());
        let expected_parse = Some(
            LexClass::ChrLit(Token::Undefined(Some('a'))),
        );

        assert_eq!(parse_output, expected_parse);
    }
    
    #[test]
    fn test_parsing_liblit() {
        let mut src_file = String::from("./src/parser/tests/lib.c0");
        let parser = Parser::new(&mut src_file);
        let tokens = parser.lexer().tokens();
        let parse_output = parse_liblit(&mut tokens.iter().peekable()); 
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
        let tokens = parser.lexer().tokens();
        let mut tokens_peekable = tokens.iter().peekable();
        let first_parse = parse_tp(&mut tokens_peekable);
        let expected_first_parse = Some(LexClass::Tp(vec![ 
            Either::Right(Token::Int), 
            Either::Right(Token::Mult)]));
        let second_parse  = parse_tp(&mut tokens_peekable);
        let expected_second_parse = Some(LexClass::Tp(vec![ 
            Either::Right(Token::Bool), 
            Either::Right(Token::LBracket), 
            Either::Right(Token::RBracket) ]));
        let third_parse = parse_tp(&mut tokens_peekable);
        let expected_third_parse = Some(LexClass::Tp(vec![ 
            Either::Right(Token::Struct), 
            Either::Left(Box::new(LexClass::Id(vec![Token::Undefined(Some('s'))])))]));
        let fourth_parse = parse_tp(&mut tokens_peekable);
        let expected_fourth_parse = Some(LexClass::Tp(vec![
            Either::Left(Box::new(LexClass::Id(vec![Token::Undefined(Some('c')), 
            Token::Undefined(Some('_'))])))]));
        assert_eq!(first_parse,  expected_first_parse);
        assert_eq!(second_parse, expected_second_parse);
        assert_eq!(third_parse,  expected_third_parse);
        assert_eq!(fourth_parse, expected_fourth_parse);
    }

    #[test]
    fn test_parsing_lv() {
        let mut src_file = String::from("./src/parser/tests/lv.c0");
        let parser = Parser::new(&mut src_file);
        let mut tokens = parser.lexer().tokens();
        let mut tokens_peekable = tokens.iter().peekable();
        let first_parse = parse_lv(&mut tokens_peekable);
        let expected_first_parse = Some(LexClass::Lv(vec![
            Either::Left(Box::new(LexClass::Id(vec![Token::Undefined(Some('t'))]))),
            Either::Right(Token::FieldDeref),
            Either::Left(Box::new(LexClass::Id(vec![Token::Undefined(Some('x'))]))),
            ]));
        let second_parse = parse_lv(&mut tokens_peekable);
        let expected_second_parse = Some(LexClass::Lv(vec![
            Either::Left(Box::new(LexClass::Id(vec![Token::Undefined(Some('t'))]))),
            Either::Right(Token::FieldSelect),
            Either::Left(Box::new(LexClass::Id(vec![Token::Undefined(Some('x'))]))),
        ]));
        let third_parse = parse_lv(&mut tokens_peekable);
        let expected_third_parse = Some(LexClass::Lv(vec![
            Either::Right(Token::LParen),
            Either::Left(Box::new(LexClass::Id(vec![Token::Undefined(Some('t'))]))),
            Either::Right(Token::RParen),
        ]));
        let fourth_parse = parse_lv(&mut tokens_peekable);
        let expected_fourth_parse = Some(LexClass::Lv(vec![
            Either::Right(Token::PointerDeref),
            Either::Right(Token::PointerDeref),
            Either::Left(Box::new(LexClass::Id(vec![Token::Undefined(Some('t'))]))),
        ]));
        assert_eq!(first_parse, expected_first_parse);
        assert_eq!(second_parse, expected_second_parse);
        assert_eq!(third_parse, expected_third_parse);
        assert_eq!(fourth_parse, expected_fourth_parse);
        // TODO: Expr testing
    }

    #[test]
    fn test_parsing_simple() {
        let mut src_file = String::from("./src/parser/tests/simple.c0");
        let parser = Parser::new(&mut src_file);
        let tokens = parser.lexer().tokens();
        let mut tokens_peekable = tokens.iter().peekable();
        // TODO: Expr testing
        let first_parse = parse_simple(&mut tokens_peekable);
        let expected_first_parse = Some(LexClass::Simple(
            vec![
                Either::Left(Box::new(LexClass::Id(vec![Token::Undefined(Some('a'))]))),
                Either::Right(Token::PostPlusEq),
            ]
        ));
        let second_parse = parse_simple(&mut tokens_peekable);
        let expected_second_parse = Some(LexClass::Simple(
            vec![
                Either::Right(Token::PointerDeref),
                Either::Left(Box::new(LexClass::Id(vec![Token::Undefined(Some('b'))]))),
                Either::Right(Token::PostMinusEq),
            ]
        ));
        assert_eq!(first_parse, expected_first_parse);
        assert_eq!(second_parse, expected_second_parse);
    }

    #[test]
    pub fn testing_keyword() {
        let mut src_file = String::from("./src/parser/tests/keyword.c0");
        let parser = Parser::new(&mut src_file);
        let tokens = parser.lexer().tokens();
        let mut tokens_peekable = tokens.iter().peekable();

        let mut parse = parse_keyword(&mut tokens_peekable);
        let mut expected_parse = Some(LexClass::Keyword(Token::If));
        assert_eq!(parse, expected_parse);
        
        parse = parse_keyword(&mut tokens_peekable);
        expected_parse = Some(LexClass::Keyword(Token::Else));
        assert_eq!(parse, expected_parse);

        parse = parse_keyword(&mut tokens_peekable);
        expected_parse = Some(LexClass::Keyword(Token::While));
        assert_eq!(parse, expected_parse);

        parse = parse_keyword(&mut tokens_peekable);
        expected_parse = Some(LexClass::Keyword(Token::For));
        assert_eq!(parse, expected_parse);

        parse = parse_keyword(&mut tokens_peekable);
        expected_parse = Some(LexClass::Keyword(Token::Return));
        assert_eq!(parse, expected_parse);

        parse = parse_keyword(&mut tokens_peekable);
        expected_parse = Some(LexClass::Keyword(Token::Assert));
        assert_eq!(parse, expected_parse);

        parse = parse_keyword(&mut tokens_peekable);
        expected_parse = Some(LexClass::Keyword(Token::Error));
        assert_eq!(parse, expected_parse);

        parse = parse_keyword(&mut tokens_peekable);
        expected_parse = Some(LexClass::Keyword(Token::Struct));
        assert_eq!(parse, expected_parse);

        parse = parse_keyword(&mut tokens_peekable);
        expected_parse = Some(LexClass::Keyword(Token::Typedef));
        assert_eq!(parse, expected_parse);

        parse = parse_keyword(&mut tokens_peekable);
        expected_parse = Some(LexClass::Keyword(Token::Int));
        assert_eq!(parse, expected_parse);

        parse = parse_keyword(&mut tokens_peekable);
        expected_parse = Some(LexClass::Keyword(Token::Bool));
        assert_eq!(parse, expected_parse);

        parse = parse_keyword(&mut tokens_peekable);
        expected_parse = Some(LexClass::Keyword(Token::String));
        assert_eq!(parse, expected_parse);

        parse = parse_keyword(&mut tokens_peekable);
        expected_parse = Some(LexClass::Keyword(Token::Char));
        assert_eq!(parse, expected_parse);

        parse = parse_keyword(&mut tokens_peekable);
        expected_parse = Some(LexClass::Keyword(Token::Void));
        assert_eq!(parse, expected_parse);

        parse = parse_keyword(&mut tokens_peekable);
        expected_parse = Some(LexClass::Keyword(Token::True));
        assert_eq!(parse, expected_parse);

        parse = parse_keyword(&mut tokens_peekable);
        expected_parse = Some(LexClass::Keyword(Token::False));
        assert_eq!(parse, expected_parse);

        parse = parse_keyword(&mut tokens_peekable);
        expected_parse = Some(LexClass::Keyword(Token::Null));
        assert_eq!(parse, expected_parse);

        parse = parse_keyword(&mut tokens_peekable);
        expected_parse = Some(LexClass::Keyword(Token::Alloc));
        assert_eq!(parse, expected_parse);

        // TODO: Solve the alloc array keyword problem
        // parse = parse_keyword(&mut tokens_peekable);
        // expected_parse = Some(LexClass::Keyword(Token::AllocArray]));
        // assert_eq!(parse, expected_parse);

        parse = parse_keyword(&mut tokens_peekable);
        expected_parse = Some(LexClass::Keyword(Token::Break));
        assert_eq!(parse, expected_parse);

        parse = parse_keyword(&mut tokens_peekable);
        expected_parse = Some(LexClass::Keyword(Token::Continue));
        assert_eq!(parse, expected_parse);
    }
    
    // TODO: <expr> testing
    #[test]
    pub fn test_parse_expr() {
        // let mut src_file = String::from("./src/parser/tests/expr.c0");
        // let parser = Parser::new(&mut src_file);
        // let tokens = parser.lexer().tokens();
        // let mut tokens_peekable = tokens.iter().peekable();

        // let mut parse = parse_exp(&mut tokens_peekable);
        // let mut expected_parse = Some(LexClass::Exp(vec![]));

    }

    // TODO: <stmt> testing
}