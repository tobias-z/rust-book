fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    println!("{}", celsius_to_fahrenheit(25.0));
    println!("{}", fahrenheit_to_celsius(77.0));

    let yaaa = String::from("this is a thirng");
    let word = get_first_word(&yaaa);
    println!("{}", word);
}

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    celsius * 1.8000 + 32.0
}

fn fahrenheit_to_celsius(fah: f64) -> f64 {
    (fah - 32.0) / 1.8000
}

fn string_test() {
    let s = String::from("hello"); // s comes into scope

    takes_ownership(&s); // s's value moves into the function...

    println!("{}", s)
}

fn takes_ownership(s: &String) {
    println!("{}", s)
}

fn get_first_word(s: &str) -> &str {
    s.split_whitespace().next().expect("No idea what happened")
}
