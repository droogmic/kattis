use std::{
    cmp::{
        max,
        Ordering::{Equal, Greater, Less},
    },
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
            if let Some(best_path) = search(&distances) {
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
pub fn search(stages: &[usize]) -> Option<Vec<usize>> {
    search_rec(0, stages).map(|(_, mut p)| {
        p.pop();
        p.reverse();
        p
    })
}
/// Returns an inverted vector, first in vec is the last distance
fn search_rec(start: usize, stages: &[usize]) -> Option<(usize, Vec<usize>)> {
    match (stages.first(), stages.get(1..stages.len())) {
        (Some(&next_stage), Some([])) => {
            if start == next_stage {
                Some((start, vec![start]))
            } else {
                None
            }
        }
        (Some(&next_stage), Some(next_stages)) => {
            let down_path = if let Some(down_next) = start.checked_sub(next_stage) {
                search_rec(down_next, next_stages)
            } else {
                None
            };
            let up_path = search_rec(start + next_stage, next_stages);
            let (prev_max, mut p) = match (down_path, up_path) {
                (None, None) => None,
                (Some(path), None) => Some(path),
                (None, Some(path)) => Some(path),
                (Some(down), Some(up)) => match down.0.partial_cmp(&up.0) {
                    Some(Less) => Some(down),
                    Some(Greater) => Some(up),
                    Some(Equal) => Some(down), // pick any
                    None => panic!(),
                },
            }?;
            p.push(start);
            Some((max(start, prev_max), p))
        }
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use crate::{func, search};

    #[test]
    fn test_search() {
        assert_eq!(search(&[10, 10]), Some(vec![10]));
        assert_eq!(search(&[10, 10, 10, 10]), Some(vec![10, 0, 10]));
        assert_eq!(search(&[3, 4, 2, 1, 6, 4, 5]), None);
    }

    #[test]
    fn test_example_1() {
        let input = include_str!("example_in.txt");
        let res = func(input.to_owned());
        println!("{}", res);
        assert_eq!(res, include_str!("example_out.txt"));
    }
}
