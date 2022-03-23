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

#[cfg(test)]
mod tests {
    use crate::func;

    #[test]
    fn test_example_1() {
        let input = include_str!("example_in.txt");
        let res = func(input.to_owned());
        println!("{}", res);
        assert_eq!(res, include_str!("example_out.txt"));
    }
}
