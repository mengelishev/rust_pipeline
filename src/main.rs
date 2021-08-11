fn main() {
    println!("Hello, World!");
    println!("Four: {}", get_four());
}

fn get_four() -> i32 {
    2 + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn super_test() {
        assert_eq!(get_four(), 4);
    }
}
