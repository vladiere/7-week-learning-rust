pub fn check_number(
    // Enter number 1 to 100
    // Check if number is in between 1 to 100
    numb: u32
) -> bool {
    numb > 0 || numb < 100
}

pub fn multiple_print(
    // Enter number of series to print a message
    n: u32,
    // Enter the message you want to print
    m: String,
) {
    let mut i = 0;
    while i < n {
        println!("{}",m);

        i += 1;
    }
}

