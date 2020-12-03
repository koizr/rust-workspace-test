fn main() {
    let num = 10;
    println!(
        "Hello, world! {} plus one is {}!",
        num,
        add_one::add_one(num)
    );
    println!(
        "Hello, world! {} plus x is {}!",
        num,
        add_one::add_random(num)
    );
}
