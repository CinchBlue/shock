use std::collections::HashMap;
use std::borrow::Borrow;

use crate::model::PrimitiveData;
use crate::parser::{ExpressionValue, Command};
use std::sync::Arc;
use std::sync::Mutex;

#[derive(Debug, Clone)]
pub struct Procedure<'a> {
    pub argnames: Vec<String>,
    pub body: Vec<Command>,
    pub parent_scope: &'a Option<VMScope<'a>>,
}

#[derive(Debug, Clone)]
pub enum Value<'a> {
    Unit,
    Primitive(PrimitiveData),
    Procedure(Procedure<'a>),
}

#[derive(Debug)]
pub struct VMScope<'a> {
    pub vars:   Arc<Mutex<HashMap<String, Value<'a>>>>,
    pub args:   Vec<(String, Value<'a>)>,
    pub acc:    Value<'a>,
    pub parent: &'a Option<VMScope<'a>>,
    pub child:  Box<Option<VMScope<'a>>>,
}

impl <'x> VMScope<'x> {
    /// Gets a variable from the current scope or checks from the parent.
    pub fn lookup_value<'a, 'b>(&'a self, name: &'b str) -> Option<Value<'x>> {
        match self.vars.lock().unwrap().get(name) {
            None => match self.parent {
                None => None,
                Some(parent) => parent.lookup_value(name)
            },
            Some(val) => Some(val.clone()),
        }
    }
}

pub struct VM<'a> {
    pub curr_scope: VMScope<'a>,
    pub curr_expr: ExpressionValue,
}

pub fn eval<'a, 'b, 'c>(vm: &'a Arc<Mutex<VM<'a>>>, expr: &'b ExpressionValue) -> Value<'a> {
    match expr {
        ExpressionValue::Primitive(primitive_data) => {
            Value::Primitive(primitive_data.clone())
        },
        ExpressionValue::Procedure(args, commands) => {
            let x = args;
            Value::Unit
        },
        ExpressionValue::Unit => {
            Value::Unit
        },
        ExpressionValue::Expression(command) => {
            if command.name == "let" && command.args.len() == 2 {
                let var_name = extract_raw_name(command, 0);
                
                let var_value = &command.args.get(1).unwrap().1;
                let val = eval(vm, &var_value);
                println!("name: {:?} val: {:?}", &var_name, &val);
                vm.lock().unwrap().curr_scope.vars.lock().unwrap().insert(var_name, val.clone());
                val.clone()
            } else if command.name == "show" {
                for (name, value) in vm.lock().unwrap().curr_scope.vars.lock().unwrap().iter() {
                    println!("{:?}\t:\t{:?}", name, value);
                }
                Value::Unit
            } else if command.name == "get" && command.args.len() == 1 {
                let var_name = extract_raw_name(command, 0);
                match vm.lock().unwrap().curr_scope.lookup_value(var_name.as_ref()) {
                    None => { Value::Unit },
                    Some(val) => { val.clone() },
                }
            } else {
                Value::Unit
            }
        },
        ExpressionValue::Block(_) => {
            Value::Unit
        }
    }
}



fn extract_raw_name(command: &Command, pos: usize) -> String {
    let mut var_name = command.args.get(pos).unwrap().0.clone();
    
    if var_name == "" {
        let var_name_data = command.args.get(pos).unwrap().1.clone();
        match var_name_data {
            ExpressionValue::Primitive(data) => match data {
                PrimitiveData::Name(s) => {
                    var_name = s;
                },
                _ => {}
            },
            _ => {}
        }
    }
    
    var_name
}
