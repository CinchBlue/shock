extern crate shock;
extern crate rustyline;

use shock::parser::{ExpressionValue, primitive_value, boolean, integer_decimal, ShockEditorContext, parse,
                    vec_to_string};
use shock::model::{PrimitiveData, PlaceData, Place};
use shock::interpreter::{VM, VMScope, Value, eval};

use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::collections::HashMap;
use std::sync::Mutex;
use std::sync::Arc;

#[macro_use]
extern crate nom;

fn main() {

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
    
    let mut context = ShockEditorContext::new();
    let mut vm = Arc::new(Mutex::new(VM {
        curr_scope: VMScope {
            vars: Arc::new(Mutex::new(HashMap::new())),
            args: Vec::new(),
            acc: Value::Unit,
            parent: &None,
            child: Box::new(None),
        },
        curr_expr: ExpressionValue::Unit,
    }));
    
    loop {
        let mut line = editor.readline(">> ");
        
        match &mut line {
            Ok(line) => {
                editor.add_history_entry(line.as_ref());
                line.push('\n');
                line.push('\n');
                //println!("{:?}", line);
                let mut result = parse(&line).map(
                    |val| { val.1 }
                ).unwrap();
                println!("PARSED: {:?}", result.get(0));
                let mut value = eval(&vm, result.get(0).unwrap());
                println!("EVAL: {:?}", value);
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