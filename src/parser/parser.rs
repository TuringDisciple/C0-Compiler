#![allow(non_snake_case)]
use lexer::lexer::*;
use std::vec::Vec;
use std::result;
use std::collections::VecDeque;
use std::string::String;

/*
The purpose of the parser is to apply semantic meaning to our language lexemes.
Our parser must therefore perform the following functions

1. Construct semantically correct structures from lexems following the language grammar
2. Identify syntactic errors in source


Fortunately there is a formal grammar specification
for the C0 language. The problems with this grammar is that
it's left recursive which will introduce complications with parsing down the line.
 */
#[derive(Clone, Debug, PartialEq)]
enum Exp{
    Id(Vec<Token>),
    Num(u32),
    Sep(Token),
    Unop(Token),
    Binop(Token),
    Asnop(Token),
    Postop(Token),
    Tp(Vec<Token>)
}

#[derive(Clone)]
pub struct Parser {
    lexer: Lexer,
    head: Option<Token>,
}
impl Parser {
    pub fn new(file_path: &mut String) -> Parser {
        let mut lexer = Lexer::new(file_path);
        let head = lexer.next();
        Parser {
            lexer,
            head,
        }
    }

    fn eat(&mut self, t: Token) -> Result<Token, ()>{
        match self.head {
            Some(h) =>{
                match (h, t) {
                    (Token::Undefined(_), Token::Undefined(_)) |
                    (Token::Num(_), Token::Num(_)) => {
                        self.head = self.lexer.next();
                        Ok(h)
                    },
                    (_, _) => {
                        if h == t {
                            self.head = self.lexer.next();
                            Ok(h)
                        } else {
                            Err(())
                        }
                    }
                }
            },
            _ => Err(()),
        }
    }

    /* The following parse functions handle the parsing of language rules that produce
     * tokens
    <id> ::= [A-Za-z_][A-Za-z0-9_]*
    <num> ::= <decnum> | <hexnum>
    <decnum> ::= 0 | [1-9][0-9]*
    <hexnum> ::= 0[xX][0-9a-fA-F]+
    <strlit> ::= "<schar>*"
    <chrlit> ::= ’<cchar>’
    <liblit> ::= <<lchar>*>
    <schar> ::= <nchar> | <esc>
    <cchar> ::= <nchar> | <esc> | " | \0
    <nchar> ::= (normal printing character except ")
    <lchar> ::= (normal printing character except >)
    <esc> ::= \n | \t | \v | \b | \r | \f | \a
    | \\ | \’ | \"
    <sep> ::= ( | ) | [ | ] | { | } | , | ;
    <unop> ::= ! | ~ | - | *
    <binop> ::= . | -> | * | / | % | + | - | << | >>
    | < | <= | >= | > | == | !=
    | & | ^ | | | && | || | ? | :
    <asnop> ::= = | += | -= | *= | /= | %= | <<= | >>=
    | &= | ^= | |=
    <postop> ::= -- | ++
     */

    // <id> ::= [A-Za-z_][A-Za-z0-9_]*
    pub fn parseId(&mut self) -> Result<Exp, ()>{
        let mut tokens = Vec::new();
        loop {
            match self.eat(Token::Undefined(None)){
                Ok(t) => tokens.push(t),
                Err(()) => break,
            }
        }
        if tokens.len() > 0 {
            Ok(Exp::Id(tokens))
        } else {
            Err(())
        }
    }

    // num ::= 0 | [1-9][0-9]* // TODO: lexing hex
    pub fn parseNum(&mut self) -> Result<Exp, ()>{
        match self.eat(Token::Num(0)) {
            Ok(Token::Num(x)) => Ok(Exp::Num(x)),
            _ => Err(()),
        }
    }

    //<sep> ::= ( | ) | [ | ] | { | } | , | ;
    pub fn parseSep(&mut self) -> Result<Exp, ()> {
        for t in vec![
            Token::LParen,
            Token::RParen,
            Token::LBracket,
            Token::RBracket,
            Token::LCurly,
            Token::RCurly,
            Token::Comma,
            Token::SemiColon
        ] {
            match self.eat(t) {
                Ok(t) => return Ok(Exp::Sep(t)),
                _ => (),
            }
        }
        Err(())
    }

    // <unop> ::= ! | ~ | - | *
    pub fn parseUnop(&mut self) -> Result<Exp, ()> {
        for t in vec![
            Token::Not,
            Token::BitNot,
            Token::Minus,
            Token::Mult,
        ] {
            match self.eat(t) {
                Ok(Token::Mult) => return Ok(Exp::Unop(Token::PointerDeref)),
                Ok(t) => return Ok(Exp::Unop(t)),
                _ => (),
            }
        }
        Err(())
    }

