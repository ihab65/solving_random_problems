fn validate(number: &String) {
    if number.starts_with("06") || number.starts_with("07") || number.starts_with("05") {
        println!(" <- Valid")
    } else {
        println!(" <- Not Valid");
    }
}

fn process(input: String) {
    let mut palce_holder = String::new();
    let input = input
        .trim_start_matches("0.")
        .to_string();

    let mut output = Vec::<String>::new();

    for c in input.chars() {

        palce_holder.push(c);
        if palce_holder.len() == 10 {
            output.push(palce_holder);
            palce_holder = String::new()
        }
    }

    for elem in output.iter(){
        print!("{}", elem);
        validate(elem)
    }
}

fn main() {
    let numbers = "0.0699827704077441894606998277040455555555".to_string();
    process(numbers);

}
