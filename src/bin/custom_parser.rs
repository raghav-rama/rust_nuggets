fn main() {
    println!("This is the start of making a custom parser in Rust.")
}

/**
 * Given a grammar, writing a parser boils down to finding a method to translate each production rule into a function.
 * It's convenient to name these functions after the production rules they express.
 * The parser for the Star language would have two functions named string and unit, respectively.
 * By translating syntax features into programming language control statements, we can arrive at a method to translate
 * the Star grammar into a validating parser.
 */
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
/**
 * To test whether a str encodes a member of the Star language, we call the string function.
 * A return value of true indicates membership.
 * Notice that Rust's Boolean type serves as the data structure returned from this parser.
 * This return value can be as simple or complex as you'd like.
 */
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
