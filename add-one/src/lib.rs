extern crate rand;

pub fn add_one(x: i32) -> i32 {
    x + 1
}

pub fn add_random(x: i32) -> i32 {
    let r: i32 = rand::random();
    x + r
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(add_one(5), 6);
    }
}
