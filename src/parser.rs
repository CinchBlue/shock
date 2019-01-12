use nom::{take_while1, is_digit, anychar};

/// Verifies that a character is an identifier character.
fn is_identifier_char(chr: u8) -> bool {
    chr == b'-' || chr == b'_' || chr.is_ascii_alphanumeric()
}

/// Verifies that a character is a ASCII lowercase character.
fn is_lowercase_alpha(chr: u8) -> bool {
    chr.is_ascii_lowercase()
}

/// Transform a vector into a string.
pub fn vec_to_string(input: &[u8]) -> String {
    String::from_utf8_lossy(input).into_owned()
}

pub fn to_s(i: Vec<u8>) -> String {
    String::from_utf8_lossy(&i).into_owned()
}

/// Creates a Vec<u8> of identifier-valid characters.
named!(identifier_consequent,
    take_while!(is_identifier_char));

/// Identifiers must start with lowercase characters.
named!(identifier_initial,
    take_while_m_n!(1, 1, is_lowercase_alpha));

/// Identifiers must start with lowercase characters and can then be followed up with
/// alphanumeric characters or dashes or underscores.
named!(pub identifier<String>,
    do_parse!(
        initial: identifier_initial >>
        rest: identifier_consequent >>
        (vec_to_string(initial) + &String::from_utf8_lossy(rest).into_owned())
    )
);

/// A string is between double quotes.
named!(string<String>,
    do_parse!(
        tag!("\"") >>
        string_contents: take_until!("\"") >>
        tag!("\"")
        >>
        (vec_to_string(string_contents))
    )
);

named!(character<String>,
    do_parse!(
        tag!("\'") >>
        c: anychar >>
        tag!("\'") >>
        (c.to_string())
    )
);

named!(integer_digits,
    take_while1!(is_digit));

named!(integer<String>,
    do_parse!(
        digits: integer_digits >>
        (vec_to_string(digits))
    )
);

#[cfg(test)]
mod tests {
    use crate::parser::identifier;
    use crate::parser::character;
    
    #[test]
    fn it_works() {
        assert_eq!(identifier(b"abas ").ok().unwrap().1, "abas".to_owned());
        assert_eq!(identifier(b"--- ").is_err(), true);
        assert_eq!(character(b"\'a\'bc").ok().unwrap().1, "a".to_owned());
    }
}

 



