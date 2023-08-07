use crate::brute_force::BruteForceSolver;
use crate::parser::parse_sat_file;

mod brute_force;
mod parser;

fn main() {
    let sat = parse_sat_file(include_str!("../resources/simple_v3_c2.cnf"));
    let mut solver = BruteForceSolver::new(sat);
    let res = solver.solve();
    println!("{}", res);
}
