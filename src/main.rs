use std::rc::Rc;

mod text;
mod mon_error;
mod token;
mod lexer;
mod mon_object;
mod parser;

fn main() {
    unsafe { std::env::set_var("RUST_BACKTRACE", "1") };

    let source_code: Rc<[char]> = match std::fs::read_to_string(&std::env::args().collect::<Vec<_>>()[1]) {
        Ok(source_code) => Rc::from(source_code.chars().collect::<Vec<_>>()),
        Err(error) => {
            println!("{error}");
            return;
        }
    };
    
    let tokens = match lexer::lex(source_code) {
        Ok(tokens) => tokens,
        Err(errors) => {
            for error in errors {
                println!("{error}");
            }

            return;
        }
    };

    for token in &tokens {
        println!("{token:?}");
    }

    let object = match parser::parse(tokens) {
        Ok(object) => object,
        Err(error) => {
            println!("{error}");
            return;
        }
    };

    println!("{object}");
}
