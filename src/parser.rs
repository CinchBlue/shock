use nom::{take_while1, is_digit, anychar};

use crate::model::{PrimitiveData};
use crate::model::{Path};

use nom::{digit, is_space};
use std::prelude::v1::Vec;

/// Verifies that a character is an identifier character.
pub fn is_identifier_char(chr: u8) -> bool {
    chr == b'-' || chr == b'_' || chr.is_ascii_alphanumeric()
}

/// Verifies that a character is a ASCII lowercase character.
pub fn is_lowercase_alpha(chr: u8) -> bool {
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
named!(pub identifier_consequent,
    take_while!(is_identifier_char));

/// Identifiers must start with lowercase characters.
named!(pub identifier_initial,
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
named!(pub string<String>,
    do_parse!(
        tag!("\"") >>
        string_contents: take_until!("\"") >>
        tag!("\"")
        >>
        (vec_to_string(string_contents))
    )
);

named!(pub character<String>,
    do_parse!(
        tag!("\'") >>
        c: anychar >>
        tag!("\'") >>
        (c.to_string())
    )
);

named!(pub integer_digits,
    take_while1!(is_digit));

named!(pub integer<String>,
    do_parse!(
        digits: integer_digits >>
        (vec_to_string(digits))
    )
);

#[derive(Debug)]
pub enum EditorCommandKeyword {
    Show,
    Construct,
    Delete,
    Put,
    Focus
}

pub enum EditorCommand {
    SHOW(Path),
    CONSTRUCT(Path, PrimitiveData),
    DELETE(Path),
    UPDATE(Path, PrimitiveData),
    FOCUS(Path),
}

#[derive(Debug, Clone)]
pub struct Command {
    pub name: String,
    pub args: Vec<(String, ExpressionValue)>,
}

impl Command {
    pub fn new() -> Self {
        Command { name: String::new(), args: Vec::new() }
    }
}

pub type ArgumentList = Vec<(String, ExpressionValue)>;

//impl Clone for Command {
//    fn clone(&self) -> Self {
//        let mut v: Vec<(String, ExpressionValue)> = Vec::new();
//        self.args.clone_into(&mut v);
//        Command {name: self.name.clone(), args: v}
//    }
//}

//impl<'a> FromIterator for &'a Command {
//    fn from_iter<T: IntoIterator<Item=&'a Command>>(iter: T) -> Self {
//        let mut v: Vec<Command> = Vec::new();
//
//        for i in iter {
//            v.append(i);
//        }
//    }
//}

pub type ExpressionBlock = Vec<Command>;

#[derive(Debug, Clone)]
pub enum ExpressionValue {
    Primitive(PrimitiveData),
    Expression(Command),
    Block(ExpressionBlock),
    Procedure(ArgumentList, ExpressionBlock),
    Unit,
}

pub fn is_linespace(chr: u8) -> bool { chr == b' ' || chr == b'\t' }

named!(pub space, take_while1!(is_space));
named!(pub linespace, take_while1!(is_linespace));
named!(pub keyword_show, alt!(tag!("show") | tag!("sh")));
named!(pub keyword_construct, alt!(tag!("construct") | tag!("cons")));
named!(pub keyword_delete, alt!(tag!("delete") | tag!("del")));
named!(pub keyword_put, alt!(tag!("put")));
named!(pub keyword_focus, alt!(tag!("focus") | tag!("fs")));

named!(pub editor_command_keyword<EditorCommandKeyword>,
    alt_complete!(
        keyword_show        => { |_| EditorCommandKeyword::Show } |
        keyword_construct   => { |_| EditorCommandKeyword::Construct } |
        keyword_delete      => { |_| EditorCommandKeyword::Delete } |
        keyword_put         => { |_| EditorCommandKeyword::Put } |
        keyword_focus       => { |_| EditorCommandKeyword::Focus }
    )
);

named!(pub sign, recognize!(opt!(one_of!("+-"))));
named!(pub integer_decimal_literal, recognize!(do_parse!(sign >> digit >> ())));
named!(pub integer_decimal<i64>,
    map_res!(
        map_res!(integer_decimal_literal, std::str::from_utf8),
        |s| i64::from_str_radix(s, 10)
    )
);
named!(pub boolean<bool>, alt!(
    tag!("true") => { |_| true } |
    tag!("false") => { |_| false }
));

named!(pub operator_initial<char>, one_of!(",.><+-=|^%~?"));
named!(pub operator_subsequent<char>, one_of!(":,.><+-=|^%~?"));
named!(pub operator_identifier<String>,
    do_parse!(
        initial: many_m_n!(1, 1, operator_initial) >>
        subsequent: many_m_n!(0, 32, operator_subsequent) >>
        (
            (||{
                initial.iter().chain(&subsequent).cloned().collect()
            })()
        )
    )
);

named!(pub
    string_content<String>,
    map!(
        escaped_transform!(
            take_until_either!("\"\\"),
            '\\',
            alt!(
                tag!("\\") => { |_| &b"\\"[..] } |
                tag!("\"") => { |_| &b"\""[..] } |
                tag!("n") => { |_| &b"\n"[..] } |
                tag!("r") => { |_| &b"\r"[..] } |
                tag!("t") => { |_| &b"\t"[..] }
            )
        ),
        to_s
    )
);
named!(pub primitive_value<PrimitiveData>,
    alt_complete!(
        integer_decimal     => { |i| PrimitiveData::Int(i) } |
        boolean             => { |b| PrimitiveData::Bool(b) } |
        string              => { |s| PrimitiveData::String(s) } |
        identifier          => { |n| PrimitiveData::Name(n) }
    )
);

named!(pub expression_value<ExpressionValue>,
    alt_complete!(
        procedure_expression => { |val| val } |
        block_expression => { |val| ExpressionValue::Block(val) } |
        primitive_value => { |val| ExpressionValue::Primitive(val) } |
        command_expression => { |val| ExpressionValue::Expression(val) }
    )
);

named!(pub command_argument_pair<(String, ExpressionValue)>,
   do_parse!(
        name: alt!(
            operator_identifier |
            terminated!(identifier, tag!(":"))
        ) >>
        linespace >>
        value: expression_value >>
        ((name, value))
   )
);

named!(pub command_arguments<Vec<(String, ExpressionValue)>>,
    many0!(alt_complete!(
        preceded!(opt!(linespace), command_argument_pair)
            => { |val| val } |
        preceded!(opt!(linespace), expression_value)
            => {|val| ("".to_owned(), val) }
    ))
);

named!(pub command<Command>, do_parse!(
    opt!(linespace) >>
    command_name: alt!(identifier | operator_identifier)  >>
    opt!(linespace) >>
    arguments: command_arguments >>
    opt!(linespace) >>
    peek!(one_of!("\r\n;)}")) >>
    (Command {name: command_name, args: arguments})
));

named!(pub command_expression<Command>,
    delimited!(char!('('), command, char!(')'))
);

named!(pub commands<Vec<Command>>, many0!(
    alt!(
        terminated!(command, one_of!("\r\n;")) |
        command_expression
    )
));

named!(pub expression<ExpressionValue>,
    alt_complete!(
        command =>          { |val| ExpressionValue::Expression(val) } |
        expression_value => { |val| val }
    )
);

named!(pub expressions<Vec<ExpressionValue>>, many0!(
    alt!(
        terminated!(expression, one_of!("\r\n;")) |
        expression
    )
));

named!(pub block_commands<Vec<Command>>,
    separated_list!(delimited!(opt!(linespace), one_of!("\r\n;"), opt!(linespace)), alt!(command | command_expression))
);

named!(pub block_expression<Vec<Command>>,
    delimited!(char!('{'), block_commands, char!('}')));

named!(pub argument_list<Vec<(String, ExpressionValue)>>,
    delimited!(char!('['), command_arguments, preceded!(opt!(linespace), char!(']'))));

named!(pub procedure_expression<ExpressionValue>,
    do_parse!(
        opt!(linespace) >>
        args: argument_list >>
        opt!(linespace) >>
        body: block_expression >>
        (ExpressionValue::Procedure(args, body))
    )
);

pub struct ShockEditorContext {
    level: u32
}

impl ShockEditorContext {
    pub fn new() -> Self {
        ShockEditorContext { level: 0 }
    }
}

pub fn parse(line: &str) ->
                            Result<(&[u8], Vec<ExpressionValue>), nom::Err<&[u8]>>
//                         Result<(&[u8], Vec<Command>), nom::Err<&[u8]>>
                         //Result<(&[u8], Command), nom::Err<&[u8]>>
{
    expressions(line.as_bytes())
//     commands(line.as_bytes())
//    command(line.as_bytes())
}

macro_rules! assert_correct_parse {
    ($parser: expr, $input: expr, $result: expr) => {
        assert_eq!(
            $parser($input.as_bytes()).ok().unwrap().1,
            $result
        );
    }
}

#[cfg(test)]
mod tests {
    
    #[test]
    fn it_works() {
        use crate::parser::identifier;
        use crate::parser::character;
        
        assert_eq!(identifier(b"abas ").ok().unwrap().1, "abas".to_owned());
        assert_eq!(identifier(b"--- ").is_err(), true);
        assert_eq!(character(b"\'a\'bc").ok().unwrap().1, "a".to_owned());
    }
   
    #[test]
    fn test_primitive_parsing() {
        use crate::parser::boolean;
        use crate::parser::integer_decimal;
        
        assert_correct_parse!(boolean, "true ", true);
        assert_correct_parse!(boolean, "false ", false);
        
        assert_correct_parse!(integer_decimal, "1 ", 1);
        assert_correct_parse!(integer_decimal, "101 ", 101);
        assert_correct_parse!(integer_decimal, "+101 ", 101);
        assert_correct_parse!(integer_decimal, "-101 ", -101);
        assert_correct_parse!(integer_decimal, "-0 ", 0);
    }
}

 



