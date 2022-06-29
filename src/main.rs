use gameaboutsquares_solver::*;
use std::env;

fn main() -> Result<(),i32> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("usage: {} <puzzle input>", args[0]);
        return Err(1);
    }
    let g = SquaresParser::parse_file(&args[1]);
    g.debug_print();
    return Ok(());
}
