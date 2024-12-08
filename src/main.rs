mod d1;
mod d2;
mod utils;

fn main() {
    //calling days
    let res =
        d2::d2::get_nb_safe_2("../inputs/day2").unwrap_or_else(|error| panic!("Err: {:?}", error));
    println!("Day Result: {}", res);
}
