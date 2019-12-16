use lexer::lexer::*;

#[derive(Clone)]
pub struct Parser {
    lexer: Lexer,
}

impl Parser {
    pub fn new(file_path: &mut String) -> Parser {
        Parser {
            lexer: Lexer::new(file_path),
        }
    }

    pub fn lexer(self) -> Lexer {
        self.lexer
    }
}
