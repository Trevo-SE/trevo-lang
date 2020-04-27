#[macro_use] extern crate lazy_static;

mod lexer;
mod error;

use lexer::lexer;
use error::*;

fn main() -> TRResult<()> {
    let toks = lexer("res/simple_example.tr")?;

    println!("Tokens: [");
    for t in toks {
        println!("\t{:?}", t);
    }
    println!("]");

    Ok(())
}
