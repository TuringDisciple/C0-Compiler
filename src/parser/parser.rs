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



