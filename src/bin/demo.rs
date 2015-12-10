extern crate linegrab;

fn main() {
    for line in linegrab::linegrab(std::env::args().nth(1), std::io::stdin()) {
        println!("{}", line);
    }
}
