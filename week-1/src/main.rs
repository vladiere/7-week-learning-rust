use week_1::*;

fn main() {
    multiple_print(5, String::from("Hello Rustacean"));

    if check_number(5) {
        println!("It is between 1 to 100");
    } else {
        println!("It is not between 1 to 100");
    }

}
