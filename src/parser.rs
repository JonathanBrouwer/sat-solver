use std::fmt::{Display, Formatter};
use itertools::Itertools;

pub enum SatResult {
    Unknown,
    Satisfiable(Vec<isize>),
    Unsatisfiable,
}

impl Display for SatResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SatResult::Unknown => writeln!(f, "s UNKNOWN"),
            SatResult::Unsatisfiable => writeln!(f, "s UNSATISFIABLE"),
            SatResult::Satisfiable(sol) => {
                writeln!(f, "s SATISFIABLE")?;
                writeln!(f, "{}", sol.iter().format(" "))
            }
        }
    }
}

pub struct SatProblem {
    pub var_count: usize,
    pub clauses_count: usize,
    pub clauses: Vec<Vec<isize>>,
}

pub fn parse_sat_file(f: &str) -> SatProblem {
    let mut sat = SatProblem {
        var_count: 0,
        clauses_count: 0,
        clauses: vec![],
    };
    let mut lines = f.lines().peekable();

    //Skip comments
    while let Some(line) = lines.peek() {
        if !line.starts_with("c") {
            break;
        }
        lines.next().unwrap();
    }
    let header = lines.next().unwrap();
    let header = header.split_whitespace().collect::<Vec<_>>();
    assert_eq!(header[0], "p");
    assert_eq!(header[1], "cnf");
    sat.var_count = header[2].parse().unwrap();
    sat.clauses_count = header[3].parse().unwrap();

    for line in lines {
        let mut line: Vec<isize> = line
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect::<Vec<_>>();
        assert_eq!(line.pop().unwrap(), 0);
        sat.clauses.push(line);
    }

    sat
}
