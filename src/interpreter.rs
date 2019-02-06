use std::collections::HashMap;

use crate::model::PrimitiveData;
use crate::parser::{ExpressionValue, Command};
use std::sync::Arc;
use std::sync::Mutex;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt;

pub trait Applicable {
    fn apply(&mut self, args: Vec<(String, Value)>, vm: &Arc<Mutex<VM>>) -> Value;
}

#[derive(Debug, Clone)]
pub struct Procedure {
    pub argnames: Vec<String>,
    pub body: Vec<Command>,
    pub scope: Arc<Mutex<VMScope>>,
}

impl Applicable for Procedure {
    fn apply(&mut self, args: Vec<(String, Value)>, vm: &Arc<Mutex<VM>>) -> Value {
        let mut value : Value = Value::Unit;
        for command in self.body.iter() {
            value = eval(vm, &ExpressionValue::Expression(command.clone()));
        }
        value
    }
}

#[derive(Clone)]
pub struct NativeProcedure {
    pub proc: fn(Vec<(String, Value)>, &Arc<Mutex<VM>>) -> Value,
}

impl NativeProcedure {
    fn new(proc: fn(Vec<(String, Value)>, &Arc<Mutex<VM>>) -> Value) -> Self {
        NativeProcedure { proc: proc }
    }
}

impl Debug for NativeProcedure {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "NativeProcedure {{ proc: {:p} }}", self.proc as *const ())
    }
}

impl Applicable for NativeProcedure {
    fn apply(&mut self, args: Vec<(String, Value)>, vm: &'_ Arc<Mutex<VM>>) -> Value { (self.proc)(args, vm) }
}

#[derive(Debug, Clone)]
pub enum Value {
    Unit,
    Primitive(PrimitiveData),
    Procedure(Procedure),
    NativeProcedure(NativeProcedure),
}


#[derive(Debug)]
pub struct VMScope {
    pub vars:   Arc<Mutex<HashMap<String, Value>>>,
    pub args:   Vec<(String, Value)>,
    pub acc:    Value,
    pub parent: Option<Arc<Mutex<VMScope>>>,
    pub child:  Option<Arc<Mutex<VMScope>>>,
}

impl VMScope {
    pub fn new(parent: Option<Arc<Mutex<VMScope>>>) -> VMScope {
        VMScope {
            vars: Arc::new(Mutex::new(HashMap::new())),
            args: Vec::new(),
            acc: Value::Unit,
            parent: parent,
            child: None,
        }
    }
    
    /// Gets a variable from the current scope or checks from the parent.
    pub fn lookup_value(&self, name: &str) -> Option<Value> {
        match self.vars.lock().unwrap().get(name) {
            None => match &self.parent {
                None => None,
                Some(parent) => parent.lock().unwrap().lookup_value(name)
            },
            Some(val) => Some(val.clone()),
        }
    }
    
    pub fn push_scope(&mut self, parent: Arc<Mutex<VMScope>>) {
        if self.child.is_none() {
            self.child = Some(Arc::new(Mutex::new(VMScope::new(
                Some(parent.clone())
            ))));
        } else {
            panic!("Cannot push a scope to the scope stack because the child is already there!");
        }
    }
    
    pub fn pop_scope(&mut self) {
        if self.child.is_some() {
            self.child = None;
        } else {
            panic!("Cannot pop a scope to the scope stack because there is no child!");
        }
    }
}

pub struct VM {
    pub curr_scope: Arc<Mutex<VMScope>>,
    pub curr_expr: ExpressionValue,
}

mod nativelib {
    use std::sync::Arc;
    use std::sync::Mutex;
    use crate::interpreter::{VM, Value, extract_first_argname};
    
    pub fn shock_let(args: Vec<(String, Value)>, vm: &Arc<Mutex<VM>>) -> Value {
        if args.len() < 2 {
            println!("LET requires two arguments.");
            return Value::Unit;
        }
        let var_name = extract_first_argname(&args, 0);
    
        let val = &args.get(1).unwrap().1;
        println!("name: {:?} val: {:?}", &var_name, &val);
        vm.lock().unwrap().curr_scope.lock().unwrap().vars.lock().unwrap().insert(var_name, val.clone());
        val.clone()
    }
    
