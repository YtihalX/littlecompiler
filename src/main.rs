use compiler::lexer::*;
fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("error usage!");
        std::process::exit(1);
    }
    let source = std::fs::read(&args[1]).unwrap();
    let lexed = lex(&source);
    for token in lexed {
        print!("{:?} ", token);
    }
}

