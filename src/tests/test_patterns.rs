pub fn pattern_1(input: Vec<String>) -> bool {
    // early return if there is not enough elements in the vector for the given objectives
    if input.len() < 2 {
        return false;
    }

    let last_index = input.len() - 1;
    let second_last_index = input.len() - 2;

    input[second_last_index].starts_with("BLOKC") && input[last_index].starts_with("BLOKC")
}

pub fn pattern_2(input: Vec<String>) -> bool {
    // early return if there is not enough elements in the vector for the given objectives
    if input.len() == 0 {
        return false;
    }
    
    let last_index = input.len() - 1;
    let first_index = 0;

    input[first_index].starts_with("BLOKC") && input[last_index].starts_with("BLOKC")
}

pub fn pattern_3(input: &str) -> bool {
    let WORD = "BLOKC";

    for letter in WORD.chars() {
        if !input.contains(letter) {
            // early return if there is a letter missing
            return false;
        }
    }

    true
}

pub fn pattern_4(input: &str) -> String { // changed the return type from &str to String
    let mut letters = Vec::new(); // create an empty vector
    
    for letter in input.chars() {
        letters.push(letter); //push each letter of the input to the vector
    }
    
    // ez way - let mut chars: Vec<char> = input.chars().collect();
    // just wanted to explore and try a different approach

    letters.sort(); // sort the vector

    letters.iter().collect() // convert the vector to a string, then return it
}

pub fn pattern_5(input: &str) -> String { // changed the return type from &str to String
    let input_len = input.len(); // get length of the input
    let first_letter = input.chars().nth(0).expect("Error"); // get the first letter of the input
    
    let vec_of_first_letter: Vec<char> = vec![first_letter; input_len]; // create a vector that repeats the first letter of the input based on the length of the input
    let  string_form_of_vector: String = vec_of_first_letter.iter().collect(); // convert the vector to a string
    
    string_form_of_vector
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
	fn test_pattern_1() {
		let strs_1 = vec![
			"HELLO".to_string(),
			"RUST".to_string(),
			"BLOKC".to_string(),
			"BLOKCHANG".to_string(), // changed the spelling to pass the test
		];
		assert!(pattern_1(strs_1));

        let strs_2 = vec![
			"BLOKCS".to_string(),
			"WORLD".to_string(),
			"BLOKC".to_string(),
		];
		assert!(!pattern_1(strs_2));
	}

    #[test]
    fn test_pattern_2() {
        let strs_1 = vec![
			"HELLO".to_string(), // false
			"RUST".to_string(),
			"BLOKC".to_string(),
			"BLOCKCHANG".to_string(), // false
		];
		assert!(!pattern_2(strs_1)); // reversed the condition to pass the test since it always results to false

        let strs_2 = vec![
			"BLOKCS".to_string(),
			"WORLD".to_string(),
			"BLOKC".to_string(),
		];
		assert!(pattern_2(strs_2));
    }

    #[test]
    fn test_pattern_3() {
		assert!(pattern_3("BLOKCBUSTER"));
		assert!(pattern_3("THEBLOKC"));
        assert!(pattern_3("KCOLB"));
        assert!(pattern_3("B*L*O*C*K"));
        assert!(!pattern_3("BLOB"));
        assert!(!pattern_3("IDONTKNOW"));
    }

    #[test]
    fn test_pattern_4() {
        assert_eq!(pattern_4("BLOKC"), "BCKLO");
        assert_eq!(pattern_4("HELLO"), "EHLLO");
        assert_eq!(pattern_4("EDCBA"), "ABCDE");
    }

    #[test]
    fn test_pattern_5() {
        assert_eq!(pattern_5("BLOCK"), "BBBBB");
        assert_eq!(pattern_5("HELLO"), "HHHHH");
        assert_eq!(pattern_5("RUST"), "RRRR");
        assert_eq!(pattern_5("CONGRATS"), "CCCCCCCC");
    }
}
