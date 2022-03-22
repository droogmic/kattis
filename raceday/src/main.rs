use std::{
    cmp::Ord,
    collections::HashMap,
    fmt::Display,
    io::{self, Read},
    num::ParseIntError,
    str::FromStr,
};

fn main() {
    let input = {
        let mut input = String::new();
        io::stdin().read_to_string(&mut input).unwrap();
        input
    };
    let res = raceday(input);
    println!("{}", res);
}

type Bib = String;
struct Runners(HashMap<Bib, Runner>);
impl FromIterator<(Bib, Runner)> for Runners {
    fn from_iter<T: IntoIterator<Item = (Bib, Runner)>>(iter: T) -> Runners {
        Runners(iter.into_iter().collect())
    }
}
macro_rules! table_fmt_str {
    () => {
        "{:<20}{:>10}{:>10}{:>10}{:>10}{:>10}{:>10}{:>10}"
    };
}
impl Display for Runners {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            table_fmt_str!(),
            "NAME", "BIB", "SPLIT1", "RANK", "SPLIT2", "RANK", "FINISH", "RANK"
        )?;
        let mut runners: Vec<(&Bib, &Runner)> = self.0.iter().collect();
        runners.sort_unstable_by(|(_, a), (_, b)| {
            a.last_name
                .partial_cmp(&b.last_name)
                .unwrap()
                .then_with(|| a.first_name.partial_cmp(&b.first_name).unwrap())
        });
        let mut split_1_ranking: Vec<&Runner> = self.0.values().collect();
        split_1_ranking.sort_unstable_by_key(|r| r.splits.split_1.as_ref().unwrap());
        let mut split_2_ranking: Vec<&Runner> = self.0.values().collect();
        split_2_ranking.sort_unstable_by_key(|r| r.splits.split_2.as_ref().unwrap());
        let mut finish_ranking: Vec<&Runner> = self.0.values().collect();
        finish_ranking.sort_unstable_by_key(|r| r.splits.finish.as_ref().unwrap());
        for (bib, runner) in runners {
            writeln!(
                f,
                table_fmt_str!(),
                format!("{}, {}", runner.last_name, runner.first_name),
                bib,
                runner.splits.split_1.as_ref().unwrap().to_string(),
                split_1_ranking.iter().position(|r| r == &runner).unwrap() + 1,
                runner.splits.split_2.as_ref().unwrap().to_string(),
                split_2_ranking.iter().position(|r| r == &runner).unwrap() + 1,
                runner.splits.finish.as_ref().unwrap().to_string(),
                finish_ranking.iter().position(|r| r == &runner).unwrap() + 1
            )?;
        }
        Ok(())
    }
}

#[derive(PartialEq)]
struct Runner {
    first_name: String,
    last_name: String,
    splits: Splits,
}

#[derive(Default, PartialEq)]
struct Splits {
    split_1: Option<Time>,
    split_2: Option<Time>,
    finish: Option<Time>,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Time {
    minutes: u8,
    seconds: u8,
}
impl FromStr for Time {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(":");
        Ok(Self {
            minutes: split.next().unwrap().parse()?,
            seconds: split.next().unwrap().parse()?,
        })
    }
}
impl Display for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}:{:02}", self.minutes, self.seconds)
    }
}

pub fn raceday(input: String) -> String {
    let mut lines = input.lines();
    let mut output = Vec::new();
    loop {
        let num_racers: usize = lines.next().unwrap().parse().unwrap();
        if num_racers == 0 {
            break;
        }
        let mut runners: Runners = lines
            .by_ref()
            .take(num_racers)
            .map(|l| {
                let mut parts = l.split_whitespace();
                let first_name = parts.next().unwrap().to_owned();
                let last_name = parts.next().unwrap().to_owned();
                let bib = parts.next().unwrap().to_owned();
                (
                    bib,
                    Runner {
                        first_name,
                        last_name,
                        splits: Splits::default(),
                    },
                )
            })
            .collect();
        for line in lines.by_ref().take(3 * num_racers) {
            let mut parts = line.split_whitespace();
            let splits = &mut runners.0.get_mut(parts.next().unwrap()).unwrap().splits;
            let split_type = parts.next().unwrap();
            let split_val: Time = parts.next().unwrap().parse().unwrap();
            match split_type {
                "S1" => splits.split_1 = Some(split_val),
                "S2" => splits.split_2 = Some(split_val),
                "F" => splits.finish = Some(split_val),
                _ => unreachable!(),
            }
        }
        output.push(runners.to_string())
    }
    output.join("\n")
}

#[cfg(test)]
mod tests {
    use crate::raceday;

    #[test]
    fn test_example_1() {
        let input = include_str!("example_in.txt");
        let res = raceday(input.to_owned());
        println!("{}", res);
        assert_eq!(res, include_str!("example_out.txt"));
    }
}
