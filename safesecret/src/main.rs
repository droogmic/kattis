use std::io::{self, Read};

use safesecret::safe_secret;

fn main() {
    let input = {
        let mut input = String::new();
        io::stdin().read_to_string(&mut input).unwrap();
        input
    };
    let cost = safe_secret(input);
    println!("{}", cost);
}
