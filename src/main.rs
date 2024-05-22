use lexer::Lexer;

mod lexer;
mod parser;
mod token;

fn main() {
    let s = "😄 久标准 Hello Ëveryone \r\n go fish";

    let mut l = Lexer::new(s);

    // let t = l.next_token();
    // let t = l.next_token();
    // let t = l.next_token();
    // let t = l.next_token();

    // println!("{:?}", t);

    println!(
        "{:?}",
        parser::Precedence::Lowest < parser::Precedence::LessGreater
    );

    for token in l {
        println!("{:?}", token);
    }
}
