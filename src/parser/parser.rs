#![allow(non_snake_case)]
use lexer::lexer::*;
use std::vec::Vec;
use std::result;
use std::collections::VecDeque;
use std::string::String;
use either::Either;

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
    Tp(Vec<Token>),
    Expr( Vec<Either<Exp,Token>>),
}
fn seps() -> Vec<Token> {
    vec![
        Token::LParen,
        Token::RParen,
        Token::LBracket,
        Token::RBracket,
        Token::LCurly,
        Token::RCurly,
        Token::Comma,
        Token::SemiColon
    ]
}

fn unops() -> Vec<Token> {
    vec![
        Token::Not,
        Token::BitNot,
        Token::Minus,
        Token::Mult,
    ]
}

fn binops() -> Vec<Token> {
    vec![
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
    ]
}

fn asnops() -> Vec<Token>{
    vec![
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
    ]
}
fn postops() -> Vec<Token>{
    vec![
        Token::PostMinusEq,
        Token::PostPlusEq,
    ]
}
fn exprTokens() -> Vec<Token> {
    vec![
        Token::LParen,
        Token::True,
        Token::False,
        Token::Num(1),
        Token::Undefined(None),
        Token::Alloc,
        Token::AllocArray,
        Token::Null,
    ]
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
    // TODO?: support for stringlit and chrlit

    // <id> ::= [A-Za-z_][A-Za-z0-9_]*
    pub fn parseId(&mut self) -> Result<Exp, ()>{
        let mut tokens = Vec::new();
        loop {
            match self.eat(Token::Undefined(None)){
                Ok(t) => tokens.push(t),
                Err(_) => break,
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
        for t in seps()  {
            match self.eat(t) {
                Ok(t) => return Ok(Exp::Sep(t)),
                _ => (),
            }
        }
        Err(())
    }


    // <unop> ::= ! | ~ | - | *
    pub fn parseUnop(&mut self) -> Result<Exp, ()> {
        for t in unops() {
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
        for t in binops() {
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
        for t in asnops(){
            match self.eat(t) {
                Ok(t) => return Ok(Exp::Asnop(t)),
                _ => (),
            }
        }

        Err(())
    }

    //<postop> ::= -- | ++
    pub fn parsePostop(&mut self) -> Result<Exp, ()> {
        for t in postops() {
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

    <tp> ::= B1 | B1<tp'> | ... | Bs | Bs<tp'>
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
                    acc.push(Token::PointerDeref);
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
                            _tpAcc.push(t);
                            match self.parseId() {
                                Ok(Exp::Id(result)) => {
                                    _tpAcc.append(&mut result.clone());
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
    /*
    Similar to <tp>, <exp> consists of a lot of left recursive generators
    <exp> ::= ( <exp> )
    | <num> | true | false | NULL
    | <vid> | <exp> <binop> <exp> | <unop> <exp>
    | <exp> ? <exp> : <exp>
    | <vid> ( [<exp> (, <exp>)*] )
    | <exp> . <fid> | <exp> -> <fid>
    | <exp> [ <exp> ]
    | alloc ( <tp> ) | alloc_array ( <tp> , <exp> )

    Using Paull's algorithm we get the following

    <exp> ::= ( <exp> ) | ( <exp> ) <exp'> | <num> | <num> <exp'>
              | true | true <exp'>
              | false | false <exp'> | NULL | NULL <exp'> | <vid> | <vid> <exp'> | <unop><exp> | <unop> <exp> <exp'>
              | <vid> ( [<exp> (, <exp>)*] )
              | <vid> ( [<exp> (, <exp>)*] ) <exp'>
              | alloc (<tp> )
              | alloc (<tp> ) <exp'>
              | alloc_array (<tp>, <exp> )
              | alloc_array (<tp>, <exp> ) <exp'>

    <exp'> ::= <binop> <exp> | <binop> <exp> <exp'> | ? <exp> : <exp> | ? <exp> : <exp> <exp'>
              | . <fid> | . <fid> <exp'> | -> <fid> | -> <fid> <exp'> | [ <exp> ] | [ <exp> ] <exp'>

    */
    fn _parseExp (&mut self, acc: &mut Vec<Either<Exp, Token>> ){
        let mut startTokens = binops();
        startTokens.push(Token::LBracket);
        for t in startTokens{
            match self.eat(t) {
                Ok(Token::LBracket) => {
                    acc.push(Either::Right(Token::LBracket));
                    match self.parseExp() {
                        Ok(e) => acc.push(Either::Left(e)),
                        _ => (),
                    }
                    match self.eat(Token::RBracket) {
                        Ok(Token::RBracket) => {
                            acc.push(Either::Right(Token::RBracket));
                            self._parseExp(acc);
                        }
                        _ => panic!("Improper syntax, missing closing bracket")
                    }
                }
                Ok(Token::FieldSelect)
                | Ok(Token::FieldDeref) => {
                    acc.push(Either::Right(t));
                    match self.parseId() {
                        Ok(e) => acc.push(Either::Left(e)),
                        _ => panic!("Invalid struct access, no field id"),
                    }
                    self._parseExp(acc);
                }
                Ok(Token::TernIf) => {
                    acc.push(Either::Right(Token::TernIf));
                    match self.parseExp() {
                        Ok(e) => acc.push(Either::Left(e)),
                        _ => (),
                    }
                    match self.eat(Token::Else) {
                        Ok(t) => {
                            acc.push(Either::Right(t));
                            match self.parseExp() {
                                Ok(e) => acc.push(Either::Left(e)),
                                _ => ()
                            }
                            self._parseExp(acc);
                        }
                        _ => panic!("unmatched ternary else"),
                    }
                }
                Ok(t) => {
                    acc.push(Either::Right(t));
                    match self.parseExp() {
                        Ok(e) => acc.push(Either::Left(e)),
                        _ => (),
                    }
                    self._parseExp(acc);
                }
                _ => ()
            }
        }
    }


    pub fn parseExp(&mut self) -> Result<Exp, ()> {
        let mut acc: Vec<Either<Exp, Token>> = Vec::new();
        let tokens = exprTokens();
        // Try parsing none token values
        match self.parseNum() {
            Ok(e) => {
                acc.push(Either::Left(e));
                self._parseExp(&mut acc);
                return Ok(Exp::Expr(acc))
            },
            _ => (),
        };
        match self.parseId() {
            Ok(e) => {
                acc.push(Either::Left(e));
                match self.eat(Token::LParen) {
                    Ok(t) => {
                        acc.push(Either::Right(Token::LParen));
                        loop{
                            match self.parseExp() {
                                Ok(e) => {
                                    acc.push(Either::Left(e));
                                    match self.eat(Token::Comma) {
                                        Ok(_) => continue,
                                        _ => break,
                                    }
                                },
                                _ => break,
                            }
                        }
                        match self.eat(Token::RParen) {
                            Ok(_) => acc.push(Either::Right(Token::RParen)),
                            _ => panic!("unmatched opening bracket"),
                        }
                        self._parseExp(&mut acc);
                        return Ok(Exp::Expr(acc));
                    }
                    _ =>  {
                        self._parseExp(&mut acc);
                        return Ok(Exp::Expr(acc));
                    }
                }
            }
            _ => (),
        };
        match self.parseUnop() {
            Ok(e) => {
                acc.push(Either::Left(e));
                match self.parseExp() {
                    Ok(e) => acc.push(Either::Left(e)),
                    _ => ()
                }
                let _exp = self._parseExp(&mut acc);
                return Ok(Exp::Expr(acc));
            }
            _ => (),
        };

        for t in exprTokens() {
            match self.eat(t) {
                Ok(Token::Alloc) => {
                    acc.push(Either::Right(Token::Alloc));
                    match self.eat(Token::LParen) {
                        Ok(_) => {
                            match self.parseTp() {
                                Ok(t) => acc.push(Either::Left(t)),
                                _ => (),
                            };
                            match self.eat(Token::RParen){
                                Ok(_) => {
                                    self._parseExp(&mut acc);
                                    return Ok(Exp::Expr(acc));
                                }
                                _ => panic!("unmatched opening parentheses"),
                            }
                        }
                        _ => panic!("syntax error, alloc requires a declaration"),
                    }
                }
                Ok(Token::AllocArray) => {
                    acc.push(Either::Right(Token::AllocArray));
                    match self.eat(Token::RParen) {
                        Ok(_) => {
                            match self.parseTp() {
                                Ok(t) => {
                                    acc.push(Either::Left(t));
                                    match self.eat(Token::Comma) {
                                        Ok(_) => {
                                            match self.parseExp() {
                                                Ok(e) => {
                                                    acc.push(Either::Left(e));
                                                    match self.eat(Token::RParen) {
                                                        Ok(_) => {
                                                            self._parseExp(&mut acc);
                                                            return Ok(Exp::Expr(acc));
                                                        },
                                                        _ => panic!("Missing closing bracket"),
                                                    }
                                                }
                                                _ => panic!("expression required as second argument to alloc_array"),
                                            }
                                        }
                                        _ => panic!("expression required as second argument to alloc_array"),
                                    }
                                },
                                _ => (),
                            }
                        }
                        _ => panic!("syntax error, no opening bracket for alloc array"),
                    }
                }
                Ok(t) => {
                    acc.push(Either::Right(t));
                    self._parseExp(&mut acc);
                    return Ok(Exp::Expr(acc));
                },
                _ => (),
            }
        };
        Err(())
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    fn vecCheck<T: PartialEq>(v1: Vec<T>, v2: Vec<T>) {
        assert_eq!(v1.len(), v2.len());
        let count = v1
            .into_iter()
            .zip(v2)
            .filter(|(a, b)| {
                *a != *b
            }).count();
        assert_eq!(count, 0);
    }
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
        vecCheck(expectedResult, results);
    }

    #[test]
    fn parsingTp() {
        let mut parser = Parser::new(&mut String::from("./src/parser/tests/tp.txt"));
        let expectedResult: Vec<Result<Exp, ()>> = vec![
            Ok(Exp::Tp(vec![Token::Int])),
            Ok(Exp::Tp(vec![Token::Char])),
            Ok(Exp::Tp(vec![Token::Bool])),
            Ok(Exp::Tp(vec![Token::String])),
            Ok(Exp::Tp(vec![Token::Void])),
            Ok(Exp::Tp(vec![Token::Struct, Token::Undefined(Some('i')), Token::PointerDeref])),
            Ok(Exp::Tp(vec![Token::Undefined(Some('i')), Token::PointerDeref, Token::PointerDeref]))
        ];
        let mut results = Vec::new();
        loop {
            match parser.parseTp() {
                Ok(exp) =>
                    results.push(Ok(exp)),
                _ => break,
            }
        }
        vecCheck(results, expectedResult);
    }

}



