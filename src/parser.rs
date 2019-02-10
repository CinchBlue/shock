use nom::{take_while1, is_digit, anychar};

use crate::model::{PrimitiveData};

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

pub fn is_identifier_initial(chr: u8) -> bool {
    chr == b'_' || chr.is_ascii_alphabetic()
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
    take_while_m_n!(1, 1, is_identifier_initial));

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


#[derive(Debug, Clone, PartialEq, Default)]
pub struct Command {
    pub name: String,
    pub args: Vec<(String, ExpressionValue)>,
}

pub type ArgumentList = Vec<(String, ExpressionValue)>;

pub type ExpressionBlock = Vec<Command>;

#[derive(Debug, Clone, PartialEq)]
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

named!(pub operator_initial<char>, one_of!(",.><+-=|^%~?*/"));
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
        identifier          => { |n| PrimitiveData::Name(n) } |
        integer_decimal     => { |i| PrimitiveData::Int(i) } |
        boolean             => { |b| PrimitiveData::Bool(b) } |
        string              => { |s| PrimitiveData::String(s) }
    )
);

named!(pub expression_value<ExpressionValue>,
    alt_complete!(
        procedure_expression => { |val| val } |
        block_expression => { |val| ExpressionValue::Block(val) } |
        command_expression => { |val| ExpressionValue::Expression(val) } |
        primitive_value => { |val| ExpressionValue::Primitive(val) }
    )
);

named!(pub command_argument_pair<(String, ExpressionValue)>,
   do_parse!(
        name: alt!(
            terminated!(identifier, tag!(":")) |
            operator_identifier
        ) >>
        linespace >>
        value: expression_value >>
        ((name, value))
   )
);

named!(pub command_arguments<Vec<(String, ExpressionValue)>>,
    separated_list!(linespace, alt_complete!(
        preceded!(opt!(linespace), command_argument_pair)
            => { |val| val } |
        preceded!(opt!(linespace), expression_value)
            => {|val| ("".to_owned(), val) }
    ))
);

named!(pub argument_list<Vec<(String, ExpressionValue)>>,
    delimited!(
        char!('['),
        many0!(preceded!(opt!(linespace), command_argument_pair)),
        char!(']')
    ));

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

named!(pub procedure_expression<ExpressionValue>,
    do_parse!(
        opt!(linespace) >>
        args: argument_list >>
        opt!(linespace) >>
        body: block_expression >>
        (ExpressionValue::Procedure(args, body))
    )
);

pub fn parse(line: &str)
    -> Result<(&[u8], Vec<ExpressionValue>), nom::Err<&[u8]>>
{
    expressions(line.as_bytes())
}



#[cfg(test)]
mod tests {
    
    macro_rules! assert_correct_parse {
        ($parser: expr, $input: expr, $result: expr) => {
            let parsed_value = $parser($input.as_bytes());
            let original_value = parsed_value.clone();
            let parsed_value = parsed_value.ok();
            assert_eq!(true, parsed_value.is_some(), "parsed value did not parse correctly: {:?}", original_value);
            assert_eq!(
                $result,
                parsed_value.unwrap().1,
            );
        }
    }
    
    #[test]
    fn it_works() {
        use crate::parser::identifier;
        use crate::parser::character;
       
        assert_correct_parse!(identifier, "x ", "x".to_owned());
        assert_correct_parse!(identifier, "abas ", "abas".to_owned());
        assert_correct_parse!(character, "\'a\'bc ", "a".to_owned());
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
    
    #[test]
    fn test_argument_list_parsing() {
        use crate::parser::command_argument_pair;
        use crate::parser::command_arguments;
        use crate::parser::argument_list;
        use crate::parser::ExpressionValue;
        use crate::model::PrimitiveData;
        
        assert_correct_parse!(
            command_argument_pair,
            "x: Int ",
            ("x".to_owned(), ExpressionValue::Primitive(PrimitiveData::Name("Int".to_owned())))
        );
        assert_correct_parse!(
            command_arguments,
            "x: Int ",
            vec![("x".to_owned(), ExpressionValue::Primitive(PrimitiveData::Name("Int".to_owned())))]
        );
        assert_correct_parse!(
            argument_list,
            "[x: Int] ",
            vec![("x".to_owned(), ExpressionValue::Primitive(PrimitiveData::Name("Int".to_owned())))]
        );
        assert_correct_parse!(
            argument_list,
            "[x: Int y: String] ",
            vec![
                ("x".to_owned(), ExpressionValue::Primitive(PrimitiveData::Name("Int".to_owned()))),
                ("y".to_owned(), ExpressionValue::Primitive(PrimitiveData::Name("String".to_owned())))
            ]
        );
        assert_correct_parse!(
            argument_list,
            "[x: Int y: String -> String] ",
            vec![
                ("x".to_owned(), ExpressionValue::Primitive(PrimitiveData::Name("Int".to_owned()))),
                ("y".to_owned(), ExpressionValue::Primitive(PrimitiveData::Name("String".to_owned()))),
                ("->".to_owned(), ExpressionValue::Primitive(PrimitiveData::Name("String".to_owned())))
            ]
        );
    }
    
    #[test]
    fn test_command() {
        use crate::parser::command;
        use crate::parser::ExpressionValue;
        use crate::parser::Command;
        use crate::model::PrimitiveData;
        
        assert_correct_parse!(
            command,
            "let x = (+ 1 1) \n",
            Command {
                name: "let".to_owned(),
                args: vec![
                    ("".to_owned(), ExpressionValue::Primitive(PrimitiveData::Name("x".to_owned()))),
                    ("=".to_owned(), ExpressionValue::Expression(
                        Command {
                            name: "+".to_owned(),
                            args: vec![
                                ("".to_owned(), ExpressionValue::Primitive(PrimitiveData::Int(1))),
                                ("".to_owned(), ExpressionValue::Primitive(PrimitiveData::Int(1)))
                            ]
                        }
                    ))
                ]
            }
        );
        
    
        assert_correct_parse!(
            command,
            "let x = [x: Int]{} \n",
            Command {
                name: "let".to_owned(),
                args: vec![
                    ("".to_owned(), ExpressionValue::Primitive(PrimitiveData::Name("x".to_owned()))),
                    ("=".to_owned(), ExpressionValue::Procedure(
                        vec![
                            ("x".to_owned(), ExpressionValue::Primitive(PrimitiveData::Name("Int".to_owned())))
                        ],
                        vec![]
                    ))
                ]
            }
        );
    }
    
    
    #[test]
    fn test_expression_value() {
        use crate::parser::expression;
        use crate::parser::ExpressionValue;
        use crate::parser::Command;
        use crate::model::PrimitiveData;
        
        assert_correct_parse!(
            expression,
            "[x: Int]{} ",
            ExpressionValue::Procedure(
                vec![
                    ("x".to_owned(),
                        ExpressionValue::Primitive(PrimitiveData::Name("Int".to_owned())))
                ],
                vec![]
            )
        );
        assert_correct_parse!(
            expression,
            "(let x = 1) ",
            ExpressionValue::Expression(
                Command {
                    name: "let".to_owned(),
                    args: vec![
                        ("".to_owned(), ExpressionValue::Primitive(PrimitiveData::Name("x".to_owned()))),
                        ("=".to_owned(), ExpressionValue::Primitive(PrimitiveData::Int(1)))
                    ]
                }
            )
        );
       
    }
}

 



