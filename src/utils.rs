const IGNORE_CHARS: [char; 9] = ['`', '\"', '\'', '(', ')', '[', ']', '&', ' '];

pub fn ignore(mut input: String) -> String {
    for chars in IGNORE_CHARS{
        input = input.replace(chars, format!("\\{}", chars).as_ref());
    }
    input
}

