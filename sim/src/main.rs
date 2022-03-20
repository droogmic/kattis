use std::io::{self, Read};

fn main() {
    let input = {
        let mut input = String::new();
        io::stdin().read_to_string(&mut input).unwrap();
        input
    };
    let res = sim(input);
    println!("{}", res);
}

enum Char {
    Back,
    Home,
    End,
    Other(char),
}

impl Char {
    fn from(c: char) -> Self {
        match c {
            '<' => Char::Back,
            ']' => Char::End,
            '[' => Char::Home,
            c => Char::Other(c),
        }
    }
}

pub fn sim(input: String) -> String {
    // Parsing
    let mut lines = input.lines();
    let num_test_cases: usize = lines.next().unwrap().parse().unwrap();
    let test_cases: Vec<String> = lines.into_iter().map(|l| l.to_owned()).collect();
    assert_eq!(test_cases.len(), num_test_cases);

    // Do
    test_cases
        .into_iter()
        .map(|t| process(t))
        .collect::<Vec<_>>()
        .as_slice()
        .join("\n")
}

enum CharOrBackspace {
    Backspace,
    Char(char),
}
impl std::fmt::Display for CharOrBackspace {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Backspace => '<',
                Self::Char(c) => *c,
            }
        )
    }
}
impl std::fmt::Debug for CharOrBackspace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(PartialEq)]
enum PushSide {
    Front,
    Back,
}

fn process(line: String) -> String {
    let ((mut front_parts, mut back_parts), last, side) = line.chars().map(Char::from).fold(
        (
            (Vec::<char>::new(), Vec::<CharOrBackspace>::new()),
            Vec::<CharOrBackspace>::new(),
            PushSide::Front,
        ),
        |((mut front_parts, mut back_parts), mut part, side), c| match c {
            Char::Back => {
                // Skip backspaces at the front
                if !part.is_empty() || side == PushSide::Back {
                    part.push(CharOrBackspace::Backspace);
                }
                ((front_parts, back_parts), part, side)
            }
            Char::Other(c) => {
                part.push(CharOrBackspace::Char(c));
                ((front_parts, back_parts), part, side)
            }
            Char::Home => {
                push_parts((&mut front_parts, &mut back_parts), part, side);
                ((front_parts, back_parts), Vec::new(), PushSide::Front)
            }
            Char::End => {
                push_parts((&mut front_parts, &mut back_parts), part, side);
                ((front_parts, back_parts), Vec::new(), PushSide::Back)
            }
        },
    );
    push_parts((&mut front_parts, &mut back_parts), last, side);
    front_parts
        .into_iter()
        .rev()
        .chain(
            back_parts
                .into_iter()
                .rev()
                .fold((Vec::<char>::new(), 0_usize), fold_part)
                .0
                .into_iter()
                .rev(),
        )
        .collect()
}

fn push_parts(
    (front_parts, back_parts): (&mut Vec<char>, &mut Vec<CharOrBackspace>),
    mut part: Vec<CharOrBackspace>,
    side: PushSide,
) {
    match side {
        PushSide::Front => front_parts.append(
            &mut part
                .into_iter()
                .rev()
                .fold((Vec::<char>::new(), 0_usize), fold_part)
                .0,
        ),
        PushSide::Back => back_parts.append(&mut part),
    };
}

fn fold_part((mut acc, backs): (Vec<char>, usize), c: CharOrBackspace) -> (Vec<char>, usize) {
    let backs = match c {
        CharOrBackspace::Backspace => backs + 1,
        CharOrBackspace::Char(c) => match backs {
            0 => {
                acc.push(c);
                0
            }
            _ => backs - 1,
        },
    };
    (acc, backs)
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
}
