fn main() {
    let mut input = String::new();
    let _ = std::io::stdin().read_line(&mut input).unwrap();
    let res = simplecalc::eval(&input).unwrap();
    println!("{res}");
}