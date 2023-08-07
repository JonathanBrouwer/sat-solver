use crate::parser::{SatProblem, SatResult};

pub struct BruteForceSolver {
    problem: SatProblem,
}

impl BruteForceSolver {
    pub fn new(problem: SatProblem) -> BruteForceSolver {
        BruteForceSolver { problem }
    }

    pub fn solve(&mut self) -> SatResult {
        let mut vars = vec![false; self.problem.var_count];

        loop {
            if self.is_sat(&vars) {
                let solution = vars
                    .iter()
                    .enumerate()
                    .map(|(i, &x)| if x { i as isize + 1 } else { -(i as isize + 1) })
                    .collect();
                return SatResult::Satisfiable(solution);
            }
            // Overflowed all options
            if increment(&mut vars) {
                return SatResult::Unsatisfiable;
            }
        }
    }

    fn is_sat(&self, vars: &[bool]) -> bool {
        self.problem
            .clauses
            .iter()
            .all(|clause| clause.iter().any(|&c| vars[c.abs() as usize - 1] ^ (c < 0)))
    }
}

/// returns true if overflows
fn increment(vars: &mut [bool]) -> bool {
    for i in (0..vars.len()).rev() {
        vars[i] = !vars[i];
        if vars[i] {
            return false;
        }
    }
    true
}