    pub fn shock_show(args: Vec<(String, Value)>, vm: &Arc<Mutex<VM>>) -> Value {
        for (name, value) in vm.lock().unwrap().curr_scope.lock().unwrap().vars.lock().unwrap().iter() {
            println!("{:?}\t:\t{:?}", name, value);
        }
        Value::Unit
    }
    
    pub fn shock_get(args: Vec<(String, Value)>, vm: &Arc<Mutex<VM>>) -> Value {
        let var_name = extract_first_argname(&args, 0);
        match vm.lock().unwrap().curr_scope.lock().unwrap().lookup_value(var_name.as_ref()) {
            None => { Value::Unit },
            Some(val) => { val.clone() },
        }
    }
}

impl VM {
    pub fn define_standard_functions(&mut self) {
        let mut scope = self.curr_scope.lock().unwrap();
        let mut bindings = scope.vars.lock().unwrap();
        bindings.insert("let".to_owned(), Value::NativeProcedure(NativeProcedure::new (nativelib::shock_let)));
        bindings.insert("show".to_owned(), Value::NativeProcedure(NativeProcedure::new (nativelib::shock_show)));
        bindings.insert("get".to_owned(), Value::NativeProcedure(NativeProcedure::new (nativelib::shock_get)));
    }
}

pub fn eval<'a, 'b, 'c>(
    vm: &'a Arc<Mutex<VM>>,
    expr: &'b ExpressionValue) -> Value {
    //println!("EVAL");
    match expr {
        ExpressionValue::Primitive(primitive_data) => {
            //println!("Primitive: {:?}", primitive_data);
            Value::Primitive(primitive_data.clone())
        },
        ExpressionValue::Procedure(args, commands) => {
            //println!("Procedure...");
            let x = args;
            Value::Procedure(Procedure{
                argnames: args.iter().map(|val| val.0.clone()).collect(),
                body: commands.clone(),
                scope: vm.lock().unwrap().curr_scope.clone(),
            })
        },
        ExpressionValue::Unit => {
            Value::Unit
        },
        ExpressionValue::Expression(command) => {
            let looked_up_value = vm.lock().unwrap().curr_scope.lock().unwrap().lookup_value(&command.name);
            match looked_up_value {
                None => {
                    println!("Could not find procedure.");
                    Value::Unit
                },
                Some(val) => {
                    match val {
                        Value::NativeProcedure(mut natproc) => {
                            natproc.apply(command.args.iter().map(
                                |x| (x.0.clone(), eval(vm, &x.1))
                            ).collect(), vm)
                        },
                        Value::Procedure(mut proc) => {
                            let mut vm_mutex = vm.lock().unwrap();
                            vm_mutex.curr_scope.lock().unwrap().push_scope(vm_mutex.curr_scope.clone());
                            let value = proc.apply(command.args.iter().map(
                                |x| (x.0.clone(), eval(vm, &x.1))
                            ).collect(), vm);
                            vm_mutex.curr_scope.lock().unwrap().pop_scope();
                            value
                        },
                        _ => {
                            println!("Cannot apply a non-procedure in a command.");
                            Value::Unit
                        }
                    }
                },
            }
        },
        ExpressionValue::Block(_) => {
            Value::Unit
        }
    }
}

fn extract_raw_name(args: &Vec<(String, ExpressionValue)>, pos: usize) -> String {
    let mut var_name = args.get(pos).unwrap().0.clone();
    if var_name == "" {
        let var_name_data = args.get(pos).unwrap().1.clone();
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

fn extract_first_argname(args: &Vec<(String, Value)>, pos: usize) -> String {
    let mut var_name = args.get(pos).unwrap().0.clone();
    if var_name == "" {
        let var_name_data = args.get(pos).unwrap().1.clone();
        match var_name_data {
            Value::Primitive(data) => match data {
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

