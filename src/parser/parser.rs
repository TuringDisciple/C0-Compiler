use lexer::lexer::*;

pub struct Parser {
      lexer: Lexer,
}

impl Parser {
      pub fn new(filePath: &mut String) -> Parser {
            Parser {
                  lexer: Lexer::new(filePath), 
            }
      }

      pub fn parse() {
            
      }
}

pub fn Parse()


#[cfg(test)]
mod test {
      use lexer::lexer::*;

}