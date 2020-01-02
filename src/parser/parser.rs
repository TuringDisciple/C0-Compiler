#![allow(non_snake_case)]
use lexer::lexer::*;
use std::vec::Vec;
use std::result;
use std::collections::VecDeque;

/*
The purpose of the parser is to apply semantic meaning to our language lexemes.
Our parser must therefore perform the following functions

1. Construct semantically correct structures from lexems following the language grammar
2. Identify syntactic errors in source


Fortunately there is a formal grammar specification
for the C0 language. The problems with this grammar is that
it's left recursive which will introduce complications with parsing down the line.
 */
#[derive(Clone)]
enum Exp{
    Id(Vec<Token>),
    Num(u32),
}

#[derive(Clone)]
pub struct Parser {
    tokens: VecDeque<Token>,
    head: Option<Token>,
}
impl Parser {
    pub fn new(file_path: &mut String) -> Parser {
        let mut tokens = Lexer::new(file_path).tokens();
        let head = tokens.pop_front();
        Parser {
            tokens,
            head,
        }
    }

    fn eat(&mut self, t: Token) -> Result<Token, ()>{
        match self.head {
            Some(h) =>{
                match (h, t) {
                    (Token::Undefined(_), Token::Undefined(_)) |
                    (Token::Num(_), Token::Num(_)) => {
                        self.head = self.tokens.pop_front();
                        Ok(h)
                    },
                    (_, _) => {
                        if h == t {
                            self.head = self.tokens.pop_front();
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
    fn parseId(&mut self) -> Result<Exp, ()>{
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
    fn parseNum(&mut self) -> Result<Exp, ()>{
        match self.eat(Token::Num(0)) {
            Ok(Token::Num(x)) => Ok(Exp::Num(x)),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsingId() {
    }

    #[test]
    fn parsingNumbers(){
    }
}



