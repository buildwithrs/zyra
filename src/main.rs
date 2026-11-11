use zyra::{Lexer, parser::Parser};

fn main() -> anyhow::Result<()> {
    println!("Write an simple programming language: Zyra in Rust");

    let mut editor = rustyline::DefaultEditor::new()?;
    let prompt = format!("> ");
    loop {
        let line = editor.readline(&prompt)?;
        println!("read line: {}", line);

        let chars = line.chars().collect::<Vec<char>>();
        let mut lexer = Lexer::new(chars);
        match lexer.lex() {
            Ok(tokens) => {
                println!("{:?}", tokens);
                let mut parser = Parser::new(tokens);
                match parser.parse() {
                    Ok(program) => println!("{:?}", program),
                    Err(e) => eprintln!("parser error: {}", e),
                }
            }
            Err(e) => eprintln!("lexer error: {e}"),
        }
    }
}
