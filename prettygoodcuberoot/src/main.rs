use std::io::{self, Read};

fn main() {
    let input = {
        let mut input = String::new();
        io::stdin().read_to_string(&mut input).unwrap();
        input.parse().unwrap()
    };
    let res = func(input);
    println!("{}", res);
}

type Int = u32;
const N: Int = 3;
type Digit = u8;

pub fn func(input: String) -> String {
    let mut digits = input
        .chars()
        .collect::<Vec<char>>()
        .rchunks(3)
        .map(|c| String::from_iter(c).parse::<Int>().unwrap())
        .rev()
        .chain(std::iter::once(0))
        .fold((Vec::<Digit>::new(), 0), |(mut found, remainder), next| {
            let current_value = 1000 * remainder + next;
            let part_found = found.as_slice();
            let calc_y = |_p: &[u8], x: u32| -> u32 {
                (0..N)
                    .map(|i| 10u32.pow(i) * pascal(N, i) * /*p.pow(i) **/ x.pow(N - i))
                    .sum()
            };
            let mut x = 0;
            let mut y = calc_y(part_found, x);
            loop {
                let temp_y = calc_y(part_found, x + 1);
                if temp_y > current_value {
                    break;
                }
                x += 1;
                y = temp_y;
            }
            found.push(Digit::try_from(x).unwrap());
            let remainder = current_value - y;
            dbg!((found, remainder))
        })
        .0;
    println!("{:?}", digits);
    if digits.pop().unwrap() >= 5 {
        round_up(&mut digits);
    }
    digits
        .into_iter()
        .skip_while(|&d| d == 0)
        .map(|d| d.to_string())
        .collect()
}

// fn long_mult(a: &[u8], b: &[u8]) -> Vec<u8> {
//     let res = vec![0; a.len() + b.len()];
//     for (i, a_i) in a.iter().enumerate() {
//         for (j, b_j) in b.iter().enumerate() {
//             let current = res[i+j] + a_i * 1 * (j < b.len() ? b[j] : 0) + carry;
//             c[i+j] = int (cur % base);
//             carry = int (cur / base);
//         }
//     }
//     while (c.size() > 1 && c.back() == 0)
//         c.pop_back();
// }

fn round_up(digits: &mut Vec<u8>) {
    let last = digits.pop().unwrap();
    if last == 9 {
        round_up(digits);
    }
    let next = (last + 1) % 10;
    digits.push(next);
}

pub fn pascal(n: Int, k: Int) -> Int {
    factorial(n)
        .checked_div(factorial(k).checked_mul(factorial(n - k)).unwrap())
        .unwrap()
}

fn factorial(i: Int) -> Int {
    (1..=i).product()
}

#[cfg(test)]
mod tests {
    use crate::{func, pascal};

    #[test]
    fn test_pascal() {
        assert_eq!(pascal(4, 1), 4);
    }

    #[test]
    fn test_example_0() {
        let input = "4192";
        let res = func(input.to_owned());
        println!("{}", res);
        assert_eq!(res, "16");
    }

    #[test]
    fn test_example_1() {
        let input = "64";
        let res = func(input.to_owned());
        println!("{}", res);
        assert_eq!(res, "4");
    }

    #[test]
    fn test_example_2() {
        let input = "472741006443";
        let res = func(input.to_owned());
        println!("{}", res);
        assert_eq!(res, "7790");
    }

    #[test]
    fn test_example_3() {
        let input = "65991621053219768206433";
        let res = func(input.to_owned());
        println!("{}", res);
        assert_eq!(res, "40410690");
    }
}
