fn main() {
    let mut count = 0;
    for arg in std::env::args().skip(1) {
        count += 1;
        println!("Arg {count}: {arg}");
    }

    if count == 0 {
        println!("No arguments passed. Try: cargo run -- foo bar baz");
    }
}
