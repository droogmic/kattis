use std::{
    collections::VecDeque,
    io::{self, Read},
};

fn main() {
    let input = {
        let mut input = String::new();
        io::stdin().read_to_string(&mut input).unwrap();
        input
    };
    let res = sim(input);
    println!("{}", res);
}

pub fn sim(input: String) -> String {
    let mut lines = input.lines();
    let num_test_cases: usize = lines.next().unwrap().parse().unwrap();
    let test_cases: Vec<String> = lines
        .into_iter()
        .map(|l| {
            l.chars()
                .fold((VecDeque::new(), None), |(mut acc, idx), c| match c {
                    ']' => (acc, None),
                    '[' => (acc, Some(0_usize)),
                    '<' => match idx {
                        Some(0) => (acc, Some(0)),
                        None => {
                            acc.pop_back();
                            (acc, None)
                        }
                        Some(i) => {
                            acc.remove(i - 1);
                            (acc, Some(i - 1))
                        }
                    },
                    c => match idx {
                        Some(0) => {
                            acc.push_front(c);
                            (acc, Some(1))
                        }
                        None => {
                            acc.push_back(c);
                            (acc, None)
                        }
                        Some(i) => {
                            acc.insert(i, c);
                            (acc, Some(i + 1))
                        }
                    },
                })
                .0
                .into_iter()
                .collect()
        })
        .collect();
    assert_eq!(test_cases.len(), num_test_cases);
    test_cases.join("\n")
}

#[cfg(test)]
mod tests {
    use crate::sim;

    #[test]
    fn test_example_1() {
        let input = "1\nmy ]]name]] is]] steva<en]<n halim]]]]]";
        let res = sim(input.to_owned());
        println!("{}", res);
        assert_eq!(res, "my name is steven halim");
    }

    #[test]
    fn test_example_2() {
        let input = "1\n<<hate<<<<loves[steva<en ] cs2040c< and also cs2040c";
        let res = sim(input.to_owned());
        println!("{}", res);
        assert_eq!(res, "steven loves cs2040 and also cs2040c");
    }

    #[test]
    fn test_example_3() {
        let input = "3\nfoo\nbar\nabc<<<<[abc";
        let res = sim(input.to_owned());
        println!("{}", res);
        assert_eq!(res, "foo\nbar\nabc");
    }

    #[test]
    fn test_example_4() {
        let input = "1\ndef[abc]ghi<<<<<<";
        let res = sim(input.to_owned());
        println!("{}", res);
        assert_eq!(res, "abc");
    }

    #[test]
    fn test_example_5() {
        let input = "1\ndef[abc]ghi<<<<<<<<<<<<<[abc";
        let res = sim(input.to_owned());
        println!("{}", res);
        assert_eq!(res, "abc");
    }
}
