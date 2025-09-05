pub fn ownership_demo() {
    let s = String::from("hello");
    let t = s;

    println!("t = {}", t);
    println!("s = {}", s);
}