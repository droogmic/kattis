use std::io::{self, Read};

fn main() {
    let input = {
        let mut input = String::new();
        io::stdin().read_to_string(&mut input).unwrap();
        input
    };
    let res = func(input);
    println!("{}", res);
}

pub fn func(input: String) -> String {
    let mut lines = input.lines();
    todo!();
}

/// Maximising mixture of 2 things...
/// mix a and b in ration of n to 1-n
fn max_mix(a: (f32, f32), b: (f32, f32)) -> f32 {
    let delta_0 = a.0 - b.0;
    let delta_1 = a.1 - b.1;
    let numerator = b.0 * delta_1 + b.1 * delta_0;
    let denominator = delta_0 * delta_1;
    let n = -0.5 * numerator / denominator;
    n
}

#[cfg(test)]
mod tests {
    use crate::func;

    #[test]
    fn test_example_1() {
        let input = include_str!("example_in_1.txt");
        let res = func(input.to_owned());
        println!("{}", res);
        assert_eq!(res, include_str!("example_out_1.txt"));
    }

    #[test]
    fn test_example_2() {
        let input = include_str!("example_in_2.txt");
        let res = func(input.to_owned());
        println!("{}", res);
        assert_eq!(res, include_str!("example_out_2.txt"));
    }
}
