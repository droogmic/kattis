use std::io::{self, Read};

#[derive(Clone, Copy, Debug)]
struct Point {
    x: f32,
    y: f32,
}

impl Point {
    fn dot(self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y
    }
    fn cross(self, other: Self) -> f32 {
        self.x * other.y - self.y * other.x
    }
}

impl std::ops::Add for Point {
    type Output = Point;
    fn add(self, other: Self) -> Self::Output {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl std::ops::Sub for Point {
    type Output = Point;
    fn sub(self, other: Self) -> Self::Output {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl std::ops::Mul<Point> for f32 {
    type Output = Point;
    fn mul(self, other: Point) -> Point {
        Point {
            x: self * other.x,
            y: self * other.y,
        }
    }
}

impl std::str::FromStr for Point {
    type Err = std::num::ParseFloatError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(' ');
        Ok(Self {
            x: split.next().unwrap().parse()?,
            y: split.next().unwrap().parse()?,
        })
    }
}

fn main() {
    let input = {
        let mut input = String::new();
        io::stdin().read_to_string(&mut input).unwrap();
        input
    };
    target(input);
}

fn target(input: String) {
    // Parsing
    let mut lines = input.lines();
    let num_vertices: usize = lines.next().unwrap().parse().unwrap();
    let vertices: Vec<Point> = {
        let mut vertices: Vec<Point> = lines
            .by_ref()
            .take(num_vertices)
            .map(|s| s.parse().unwrap())
            .collect();
        if vertices.first().unwrap().x > vertices.last().unwrap().x {
            vertices.reverse();
        }
        vertices.push(vertices.first().unwrap().clone());
        vertices
    };
    let mut cases = Vec::new();
    loop {
        let num_shots: usize = lines.next().unwrap().parse().unwrap();
        if num_shots == 0 {
            break;
        }
        let shots: Vec<Point> = lines
            .by_ref()
            .take(num_shots)
            .map(|s| s.parse().unwrap())
            .collect();
        cases.push(shots);
    }

    // Calc
    for (idx, shots) in cases.into_iter().enumerate() {
        println!("Case {}", idx + 1);
        for shot in shots {
            let edge_distances: Vec<(f32, f32)> = vertices
                .as_slice()
                .windows(2)
                .map(|edges| {
                    if let [l, r] = edges {
                        point_line_segment_distance(shot, *l, *r)
                    } else {
                        unreachable!()
                    }
                })
                .collect();
            let min = edge_distances
                .into_iter()
                .min_by(|left, right| left.0.partial_cmp(&right.0).unwrap())
                .unwrap();

            if min.1 > 0.0 {
                println!("Miss! {:.8}", min.0);
            } else if min.1 < 0.0 {
                println!("Hit! {:.8}", min.0);
            } else {
                println!("Winged!");
            }
        }
    }
}

fn point_distance_squared(p1: Point, p2: Point) -> f32 {
    (p2.x - p1.x).powi(2) + (p2.y - p1.y).powi(2)
}

fn point_distance(p1: Point, p2: Point) -> f32 {
    point_distance_squared(p1, p2).sqrt()
}

fn point_line_segment_distance(p0: Point, p1: Point, p2: Point) -> (f32, f32) {
    let t = (p2 - p1).dot(p0 - p1) / point_distance_squared(p1, p2);
    let t = t.min(1.0).max(0.0);
    let projection = p1 + t * (p2 - p1);
    (point_distance(projection, p0), (p2 - p1).cross(p0 - p1))
}

#[cfg(test)]
mod tests {
    use crate::target;

    #[test]
    fn example() {
        let input = r#"4
0 0
0 10
10 10
10 0
2
3 1
11 11
3
0 0
10 10
10 0
1
5 0
0"#;
        target(input.to_owned());
    }
}
