#[allow(dead_code)]
fn split_string<'a>(string: &'a str, delimeter: &str) -> Vec<&'a str> {
    if string.is_empty() || delimeter.is_empty() {
        return Vec::new();
    }

    let mut result = Vec::new();
    let mut start = 0;

    while let Some(end) = string[start..].find(delimeter) {
        let end = start + end;
        if start != end {
            result.push(&string[start..end]);
        }
        start = end + delimeter.len();
    }

    if start < string.len() {
        result.push(&string[start..]);
    }

    result
}

#[derive(PartialEq, Debug)]
struct Differences<'a> {
    only_in_first: Vec<&'a str>,
    only_in_second: Vec<&'a str>,
}

#[allow(dead_code)]
fn find_differences<'a>(first_string: &'a str, second_string: &'a str) -> Differences<'a> {
    let first_words: Vec<&str> = first_string.split_whitespace().collect();
    let second_words: Vec<&str> = second_string.split_whitespace().collect();

    let only_in_first: Vec<&str> = first_words
        .iter()
        .filter(|&&word1| !second_words.iter().any(|&word2| word1 == word2))
        .cloned()
        .collect();
    let only_in_second: Vec<&str> = second_words
        .iter()
        .filter(|&&word2| !first_words.iter().any(|&word1| word1 == word2))
        .cloned()
        .collect();

    Differences {
        only_in_first,
        only_in_second,
    }
}

#[allow(dead_code)]
fn merge_names<'a>(first_name: &'a str, second_name: &'a str) -> String {
    let mut buffer = String::new();
    let mut first_chars = first_name.chars().peekable();
    let mut second_chars = second_name.chars().peekable();

    fn is_vowel(c: char) -> bool {
        matches!(c.to_ascii_lowercase(), 'a' | 'e' | 'i' | 'o' | 'u')
    }

    fn append_until_vowel<'b>(
        chars: &'b mut std::iter::Peekable<impl Iterator<Item = char>>,
        buffer: &'b mut String,
    ) {
        let mut first_char_added = false;

        while let Some(&c) = chars.peek() {
            if !first_char_added {
                buffer.push(c);
                chars.next();
                first_char_added = true;
            } else if is_vowel(c) {
                break;
            } else {
                buffer.push(c);
                chars.next();
            }
        }
    }

    let mut first_turn = true;

    while first_chars.peek().is_some() || second_chars.peek().is_some() {
        if first_turn {
            append_until_vowel(&mut first_chars, &mut buffer);
        } else {
            append_until_vowel(&mut second_chars, &mut buffer);
        }
        first_turn = !first_turn;
    }

    buffer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_string() {
        // First, make sure the lifetimes were correctly marked
        let matches;
        let string_to_split = String::from("Hello, World!");

        {
            let delimeter = String::from(", ");
            matches = split_string(&string_to_split, &delimeter);
        }
        println!("Matches can be printed! See: {:?}", matches);

        // Now check the split logic
        assert_eq!(split_string(&"", &""), Vec::<&str>::new());
        assert_eq!(
            split_string(&"Hello, World!", &", "),
            vec!["Hello", "World!"]
        );
        assert_eq!(
            split_string(
                &"I this think this that this sentence this is this very this confusing this ",
                &" this "
            ),
            vec!["I", "think", "that", "sentence", "is", "very", "confusing"]
        );
        assert_eq!(
            split_string(&"apple🍎banana🍎orange", &"🍎"),
            vec!["apple", "banana", "orange"]
        );
        assert_eq!(
            split_string(
                &"Ayush;put|a,lot~of`random;delimeters|in|this,sentence",
                &";"
            ),
            vec![
                "Ayush",
                "put|a,lot~of`random",
                "delimeters|in|this,sentence"
            ]
        );
    }

    #[test]
    fn test_find_differences() {
        assert_eq!(
            find_differences(&"", &""),
            Differences {
                only_in_first: Vec::new(),
                only_in_second: Vec::new()
            }
        );
        assert_eq!(
            find_differences(&"pineapple pen", &"apple"),
            Differences {
                only_in_first: vec!["pineapple", "pen"],
                // only_in_second: Vec::new()
                only_in_second: vec!["apple"]
            }
        );
        assert_eq!(
            find_differences(
                &"Sally sold seashells at the seashore",
                &"Seashells seashells at the seashore"
            ),
            Differences {
                only_in_first: vec!["Sally", "sold"],
                only_in_second: vec!["Seashells"]
            }
        );
        assert_eq!(
            find_differences(
                "How much wood could a wood chuck chuck",
                "If a wood chuck could chuck wood"
            ),
            Differences {
                only_in_first: vec!["How", "much"],
                only_in_second: vec!["If"]
            }
        );
        assert_eq!(
            find_differences(
                &"How much ground would a groundhog hog",
                &"If a groundhog could hog ground"
            ),
            Differences {
                only_in_first: vec!["How", "much", "would"],
                only_in_second: vec!["If", "could"]
            }
        );
    }

    #[test]
    fn test_merge_names() {
        assert_eq!(merge_names(&"alex", &"jake"), "aljexake");
        assert_eq!(merge_names(&"steven", &"stephen"), "ststevephenen");
        assert_eq!(merge_names(&"gym", &"rhythm"), "gymrhythm");
        assert_eq!(merge_names(&"walter", &"gibraltor"), "wgaltibreraltor");
        assert_eq!(merge_names(&"baker", &"quaker"), "bqakueraker");
        assert_eq!(merge_names(&"", &""), "");
        assert_eq!(merge_names(&"samesies", &"samesies"), "ssamamesesiieses");
        assert_eq!(merge_names(&"heather", &"meagan"), "hmeeathageran");
        assert_eq!(merge_names(&"panda", &"turtle"), "ptandurtlae");
        assert_eq!(merge_names(&"hot", &"sauce"), "hsotauce");
        assert_eq!(merge_names(&"", &"second"), "second");
        assert_eq!(merge_names(&"first", &""), "first");
    }
}
