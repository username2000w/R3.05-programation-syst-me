fn main() {
    let sentence1 = "Bonjour Limoges";
    let sentence2 = "";

    print_first_word1(sentence1);
    print_first_word1(sentence2);

    println!("{}", String::from(print_first_word2(sentence1)));
    // println!("{}", String::from(print_first_word2(sentence2)));

    iterate_over_words(sentence1);
    iterate_over_words(sentence2);
}

fn print_first_word1(sentence: &str) {
    match sentence {
        "" => println!("Chaine vide"),
        _ => println!("Premier mot: {}", sentence.split(" ").next().unwrap()),
    }
}

fn print_first_word2(sentence: &str) -> &str {
    sentence.split_whitespace().next().expect("La chaîne dois être non vide")
}

fn iterate_over_words(sentence: &str) {
    for word in sentence.split_whitespace() {
        println!("Mot: {}", word);
    }
}