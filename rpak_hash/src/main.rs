fn main() {
    println!("Input string to hash");

    let mut to_hash = String::new();

    std::io::stdin()
        .read_line(&mut to_hash)
        .expect("Failed to read line");

    to_hash = to_hash.replace("\n", "");

    println!(
        "{:X} | {:?}",
        rpak::hash(to_hash.clone()),
        to_hash.as_bytes()
    )
}
