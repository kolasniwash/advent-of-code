use std::collections::HashMap;

fn add_vowel_count(letter: char, vowels: &mut HashMap<char, i32>) {
    if letter == 'a' || letter == 'e' || letter == 'i' || letter == 'o' || letter == 'u' {
        *vowels.entry(letter).or_insert(1) += 1;
    }
}

fn not_contains_forbidden_string(slice: String) -> bool {
    if slice.contains("ab") || slice.contains("cd") || slice.contains("pq") || slice.contains("xy")
    {
        false
    } else {
        true
    }
}

fn is_nice_part_1(input: String) -> bool {
    let mut has_vowels = false;
    let mut has_double = false;
    let mut no_forbidden = true;

    let mut prev_char = 'a';
    let mut vowels: HashMap<char, i32> = HashMap::new();

    for curr in input.chars().enumerate() {
        let index = curr.0;
        let curr_char = curr.1;

        if index > 0 {
            if !has_double && curr_char.eq(&prev_char) {
                    has_double = true;
            }
            println!("{} {}", format!("{}{}", prev_char, curr_char), no_forbidden);
            if no_forbidden {
                no_forbidden = not_contains_forbidden_string(format!("{}{}", prev_char, curr_char))
            }

            if !has_vowels {
                add_vowel_count(curr_char, &mut vowels);
                let vowel_count: i32 = vowels.values().sum();
                if vowel_count > 2 {
                    has_vowels = true;
                }
            }

            prev_char = curr_char.clone();
        } else {
            add_vowel_count(curr_char, &mut vowels);
            prev_char = curr_char.clone();
        }
    }
    println!("{} {} {} {}", input, has_vowels, no_forbidden, has_double);
    if has_vowels && no_forbidden && has_double {
        true
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_is_nice_part_1() {
        assert_eq!(is_nice_part_1(String::from("ugknbfddgicrmopn")), true);
        assert_eq!(is_nice_part_1(String::from("aaa")), true);
        assert_eq!(is_nice_part_1(String::from("jchzalrnumimnmhp")), false);
        assert_eq!(is_nice_part_1(String::from("haegwjzuvuyypxyu")), false);
        assert_eq!(is_nice_part_1(String::from("dvszwmarrgswjxmb")), false);
    }
}
