fn main() {
    let string1 = "-17";
    let string2 = "Tux";

    convert_to_int(string1);
    convert_to_int(string2);

    convert_to_int2(string1);
    // convert_to_int2(string2);
}

fn convert_to_int(string : &str) {
    match string.parse::<i32>() {
        Ok(i) => println!("Le carré de {} vaux {}", string, i.pow(2)),
        Err(e) => println!("{} n'est pas un entier: {}", string, e),
    }
}

fn convert_to_int2(string : &str) {
    string.parse::<i32>().expect("invalid digit found in string");
}