use std::io::{self, Read};

type Float = f64;

fn main() {
    let input = {
        let mut input = String::new();
        io::stdin().read_to_string(&mut input).unwrap();
        input
    };
    let res = func(input);
    println!("{}", res);
}

pub fn func(input: String) -> Float {
    let mut lines = input.lines();
    let (troop_type_num, budget): (usize, usize) = {
        let mut split = lines.next().unwrap().split_whitespace();
        (
            split.next().unwrap().parse().unwrap(),
            split.next().unwrap().parse().unwrap(),
        )
    };
    let troop_types: Vec<[Float; 2]> = lines
        .map(|l| {
            let mut split = l.split_whitespace();
            let cost: usize = split.next().unwrap().parse().unwrap();
            let health: Float = split.next().unwrap().parse().unwrap();
            let potency: Float = split.next().unwrap().parse().unwrap();
            let cost = cost as Float;
            [health / cost, potency / cost]
        })
        .collect();
    assert_eq!(troop_types.len(), troop_type_num);
    let max_efficacy = troop_types
        .iter()
        .enumerate()
        .map(|(idx, a)| {
            troop_types
                .iter()
                .skip(idx + 1)
                .map(move |b| max_efficacy(a, b))
        })
        .flatten()
        .filter(|v| v.is_normal())
        .max_by(|&left, &right| left.partial_cmp(&right).unwrap())
        .unwrap();
    let budget = budget as Float;
    budget * budget * max_efficacy
}

/// Maximising mixture of 2 things...
/// mix a and b in ratio of n to 1-n
pub fn max_mix(a: &[Float; 2], b: &[Float; 2]) -> Float {
    let delta_0 = a[0] - b[0];
    let delta_1 = a[1] - b[1];
    let numerator = b[0] * delta_1 + b[1] * delta_0;
    let denominator = delta_0 * delta_1;
    let n = -0.5 * numerator / denominator;
    n
}

fn max_efficacy(a: &[Float; 2], b: &[Float; 2]) -> Float {
    let n = max_mix(a, b);
    (n * a[0] + (1.0 - n) * b[0]) * (n * a[1] + (1.0 - n) * b[1])
}

#[cfg(test)]
mod tests {
    use crate::{func, max_mix};

    #[test]
    fn test_max_mix() {
        assert!(max_mix(&[1.0, 0.1], &[0.1, 1.0]) - 0.5 < 0.0001);
    }

    #[test]
    fn test_example_1() {
        let input = include_str!("example_in_1.txt");
        let res = func(input.to_owned());
        println!("{}", res);
        assert!(res - 19436.05 < 0.005);
    }

    #[test]
    fn test_example_2() {
        let input = include_str!("example_in_2.txt");
        let res = func(input.to_owned());
        println!("{}", res);
        assert!(res - 3025.00 < 0.005);
    }
}
