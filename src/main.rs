use gameaboutsquares_solver::*;
use std::{env};

fn main() -> Result<(),i32> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("usage: {} <puzzle input>", args[0]);
        return Err(1);
    }
    let g = SquaresParser::parse_file(&args[1]);
    debug_print(&g.state, &g.data);
    let result = Solver::solve(g);
    match result {
        Some(path) => println!("{:?}", path),
        None => println!("No solution found!"),
    }
    Ok(())
}
