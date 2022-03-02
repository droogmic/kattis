use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};
use std::io::{self, Read};

#[derive(Clone, Debug)]
struct Map(HashMap<(isize, isize), Cell>);

#[derive(Clone, Debug)]
enum Cell {
    Bank,
    Terrain(Terrain),
    Dot,
    Barricade(Terrain),
}

impl Cell {
    fn from_char(c: char) -> Self {
        match c {
            'B' => Self::Bank,
            '.' => Self::Dot,
            c => Self::Terrain(
                Terrain::try_from(u32::from(c)).unwrap()
                    - Terrain::try_from(u32::from('a')).unwrap(),
            ),
        }
    }
}

type Terrain = usize;
type Cost = usize;

fn main() {
    let input = {
        let mut input = String::new();
        io::stdin().read_to_string(&mut input).unwrap();
        input
    };
    let cost = cops_and_robbers(input);
    println!("{}", cost);
}

fn cops_and_robbers(input: String) -> Cost {
    // Parsing
    let mut lines = input.lines();
    let mut first = lines.next().unwrap().split(' ');
    let n: usize = first.next().unwrap().parse().unwrap();
    let m: usize = first.next().unwrap().parse().unwrap();
    let c: usize = first.next().unwrap().parse().unwrap();
    let mut map: Map = Map(HashMap::new());
    let mut bank_pos = None;
    for (y, line) in lines.by_ref().take(m).enumerate() {
        assert!(y < m);
        let y = y.try_into().unwrap();
        for (x, c) in line.chars().enumerate() {
            assert!(x < n);
            let x = x.try_into().unwrap();
            map.0.insert((x, y), Cell::from_char(c));
            if c == 'B' {
                bank_pos = Some((x, y));
            }
        }
    }
    let terrain_costs: Vec<Cost> = lines
        .next()
        .unwrap()
        .split(' ')
        .map(|c| c.parse().unwrap())
        .collect();
    assert_eq!(terrain_costs.len(), c);

    // eprintln!("{:#?}", map);

    // Calc
    let all_barricades = {
        let mut barricade_map = map.clone();
        let mut cost = 0;
        for cell in barricade_map.0.values_mut() {
            if let Cell::Terrain(c) = cell.clone() {
                *cell = Cell::Barricade(c);
                cost += terrain_costs[c];
            }
        }
        (barricade_map, cost)
    };
    let barricade_options = recursive_reduce(
        &all_barricades.0,
        all_barricades.1,
        bank_pos.unwrap(),
        &terrain_costs,
    );
    let min_barricade = barricade_options
        .iter()
        .min_by_key(|(_map, cost)| cost)
        .unwrap();

    // eprintln!("{:#?}", min_barricade.0);

    min_barricade.1
}

fn recursive_reduce(
    map: &Map,
    cost: Cost,
    bank_pos: (isize, isize),
    terrain_costs: &[Cost],
) -> Vec<(Map, Cost)> {
    let mut options: Vec<(Map, Cost)> = Vec::new();
    for (loc, cell) in map.0.iter() {
        if let Cell::Barricade(c) = cell {
            let mut next_map = map.clone();
            next_map
                .0
                .entry(*loc)
                .and_modify(|cell| *cell = Cell::Terrain(*c));
            let next_cost = cost - terrain_costs[*c];
            let mut next_options = recursive_reduce(&next_map, next_cost, bank_pos, terrain_costs);
            options.append(&mut next_options);
            if !check_escape(&next_map, &[bank_pos]) {
                options.push((next_map, next_cost))
            }
        }
    }
    options
}

/// return if escape
fn check_escape(map: &Map, path: &[(isize, isize)]) -> bool {
    let last = path.last().unwrap();
    for offset in [(1, 0), (0, 1), (-1, 0), (0, -1)].iter() {
        let next = (last.0 + offset.0, last.1 + offset.1);
        if path.contains(&next) {
            continue;
        }
        match map.0.get(&next) {
            None => return true, // Escaped
            Some(Cell::Barricade(_)) => {}
            Some(_) => {
                let mut new_path = path.to_vec();
                new_path.push(next);
                if check_escape(map, &new_path) {
                    return true;
                }
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{check_escape, cops_and_robbers, Cell, Map};

    #[test]
    fn test_example() {
        let input = r#"4 3 3
.abc
abBc
.abc
1 7 5"#;
        assert_eq!(cops_and_robbers(input.to_owned()), 22);
    }

    #[test]
    fn test_check_escape() {
        let mut map: Map = Map(HashMap::new());
        map.0.insert((0, 0), Cell::Dot);
        map.0.insert((1, 0), Cell::Barricade(0));
        map.0.insert((2, 0), Cell::Barricade(0));
        map.0.insert((3, 0), Cell::Dot);
        map.0.insert((0, 1), Cell::Barricade(0));
        map.0.insert((1, 1), Cell::Bank);
        map.0.insert((2, 1), Cell::Dot);
        map.0.insert((3, 1), Cell::Barricade(0));
        map.0.insert((0, 2), Cell::Barricade(0));
        map.0.insert((1, 2), Cell::Dot);
        map.0.insert((2, 2), Cell::Terrain(0));
        map.0.insert((3, 2), Cell::Barricade(0));
        map.0.insert((0, 3), Cell::Dot);
        map.0.insert((1, 3), Cell::Barricade(0));
        map.0.insert((2, 3), Cell::Barricade(0));
        map.0.insert((3, 3), Cell::Dot);
        let bank = vec![(1, 1)];
        assert!(!check_escape(&map, &bank));
        map.0.insert((3, 2), Cell::Terrain(0));
        assert!(check_escape(&map, &bank));
    }
}
