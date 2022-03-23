use std::{
    cmp::Ordering::{Equal, Greater, Less},
    io::{self, Read},
};

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
    let num_test_scenarios: usize = lines.next().unwrap().parse().unwrap();
    (0..num_test_scenarios)
        .map(|_| {
            let _num_distances: usize = lines.next().unwrap().parse().unwrap();
            lines
                .next()
                .unwrap()
                .split_whitespace()
                .map(|d| d.parse().unwrap())
                .collect::<Vec<usize>>()
        })
        .map(|distances| {
            let paths = search(&distances);
            if let Some(best_path) = paths
                .into_iter()
                .min_by_key(|path| *path.iter().max().unwrap())
            {
                std::iter::once("U")
                    .chain(best_path.windows(2).map(|w| match w[0].partial_cmp(&w[1]) {
                        Some(Less) => "U",
                        Some(Greater) => "D",
                        Some(Equal) => panic!(),
                        None => panic!(),
                    }))
                    .chain(std::iter::once("D"))
                    .collect::<String>()
            } else {
                "IMPOSSIBLE".to_owned()
            }
        })
        .collect::<Vec<_>>()
        .as_slice()
        .join("\n")
}

/// Returns paths
pub fn search(stages: &[usize]) -> Vec<Vec<usize>> {
    search_rec(0, stages)
        .into_iter()
        .map(|mut v| {
            v.reverse();
            v
        })
        .collect()
}
/// Returns an inverted vector, first in vec is the last distance
fn search_rec(start: usize, stages: &[usize]) -> Vec<Vec<usize>> {
    match (stages.first(), stages.get(1..stages.len())) {
        (Some(&next_stage), Some([])) => {
            if start == next_stage {
                vec![vec![]]
            } else {
                vec![]
            }
        }
        (Some(&next_stage), Some(next_stages)) => {
            let mut paths = Vec::new();
            if next_stage <= start {
                let next = start - next_stage;
                let down_paths = search_rec(next, next_stages);
                for mut down_path in down_paths {
                    down_path.push(next);
                    paths.push(down_path);
                }
            }
            if start {
                let next = start + next_stage;
                let up_paths = search_rec(next, next_stages);
                for mut up_path in up_paths {
                    up_path.push(next);
                    paths.push(up_path);
                }
            }
            paths
        }
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use crate::{func, search};

    #[test]
    fn test_search() {
        assert_eq!(search(&[10, 10]), [[10]]);
        assert_eq!(search(&[10, 10, 10, 10]), [[10, 0, 10], [10, 20, 10]]);
    }

    #[test]
    fn test_example_1() {
        let input = include_str!("example_in.txt");
        let res = func(input.to_owned());
        println!("{}", res);
        assert_eq!(res, include_str!("example_out.txt"));
    }
}
