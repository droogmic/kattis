use std::io::{self, Read};

fn main() {
    let input = {
        let mut input = String::new();
        io::stdin().read_to_string(&mut input).unwrap();
        input
    };
    let res = hyper_pyramids(input);
    println!("{}", res);
}

pub fn hyper_pyramids(input: String) -> String {
    // Parsing
    let mut input = input.split(" ");
    let d: usize = input.next().unwrap().parse().unwrap();
    let h: usize = input.next().unwrap().parse().unwrap();

    return "foo".to_owned();
}

// Get base of pascal triangle, sampled at n, valid for 1 <= n <= h, 1 <= z <= h
pub const fn pascal_base(n: usize, h: usize, z: usize) -> usize {
    let nn = n + z - 1;
    pascal_number(n, z - 1) * pascal_widest_base(nn, h)
}

// Get widest base of pascal triangle, sampled at n, valid for 1 <= n <= h
pub const fn pascal_widest_base(n: usize, h: usize) -> usize {
    pascal_number(h - n + 1, n - 1)
}

// Triangular Number in Higher Dimension
pub const fn pascal_number(n: usize, d: usize) -> usize {
    if d == 0 {
        return 1;
    }
    choose(n + (d - 1), d)
}

// Binomial
pub const fn choose(n: usize, k: usize) -> usize {
    if k == 0 {
        return 1;
    }
    (n * choose(n - 1, k - 1)) / k
}

#[cfg(test)]
mod tests {
    use crate::{hyper_pyramids, pascal_base, pascal_number, pascal_widest_base};

    #[test]
    fn test_example() {
        let input = "3 5";
        let res = hyper_pyramids(input.to_owned());
        println!("{}", res);
        assert_eq!(res, "1\n4\n6\n12");
    }

    #[test]
    fn test_pascal_number() {
        assert_eq!(pascal_number(1, 0), 1);
        assert_eq!(pascal_number(2, 0), 1);
        assert_eq!(pascal_number(1, 1), 1);
        assert_eq!(pascal_number(2, 1), 2);
        assert_eq!(pascal_number(1, 2), 1);
        assert_eq!(pascal_number(2, 2), 3);
        assert_eq!(pascal_number(3, 2), 6);
        assert_eq!(pascal_number(2, 3), 4);
        assert_eq!(pascal_number(3, 3), 10);
    }

    #[test]
    fn test_pascal_widest_base() {
        assert_eq!(pascal_widest_base(1, 5), 1);
        assert_eq!(pascal_widest_base(2, 5), 4);
        assert_eq!(pascal_widest_base(3, 5), 6);
        assert_eq!(pascal_widest_base(4, 5), 4);
        assert_eq!(pascal_widest_base(5, 5), 1);
    }

    #[test]
    fn test_pascal_base() {
        assert_eq!(pascal_base(1, 5, 1), 1);
        assert_eq!(pascal_base(2, 5, 1), 4);
        assert_eq!(pascal_base(3, 5, 1), 6);
        assert_eq!(pascal_base(4, 5, 1), 4);
        assert_eq!(pascal_base(5, 5, 1), 1);
        assert_eq!(pascal_base(1, 5, 2), 4);
        assert_eq!(pascal_base(2, 5, 2), 12);
        assert_eq!(pascal_base(3, 5, 2), 12);
        assert_eq!(pascal_base(4, 5, 2), 4);
        assert_eq!(pascal_base(1, 5, 3), 6);
        assert_eq!(pascal_base(2, 5, 3), 12);
        assert_eq!(pascal_base(3, 5, 3), 6);
    }
}
