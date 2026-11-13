use arbitru_core::{TicTacToe, count_outcomes, perft};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let args: Vec<&str> = args.iter().map(String::as_str).collect();
    match args.as_slice() {
        ["--version"] => println!("arbitru {}", env!("CARGO_PKG_VERSION")),
        ["perft", d] => match d.parse::<u32>() {
            Ok(d) => println!("{}", perft(&TicTacToe::new(), d)),
            Err(_) => usage(),
        },
        ["outcomes"] => {
            let (a, b, c) = count_outcomes(&TicTacToe::new());
            println!("first wins {a}, second wins {b}, draws {c}");
        }
        _ => usage(),
    }
}

fn usage() {
    eprintln!("usage: arbitru perft DEPTH | outcomes | --version");
    std::process::exit(2);
}
