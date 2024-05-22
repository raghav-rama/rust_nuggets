#[cfg(test)]
mod tests {
    pub fn string(string: &str) -> bool {
        let mut state = State {
            cursor: 0,
            characters: string.chars().collect::<Vec<_>>(),
        };

        loop {
            if !unit(&mut state) {
                break;
            }
        }

        state.cursor == state.characters.len()
    }

    fn unit(state: &mut State) -> bool {
        match state.characters.get(state.cursor) {
            Some(character) => {
                if character == &'*' {
                    state.cursor += 1;

                    true
                } else {
                    false
                }
            }
            None => false,
        }
    }

    struct State {
        cursor: usize,
        characters: Vec<char>,
    }

    #[test]
    fn test_empty_string() {
        let empty_string = "";
        assert!(string(empty_string)); // Empty string is valid
    }

    #[test]
    fn test_single_asterisk() {
        let single_asterisk = "*";
        assert!(string(single_asterisk));
    }

    #[test]
    fn test_multiple_asterisks() {
        let multiple_asterisks = "*****";
        assert!(string(multiple_asterisks));
    }

    #[test]
    fn test_non_asterisk_characters() {
        let invalid_string1 = "a**";
        let invalid_string2 = "*b*";
        let invalid_string3 = "**c";

        assert!(!string(invalid_string1));
        assert!(!string(invalid_string2));
        assert!(!string(invalid_string3));
    }

    #[test]
    fn test_mixed_characters() {
        let invalid_string = "ab*c*de";
        assert!(!string(invalid_string));
    }

    // Additional tests for edge cases (optional)
    #[test]
    fn test_unit_single_asterisk() {
        let mut state = State {
            cursor: 0,
            characters: vec!['*'],
        };
        assert!(unit(&mut state));
        assert_eq!(state.cursor, 1);
    }

    #[test]
    fn test_unit_non_asterisk() {
        let mut state = State {
            cursor: 0,
            characters: vec!['a'],
        };
        assert!(!unit(&mut state));
        assert_eq!(state.cursor, 0); // Cursor should not advance
    }
}
