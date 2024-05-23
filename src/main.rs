use lexer::Lexer;

pub mod ast;
mod lexer;
mod parser;
mod token;

fn main() {
    let s = "😄 久标准 Hello Ëveryone \r\n go fish";

    let l = Lexer::new(s);
    let mut p = parser::Parser::new(l);
    let pr = p.parse_program();

    // let t = l.next_token();
    // let t = l.next_token();
    // let t = l.next_token();
    // let t = l.next_token();

    // println!("{:?}", t);
    println!(
        "{:?}",
        parser::Precedence::Lowest < parser::Precedence::LessGreater
    );
}
