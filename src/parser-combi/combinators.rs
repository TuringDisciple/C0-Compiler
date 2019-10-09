use std::fs::File;
use std::io;
mod typeclass;
// TODO: functional parser combinator library
struct Parser<T> {
    parses: [(T, [u8])]
}

// TODO: ???
impl Functor for Parser {
    pub fn fmap(f: &Fn(A) -> B) -> Parser<B> {

    }
}

impl<T> Parser<T> {
    // TODO: Proper definiton of parse function
    pub fn parse(stream: &[u8]) -> Parser<T>{
        Parser{
            parses: []
        }
    }
}

fn parse_char(c: char) -> Parse<char> {
    
}


