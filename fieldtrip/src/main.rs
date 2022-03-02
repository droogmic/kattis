use std::io::{self, Read};

fn main() {
    let input = {
        let mut input = String::new();
        io::stdin().read_to_string(&mut input).unwrap();
        input
    };
    let cost = fieldtrip(input);
    println!("{}", cost);
}

pub fn fieldtrip(input: String) -> String {
    // Parsing
    let mut lines = input.lines();
    let num_class_section: usize = lines.next().unwrap().parse().unwrap();
    let class_section: Vec<usize> = lines
        .next()
        .unwrap()
        .split(' ')
        .map(|t| t.parse().unwrap())
        .collect();
    assert_eq!(num_class_section, class_section.len());

    let sum = class_section.iter().sum::<usize>();
    let bus_size = sum / 3;
    let sections = class_section
        .iter()
        .enumerate()
        .fold(
            Some((Vec::new(), 0)),
            |acc: Option<(Vec<usize>, usize)>, (section_idx, section_size)| {
                acc.and_then(|(mut result, partial_bus)| {
                    let new_bus = partial_bus + section_size;
                    if new_bus > bus_size {
                        None
                    } else if new_bus == bus_size {
                        result.push(section_idx + 1);
                        Some((result, 0))
                    } else {
                        Some((result, new_bus))
                    }
                })
            },
        )
        .map(|(mut sections, _)| {
            sections.pop();
            sections
        });

    return sections.map_or("-1".to_owned(), |s| {
        s.iter()
            .map(|i| i.to_string())
            .collect::<Vec<_>>()
            .as_slice()
            .join(" ")
    });
}

#[cfg(test)]
mod tests {
    use crate::fieldtrip;

    #[test]
    fn test_example() {
        let input = "9\n1 2 3 1 2 3 1 2 3";
        let res = fieldtrip(input.to_owned());
        println!("{}", res);
        assert_eq!(res, "3 6");
    }
}
