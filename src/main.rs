use std::io;

fn main() {
    println!("Fibbi Finder: please enter the Nth to be displayed: Integers please");

    let mut nth = String::new();
    let mut nth_clean_vec = Vec::new();

    io::stdin()
        .read_line(&mut nth)
        .expect("Failed to read line");

    for char in nth.chars(){
        if char.is_ascii_digit() {
            nth_clean_vec.push(char as u8)
        }
    }

    if nth_clean_vec.is_empty(){
        println!("Failure: Bad/No input detected");
        return;
    }

    let clean_string = String::from_utf8(nth_clean_vec).unwrap();
    let nth_clean_int: i32 = clean_string.parse().expect("Not a valid number");

    let mut fibbi_0: u128 = 0;
    let mut fibbi_1: u128 = 1;
    let mut fibbi_var: u128 = 0;

    for _i in 0..nth_clean_int  {
        // println!("fibbi_a = {fibbi_0}");
        // println!("fibbi_b = {fibbi_1}");
        fibbi_var = fibbi_0 + fibbi_1;
        // println!("fibbi_sum = {fibbi_var}");
        fibbi_0 = fibbi_1;
        fibbi_1 = fibbi_var;
    }

    println!("Your requested fibbi sequence {} was {}", nth.trim(), fibbi_var);
    main_again();
}

fn main_again() {
    println!("Would you like to sequence a new Nth? y/n");

    let mut continue_yn = String::new();
    io::stdin()
        .read_line(&mut continue_yn)
        .expect("Failed to read line");

    match continue_yn.trim() {
        "y" | "Y" => {
            main()
        },
        "n" | "N" => {
            println!("Exiting...");
            return;
        },
        _ => {
            println!("Failure: Bad Input, exiting...");
            return;
        },
    };
}
