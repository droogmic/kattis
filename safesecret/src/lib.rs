use std::{
    collections::{hash_map::DefaultHasher, HashMap, VecDeque},
    hash::{Hash, Hasher},
    ops::Add,
};

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
enum Token {
    Add,
    Sub,
    Mul,
    Question,
    Val(isize),
}

#[derive(Clone, Copy, Debug)]
enum Op {
    Val(isize),
    Fork([isize; 3]),
}

impl Token {
    fn op(&self, left: &Token, right: &Token) -> Op {
        let left = if let Token::Val(v) = left {
            v
        } else {
            unreachable!()
        };
        let right = if let Token::Val(v) = right {
            v
        } else {
            unreachable!()
        };
        match self {
            Token::Add => Op::Val(left + right),
            Token::Sub => Op::Val(left - right),
            Token::Mul => Op::Val(left * right),
            Token::Question => Op::Fork([left + right, left - right, left * right]),
            _ => unreachable!(),
        }
    }
    fn min_max(&self, left: &Token, right: &Token) -> (isize, isize) {
        match self.op(left, right) {
            Op::Val(val) => return (val, val),
            Op::Fork(vals) => return (*vals.iter().min().unwrap(), *vals.iter().max().unwrap()),
        }
    }
}

impl std::str::FromStr for Token {
    type Err = std::num::ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "+" => Self::Add,
            "-" => Self::Sub,
            "*" => Self::Mul,
            "?" => Self::Question,
            v => Self::Val(v.parse()?),
        })
    }
}

struct Memoization(HashMap<u64, (isize, isize)>);

pub fn safe_secret(input: String) -> String {
    // Parsing
    let mut lines = input.lines();
    let pair_count: usize = lines.next().unwrap().parse().unwrap();
    let mut token_ring: VecDeque<Token> = lines
        .next()
        .unwrap()
        .split(' ')
        .map(|t| t.parse().unwrap())
        .collect();
    assert_eq!(token_ring.len(), 2 * pair_count);

    // Memoization
    let mut mem = Memoization::new();

    // Calc
    (0..pair_count)
        .map(|_| {
            let (min, max) = mem.reduce(
                token_ring
                    .iter()
                    .cloned()
                    .take(token_ring.len() - 1)
                    .collect::<Vec<_>>()
                    .as_slice(),
            );
            token_ring.rotate_left(2);
            min.abs().to_string().add(&max.abs().to_string())
        })
        .collect::<Vec<_>>()
        .as_slice()
        .join("")
}

impl Memoization {
    fn new() -> Self {
        Self(HashMap::new())
    }
    fn reduce(&mut self, expression: &[Token]) -> (isize, isize) {
        let mut hasher = DefaultHasher::new();
        expression.hash(&mut hasher);
        let hash = hasher.finish();

        let min_max = self.0.get(&hash);
        if let Some(min_max) = min_max {
            return *min_max;
        }
        assert!(expression.len() >= 3);
        if expression.len() == 3 {
            return expression[1].min_max(&expression[0], &expression[2]);
        }
        let min_max = (0..(expression.len() / 2)).fold(
            (std::isize::MAX, std::isize::MIN),
            |best_min_max, op_idx| {
                let op_idx = 2 * op_idx + 1;
                let mut recurse_min_max = |val: isize| -> (isize, isize) {
                    self.reduce(
                        expression
                            .iter()
                            .take(op_idx - 1)
                            .cloned()
                            .chain(std::iter::once(Token::Val(val)))
                            .chain(expression.iter().skip(op_idx + 2).cloned())
                            .collect::<Vec<_>>()
                            .as_slice(),
                    )
                };
                match expression[op_idx].op(&expression[op_idx - 1], &expression[op_idx + 1]) {
                    Op::Val(val) => {
                        let min_max = recurse_min_max(val);
                        get_min_max(best_min_max, min_max)
                    }
                    Op::Fork(vals) => {
                        let min_max = vals.iter().fold(
                            (std::isize::MAX, std::isize::MIN),
                            |best_min_max, val| {
                                let min_max = recurse_min_max(*val);
                                get_min_max(best_min_max, min_max)
                            },
                        );
                        get_min_max(best_min_max, min_max)
                    }
                }
            },
        );
        self.0.insert(hash, min_max);
        min_max
    }
}

fn get_min_max(lhs: (isize, isize), rhs: (isize, isize)) -> (isize, isize) {
    (lhs.0.min(rhs.0), lhs.1.max(rhs.1))
}

#[cfg(test)]
mod tests {
    use crate::{safe_secret, Memoization, Token};

    #[test]
    fn test_example() {
        let input = "5\n1 ? 5 + 0 ? -2 - -3 *";
        let res = safe_secret(input.to_owned());
        println!("{}", res);
        assert_eq!(res, "914710203014163336");
    }

    #[test]
    fn test_recursive_reduce() {
        let input = vec![
            Token::Val(1),
            Token::Add,
            Token::Val(2),
            Token::Mul,
            Token::Val(3),
        ];
        let mut mem = Memoization::new();
        assert_eq!(mem.reduce(input.as_slice()), (7, 9));
    }
}
