extern crate shock;
extern crate rustyline;

use shock::parser::vec_to_string;

#[macro_use]
extern crate nom;

use shock::model::{Place, PrimitiveData, PlaceData};
use std::collections::HashMap;
use shock::model::Path;

use rustyline::error::ReadlineError;
use rustyline::Editor;
use shock::parser::{identifier, to_s};
use nom::{digit, is_space};

#[derive(Debug)]
enum EditorCommandKeyword {
    Show,
    Construct,
    Delete,
    Put,
    Focus
}

enum EditorCommand {
    SHOW(Path),
    CONSTRUCT(Path, PrimitiveData),
    DELETE(Path),
    UPDATE(Path, PrimitiveData),
    FOCUS(Path),
}

#[derive(Debug)]
struct Command {
    name: String,
    args: Vec<(String, PrimitiveData)>
}

fn is_linespace(chr: u8) -> bool { chr == b' ' || chr == b'\t' }

named!(space, take_while1!(is_space));
named!(linespace, take_while1!(is_linespace));
named!(keyword_show, alt!(tag!("show") | tag!("sh")));
named!(keyword_construct, alt!(tag!("construct") | tag!("cons")));
named!(keyword_delete, alt!(tag!("delete") | tag!("del")));
named!(keyword_put, alt!(tag!("put")));
named!(keyword_focus, alt!(tag!("focus") | tag!("fs")));

named!(editor_command_keyword<EditorCommandKeyword>,
    alt_complete!(
        keyword_show        => { |_| EditorCommandKeyword::Show } |
        keyword_construct   => { |_| EditorCommandKeyword::Construct } |
        keyword_delete      => { |_| EditorCommandKeyword::Delete } |
        keyword_put         => { |_| EditorCommandKeyword::Put } |
        keyword_focus       => { |_| EditorCommandKeyword::Focus }
    )
);

named!(sign, recognize!(opt!(one_of!("+-"))));
named!(integer_decimal_literal, recognize!(do_parse!(sign >> digit >> ())));
named!(integer_decimal<i64>,
    map_res!(
        map_res!(integer_decimal_literal, std::str::from_utf8),
        |s| i64::from_str_radix(s, 10)
    )
);
named!(boolean<bool>, alt!(
    tag!("true") => { |_| true } |
    tag!("false") => { |_| false }
));

named!(operator_initial<char>, one_of!(",.:><+-=|^%~?"));
named!(operator_identifier<String>,
    do_parse!(
        id: many_m_n!(1, 32, operator_initial) >>
        (id.into_iter().collect())
    )
);

named!(
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
named!(string<String>, delimited!(tag!("\""), string_content, tag!("\"")));

named!(primitive_value<PrimitiveData>,
    alt_complete!(
        integer_decimal     => { |i| PrimitiveData::Int(i) } |
        boolean             => { |b| PrimitiveData::Bool(b) } |
        string              => { |s| PrimitiveData::String(s) } |
        identifier          => { |n| PrimitiveData::Name(n) }
    )
);

named!(command_argument_pair<(String, PrimitiveData)>,
   do_parse!(
        opt!(linespace) >>
        name: alt!(
            terminated!(identifier, tag!(":")) |
            operator_identifier
        ) >>
        linespace >>
        value: primitive_value >>
        ((name, value))
   )
);

named!(command_arguments<Vec<(String, PrimitiveData)>>,
    many1!(alt_complete!(
        preceded!(opt!(linespace), primitive_value)  => { |val| ("".to_owned(), val) } |
        command_argument_pair                    => { |val| val }
    ))
);

named!(command<Command>, do_parse!(
    opt!(linespace) >>
    command_name: identifier >>
    arguments: command_arguments >>
    opt!(linespace) >>
    one_of!("\t\r\n;") >>
    (Command {name: command_name, args: arguments})
));

named!(commands<Vec<Command>>, many1!(
    command
));


struct ShockEditorContext {
    level: u32
}

fn parse(line: &str) -> Result<(&[u8], Vec<Command>), nom::Err<&[u8]>> {
     commands(line.as_bytes())
}

macro_rules! assert_correct_parse {
    ($parser: expr, $input: expr, $result: expr) => {
        assert_eq!(
            $parser($input.as_bytes()).ok().unwrap().1,
            $result
        );
    }
}


fn main() {
    assert_correct_parse!(boolean, "true ", true);
    assert_correct_parse!(boolean, "false ", false);
    
    assert_correct_parse!(integer_decimal, "1 ", 1);
    assert_correct_parse!(integer_decimal, "101 ", 101);
    assert_correct_parse!(integer_decimal, "+101 ", 101);
    assert_correct_parse!(integer_decimal, "-101 ", -101);
    assert_correct_parse!(integer_decimal, "-0 ", 0);
   
    let value = primitive_value("123 ".as_bytes());
    println!("{:?}", value);
    
    println!("Welcome to Shock 0.1.0.");
    println!("Initializing editor...");
    let mut editor = Editor::<()>::new();
    println!("Initialized editor.");
    println!("Loading history...");
    if editor.load_history(".shock-history").is_err() {
        println!("No previous history loaded.")
    } else {
        println!("Loaded history.");
    }
    
    let mut context = ShockEditorContext {level: 0};
    
    loop {
        let mut line = editor.readline(">> ");
        
        match &mut line {
            Ok(line) => {
                line.push('\n');
                line.push('\n');
                //println!("{:?}", line);
                editor.add_history_entry(line.as_ref());
                let result = parse(&line);
                println!("{:?}", result.map(|val| { val.1 }));
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }
    println!("{}", match editor.save_history(".shock-history") {
        Ok(_) => "Saved history.",
        Err(_) => "Error. Could not save history.",
    });
    
    
    let mut p = Place::new("root".to_string(), HashMap::new());
    
    println!("{:?}", p);
    
    p.put_attr("type".to_string(), PlaceData::Data(PrimitiveData::String("Place".to_string())));
    
    println!("{:?}", p);
    
    
}