    //<binop> ::= . | -> | * | / | % | + | - | << | >>
    //    | < | <= | >= | > | == | !=
    //    | & | ^ | | | && | || | ? | :
    pub fn parseBinop(&mut self) -> Result<Exp, ()> {
        for t in vec![
            Token::FieldSelect,
            Token::FieldDeref,
            Token::Mult,
            Token::Div,
            Token::Mod,
            Token::Mod,
            Token::Plus,
            Token::Minus,
            Token::LShift,
            Token::RShift,
            Token::Lt,
            Token::Lte,
            Token::Gte,
            Token::Gt,
            Token::Equality,
            Token::NotEq,
            Token::And,
            Token::Xor,
            Token::Or,
            Token::BooleanAnd,
            Token::BooleanOr,
            Token::TernIf,
            Token::TernElse,
        ] {
            match self.eat(t) {
                Ok(t) => return Ok(Exp::Binop(t)),
                _ => (),
            }
        }
        Err(())
    }

    //<asnop> ::= = | += | -= | *= | /= | %= | <<= | >>=
    //    | &= | ^= | |=
    pub fn parseAsnop(&mut self) -> Result<Exp, ()> {
        for t in vec![
            Token::Equal,
            Token::PlusEq,
            Token::MinusEq,
            Token::MultEq,
            Token::DivEq,
            Token::ModEq,
            Token::LShiftEq,
            Token::RShiftEq,
            Token::AndEq,
            Token::XorEq,
            Token::OrEq,
        ] {
            match self.eat(t) {
                Ok(t) => return Ok(Exp::Asnop(t)),
                _ => (),
            }
        }

        Err(())
    }
    //<postop> ::= -- | ++
    pub fn parsePostop(&mut self) -> Result<Exp, ()> {
        for t in vec![
            Token::PostPlusEq,
            Token::PostMinusEq,
        ]{
            match self.eat(t) {
                Ok(t) => return Ok(Exp::Postop(t)),
                _ => (),
            }
        }

        Err(())
    }

