// string-exercices.rs
fn print_my_string(to_print: &str) {
    println!("{}", to_print);
}

fn main() {
    let static_one = "Static Hello World !";
    let mut my_string = String::new();
    my_string.push('H');
    my_string.push_str("ello World !");

    print_my_string(static_one);
    print_my_string(&my_string);

    let utf8_string =
        String::from_utf8(vec![72, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100, 32, 33])
            .expect("Something terrible happened during the utf8 sequence decoding process !");

    let non_utf8_string =
        String::from_utf8(vec![256])
            .expect("Something terrible happened during the utf8 sequence decoding process !");

    print_my_string(&utf8_string);
    print_my_string(&non_utf8_string);

    let rusty_potter = String::from("You are a Rust coder, Harry");

    let potter_array: Vec<&str> = rusty_potter.split_whitespace().collect();

    print_my_string(potter_array[2]);
}
