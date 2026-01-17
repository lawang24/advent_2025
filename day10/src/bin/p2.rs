use good_lp::{
    Expression, ProblemVariables, Solution, SolverModel, Variable, constraint, microlp, variable,
};
use std::fs;

fn joltage_dp(buttons: &Vec<Vec<&str>>, joltages: &Vec<u64>) -> f64 {
    // n is num buttons
    // m is num constraints (machines)
    // full data column should be m x n 2d array, each row_i representing constraint[i]
    // if button[j] influences machine[i], then data[i][j] == 1 else 0
    let n = buttons.len();
    let m = joltages.len();
    let mut linear_program_data = Vec::new();

    for i in 0..m {
        let mut new_row = Vec::new();
        let val = i.to_string();
        for button in buttons {
            if (button).contains(&val.as_str()) {
                new_row.push(1.0);
            } else {
                new_row.push(0.0);
            }
        }
        linear_program_data.push(new_row);
    }

    let mut problem = ProblemVariables::new();
    let x: Vec<Variable> = problem.add_vector(variable().integer().min(0), n);
    let objective: Expression = x.iter().sum();
    let mut model = problem.minimise(objective).using(microlp);

    for (i, row) in linear_program_data.iter().enumerate() {
        let expr = x
            .iter()
            .zip(row.iter()) // create tuples of val, coeff
            .fold(0.0.into(), |acc: Expression, (&v, &c)| acc + c * v);

        model = model.with(constraint!(expr == joltages[i] as f64));
    }

    let solution = model.solve().unwrap();
    let mut answer = 0_f64;
    for v in x {
        answer += solution.value(v);
    }
    println!("{answer}");
    answer
}

fn buttons_encoder(buttons: &str) -> Vec<&str> {
    let buttons: Vec<&str> = buttons[1..buttons.len() - 1].split(',').collect();
    buttons
}

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let mut answer = 0_f64;
    for line in contents.lines() {
        let mut items = line.split_whitespace();
        let _ = items.next().unwrap();
        let joltages = items.next_back().unwrap();
        let joltages: Vec<u64> = joltages[1..joltages.len() - 1]
            .split(',')
            .map(|x| x.parse().unwrap())
            .collect();

        let buttons: Vec<&str> = items.collect();
        let button_encoded: Vec<Vec<&str>> = buttons.iter().map(|x| buttons_encoder(x)).collect();
        answer += joltage_dp(&button_encoded, &joltages);
    }

    println!("{answer}")
}