    /*
    <tp> symbols are our first case of directly left recursive grammar rules

   <tp> ::= int | bool | string | char | void
            | <tp> * | <tp> [ ] | struct <sid> | <aid>

    Directly left recursive grammar rules are of the form

    A -> Ab

    In the tp case these are the legal transforms
    <tp> ::= <tp> * | <tp> []

    These introduce problems in parsing as they would cause an infinite when being parsed
    For example in this example

    int ***
    This is a valid pointer declaration, but first into the token stream will be int, thereby leaving ***
    not bound. Alternatively we could "look ahead", i.e. <tp> ***, parse *** and try to parse <tp>
    but this could lead to infinite recursion

    I will be following the paper "Removing Left Recursion from Context-Free Grammars" to solve this issue.
    This will be done using Paull's algorithm. An rough illustration is below

    <tp> ::= <tp> * | <tp> [] | B1 ... | BS (B1...Bs are the renaming non-left recursive productions)

    We modify to the following

    <tp> ::= B1<tp'> | ... | Bs<tp'>
    <tp'> ::= * | *<tp'> | [] | []<tp'>
    */
    fn _parseTp(&mut self, acc: &mut Vec<Token>) -> Vec<Token> {
        for t in vec![
            Token::Mult,
            Token::LBracket,
        ] {
            match self.eat(t) {
                Ok(Token::LBracket) => {
                    match self.eat(Token::RBracket) {
                        Ok(_) => {
                            acc.push(Token::LBracket);
                            acc.push(Token::RBracket);
                            return self._parseTp(acc);
                        },
                        _ => panic!("Unmatched left square bracket in source"),
                    }
                }
                Ok(t) => {
                    acc.push(t);
                    return self._parseTp(acc);
                }
                _ => (),
            }
        }
        acc.to_vec()
    }
    pub fn parseTp(&mut self) -> Result<Exp, ()> {
        let mut _tpAcc: Vec<Token> = vec![];
        for t in vec![
            Token::Int,
            Token::Bool,
            Token::Char,
            Token::String,
            Token::Void,
            Token::Struct,
            Token::Undefined(None),
            // TODO:
        ] {
            match self.eat(t) {
                Ok(t) => {
                    match t {
                        Token::Struct => {
                            let mut id = match self.parseId() {
                                Ok(Exp::Id(t)) => t,
                                _ => panic!("No identifier for struct"),
                            };
                            _tpAcc.push(t);
                            _tpAcc.append(&mut id);
                        }
                        Token::Undefined(_) => {
                            let mut id = vec![t];
                            match self.parseId() {
                                Ok(Exp::Id(result)) => {
                                    id.append(&mut result.clone());
                                    _tpAcc.append(&mut id);
                                }
                                _ => (),
                            }
                        }
                        _ => _tpAcc.push(t),
                    }
                    let leftRecursion = self._parseTp(&mut _tpAcc);
                    return Ok(Exp::Tp(leftRecursion))
                },
                _ => (),
            }
        }

        Err(())
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsingLexicalTokens() {
        let mut parser = Parser::new(&mut String::from("./src/parser/tests/tokens.txt"));
        let expectedResult: Vec<Result<Exp, ()>> = vec![
            Ok(Exp::Id(vec![Token::Undefined(Some('v')), Token::Undefined(Some('a'))])),
            Ok(Exp::Num(1234)),
            Ok(Exp::Sep(Token::LParen)),
            Ok(Exp::Sep(Token::RParen)),
            Ok(Exp::Sep(Token::LBracket)),
            Ok(Exp::Sep(Token::RBracket)),
            Ok(Exp::Sep(Token::LCurly)),
            Ok(Exp::Sep(Token::RCurly)),
            Ok(Exp::Sep(Token::Comma)),
            Ok(Exp::Sep(Token::SemiColon)),
            Ok(Exp::Unop(Token::Not)),
            Ok(Exp::Unop(Token::BitNot)),
            Ok(Exp::Unop(Token::PointerDeref)),
            Ok(Exp::Unop(Token::Minus)),
            Ok(Exp::Binop(Token::FieldSelect)),
            Ok(Exp::Binop(Token::FieldDeref)),
            Ok(Exp::Binop(Token::Div)),
            Ok(Exp::Binop(Token::Mod)),
            Ok(Exp::Binop(Token::Plus)),
            Ok(Exp::Binop(Token::LShift)),
            Ok(Exp::Binop(Token::RShift)),
            Ok(Exp::Binop(Token::Lt)),
            Ok(Exp::Binop(Token::Lte)),
            Ok(Exp::Binop(Token::Gte)),
            Ok(Exp::Binop(Token::Gt)),
            Ok(Exp::Binop(Token::Equality)),
            Ok(Exp::Binop(Token::NotEq)),
            Ok(Exp::Binop(Token::And)),
            Ok(Exp::Binop(Token::Xor)),
            Ok(Exp::Binop(Token::Or)),
            Ok(Exp::Binop(Token::BooleanAnd)),
            Ok(Exp::Binop(Token::BooleanOr)),
            Ok(Exp::Binop(Token::TernIf)),
            Ok(Exp::Binop(Token::TernElse)),
            Ok(Exp::Asnop(Token::Equal)),
            Ok(Exp::Asnop(Token::PlusEq)),
            Ok(Exp::Asnop(Token::MinusEq)),
            Ok(Exp::Asnop(Token::MultEq)),
            Ok(Exp::Asnop(Token::DivEq)),
            Ok(Exp::Asnop(Token::ModEq)),
            Ok(Exp::Asnop(Token::LShiftEq)),
            Ok(Exp::Asnop(Token::RShiftEq)),
            Ok(Exp::Asnop(Token::AndEq)),
            Ok(Exp::Asnop(Token::XorEq)),
            Ok(Exp::Asnop(Token::OrEq)),
            Ok(Exp::Postop(Token::PostMinusEq)),
            Ok(Exp::Postop(Token::PostPlusEq)),
        ];
        let mut results: Vec<Result<Exp, ()>> = Vec::new();
        loop {
            match parser.parseId() {
                Ok(exp) => {
                    results.push(Ok(exp));
                    continue
                },
                _ => (),
            }

            match parser.parseNum() {
                Ok(exp) => {
                    results.push(Ok(exp));
                    continue
                },
                _ => (),
            }

            match parser.parseSep() {
                Ok(exp) => {
                    results.push(Ok(exp));
                    continue
                },
                _ => (),
            }

            match parser.parseUnop(){
                Ok(exp) => {
                    results.push(Ok(exp));
                    continue
                },
                _ => (),
            }

            match parser.parseBinop() {
                Ok(exp) => {
                    results.push(Ok(exp));
                    continue
                },
                _ => (),
            }

            match parser.parseAsnop() {
                Ok(exp) => {
                    results.push(Ok(exp));
                    continue
                },
                _ => (),
            }

            match parser.parsePostop(){
                Ok(exp) => {
                    results.push(Ok(exp));
                    continue
                }
                _ => ()
            }
            break
        }
        assert_eq!(expectedResult.len(), results.len());
        let count = expectedResult.into_iter().zip(results).filter(|(a, b)| {
            match (a, b) {
                (Ok(x), Ok(y)) => *x != *y,
                _ => true,
            }
        }).count();
        assert_eq!(count, 0);
    }
}



