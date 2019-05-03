mod lexer;

fn main() {
    println!("Hello, world!");
    let file_path: String = "~/src/test/exp.c0".to_string();
    let lexer = lexer::lexer::Lexer::new(file_path);
}
