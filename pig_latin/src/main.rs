use std::io;

fn main() {
    //ask for user input
    println!("Enter set of words!");
    let mut words = String::new();

    io::stdin().read_line(&mut words)
        .expect("Failed to read line!");

    //extract pig latin with function call
    let result = pig_latin_words(& words);

    //print result
    println!("The pig latin is: \n\t {result}");
}

//to check for vowel initial
fn is_vowel_initial (initial: &str) -> bool {
    let vowel = ["a", "e", "i", "o", "u"];

    for ch in vowel.iter() {
        if *ch == initial {
            return true;
        }
    }
    false
}

fn pig_latin_words (words: &String) -> String{
    //create new strng to store pig latin words
    let mut pig_latin = String::new();

    //split string into word
    for word in words.split_whitespace() {
        let initial = &word[0..1];

        //add hay to vowel initials
        if is_vowel_initial(initial) {
            pig_latin.push_str(word);
            pig_latin.push_str("-hay ");
        }
        //The first consonant of each word is moved to the end of the word and ay is added
        else{
            pig_latin.push_str(&word[1..]);
            pig_latin.push_str("-");
            pig_latin.push_str(initial);
            pig_latin.push_str("ay ");
        }
    }
    pig_latin
}
