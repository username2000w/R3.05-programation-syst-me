fn main() {
    let sentence1 = "Bonjour Limoges";
    let sentence2 = "";

    print_first_word1(sentence1);
    print_first_word1(sentence2);
}

fn print_first_word1(sentence: &str) {
    match sentence {
        "" => println!("Chaine vide"),
        _ => println!("Premier mot: {}", sentence.split(" ").next().unwrap()),
    }
}