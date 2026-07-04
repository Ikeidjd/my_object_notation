mod token;
mod lexer;

fn main() {
    unsafe { std::env::set_var("RUST_BACKTRACE", "1") };

    let source_code = match std::fs::read_to_string(&std::env::args().collect::<Vec<_>>()[1]) {
        Ok(source_code) => source_code,
        Err(error) => {
            println!("{error}");
            return;
        }
    };
    
    let tokens = match lexer::lex(&source_code) {
        Ok(tokens) => tokens,
        Err(errors) => {
            for error in errors {
                println!("{error}");
            }

            return;
        }
    };

    for token in tokens {
        println!("{token:?}");
    }
}
