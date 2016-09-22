//https://www.reddit.com/r/dailyprogrammer/comments/52enht/20160912_challenge_283_easy_anagram_detector/
use std::collections::HashMap;

fn main() {

//    let inputs = ["\"Clint Eastwood\" ? \"Old West Action\""; "\"parliament\" ? \"partial man\""];

    process_input("\"Clint Eastwood\" ? \"Old West Action\"");
    process_input("\"parliament\" ? \"partial man\"");
}

fn process_input(input: &str) {
    println!("{}", input);
    let lower = input.to_lowercase();
    let mut split = lower.split(" ? ");

    let mut char_counts: HashMap<char, i32> = HashMap::new();
    let mut first: bool = true;
    for s in split {
        for c in s.chars() {
            if !c.is_whitespace() {
                let change = if first { 1 } else { -1 };
                *char_counts.entry(c).or_insert(0) += change;
            }
        }
        first = false;
    }

    let mut anagram = true;
    for (c, count) in &char_counts {
        if *count != 0 {
            anagram = false;
        }
    }

    split = lower.split("?");
    let vec: Vec<&str> = split.collect();

    let message = if anagram { "is an anagram of" } else { "is not an anagram of" };
    println!("{}{}{}", vec[0], message, vec[1]);
}