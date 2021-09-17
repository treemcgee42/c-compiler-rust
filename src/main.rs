mod defs;
mod lexer;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    scan_file(args[1].trim());
}

// ---- Loop scanning in all the tokens in the input file ----
// ---- Print out details of each token found. ---------------
fn scan_file(file_path: &str) {
    let file_as_str = std::fs::read_to_string(file_path).unwrap();
    let mut cursor = lexer::Cursor::from_str(&file_as_str);

    loop {
        match cursor.get_next_token().kind {
            defs::TokenKind::EOF => { break; }
            defs::TokenKind::TIntLit(n) => {
                println!("Token TIntLit, value {}", n);
            }
            tk => {
                println!("Token {:?}", tk);
            }
        }
    }
}