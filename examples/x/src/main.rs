use xaoc_proc::*;

xaoc! {
    year = 2021;
    day = 12;
}

// TODO
// xaoc_example! {
//     input = "0,0,0,0,0";
//     1: output = 0;
//     2: output = 5;
// }

xaoc_types! {
    type Input = Vec<i32>;
    type Output = i32;
}

#[xaoc_input]
fn input(s: &str) -> Input {
    s.split(',').map(|s| s.parse::<i32>().unwrap()).collect()
}

#[xaoc_solver(part = 1)]
fn solver(input: &Input) -> Output1 {
    input.iter().sum()
}

#[xaoc_solver(part = 2)]
fn solver_2(input: &Input) -> Output2 {
    input.iter().filter(|&&it| it == 0).count().try_into().unwrap()
}

fn main() {} // TODO
