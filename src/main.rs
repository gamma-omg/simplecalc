use std::io::Write;

fn main() {
    loop {
        print!("[>] ");
        std::io::stdout().flush().unwrap();

        let mut input = String::new();
        let bytes_read = std::io::stdin().read_line(&mut input).unwrap();
        if bytes_read <= 1 {
            break;
        }

        match simplecalc::eval(&input) {
            Ok(res) => println!("[=] {res}"),
            Err(e) => println!("[E] {e}"),
        }

        println!();
    }
}
