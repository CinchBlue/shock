use std::collections::HashMap;

use crate::model::PrimitiveData;
use crate::parser::{ExpressionValue, Command};
use std::sync::Arc;
use std::sync::Mutex;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt;
use std::mem;

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
        // First, temporarily bind the variables in the current scope.
        for binding in args.iter() {
            vm.lock().unwrap().curr_scope.lock().unwrap().set_value(
                &binding.0, &binding.1);
        }
        // Evaluate each expression in the block, return Unit if empty.
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
    Path(Vec<String>),
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
    
    pub fn lookup_direct_value(&self, name: &str) -> Option<Value> {
        self.vars.lock().unwrap().get(name).cloned()
    }
    
    pub fn set_value(&self, name: &str, val: &Value) {
        self.vars.lock().unwrap().insert(name.to_string(), val.clone());
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
//        println!("LET name: {:?} val: {:?}", &var_name, &val);
        vm.lock().unwrap().curr_scope.lock().unwrap().vars.lock().unwrap().insert(var_name, val.clone());
        val.clone()
    }
   
    #[allow(unused_variables)]
    pub fn shock_show(args: Vec<(String, Value)>, vm: &Arc<Mutex<VM>>) -> Value {
//        println!("SHOW:");
        for (name, value) in vm.lock().unwrap().curr_scope.lock().unwrap().vars.lock().unwrap().iter() {
            println!("{:?}\t:\t{:?}", name, value);
        }
        Value::Unit
    }
    
    pub fn shock_get(args: Vec<(String, Value)>, vm: &Arc<Mutex<VM>>) -> Value {
        if args.len() < 0 || args.len() >= 2 { return Value::Unit; }
        let var_name = extract_first_argname(&args, 0);
        println!("GET {:?}", var_name);
        match vm.lock().unwrap().curr_scope.lock().unwrap().lookup_value(var_name.as_ref()) {
            None => { Value::Unit },
            Some(val) => { val.clone() },
        }
    }
    
    fn to_float(value: &Value, vm: &Arc<Mutex<VM>>) -> f64 {
            use crate::interpreter::nativelib::shock_get;
            use crate::model::PrimitiveData;
           
            match value {
                Value::Primitive(primitive) => match primitive {
                    PrimitiveData::Name(v) => {
                        let val = shock_get(
                            vec![(v.to_string(), Value::Unit)],
                            vm);
                        to_float(&val, vm)
                    },
                    PrimitiveData::Bool(v) => if *v { 1.0 } else { 0.0 },
                    PrimitiveData::Byte(v) => *v as f64,
                    PrimitiveData::Int(v) => *v as f64,
                    PrimitiveData::Float(v) => *v as f64,
                    _ => 0.0,
                },
                Value::Path(path_components) => {
                    match vm.lock().unwrap().lookup_path(path_components) {
                        None => 0.0,
                        Some(value) => to_float(&value, vm),
                    }
                },
                _ => 0.0
            }
        }
    
    pub mod arith {
        use crate::interpreter::{VM, Value};
        use std::sync::{Arc, Mutex};
        use crate::model::PrimitiveData;
    
        
        
        macro_rules! numeric_op_impl {
            ($name: ident, $op: tt) => {
                pub fn $name(args: Vec<(String, Value)>, vm: &Arc<Mutex<VM>>) -> Value {
                    use crate::interpreter::nativelib::to_float;
                
                    if args.len() == 0 { return Value::Primitive(PrimitiveData::Int(0)) }
                    let (first, rest) = args.split_at(1);
                    println!("first: {:?}, rest: {:?}", first, rest);
                    let mut acc = to_float(&first[0].1, vm);
                    for expr in rest {
                        acc = acc $op to_float(&expr.1, vm);
                    }
                    
                    if acc.is_normal() && math::round::ceil(acc, 0) == math::round::floor(acc, 0) {
                        Value::Primitive(PrimitiveData::Int(acc as i64))
                    } else {
                        Value::Primitive(PrimitiveData::Float(acc))
                    }
                }
            }
        }
        
        numeric_op_impl!(add, +);
        numeric_op_impl!(sub, -);
        numeric_op_impl!(mult, *);
        numeric_op_impl!(div, /);
        numeric_op_impl!(modulo, %);
    }
}

impl VM {
    pub fn define_standard_functions(&mut self) {
        let scope = self.curr_scope.lock().unwrap();
        let mut bindings = scope.vars.lock().unwrap();
        bindings.insert("let".to_owned(), Value::NativeProcedure(NativeProcedure::new (nativelib::shock_let)));
        bindings.insert("show".to_owned(), Value::NativeProcedure(NativeProcedure::new (nativelib::shock_show)));
        bindings.insert("get".to_owned(), Value::NativeProcedure(NativeProcedure::new (nativelib::shock_get)));
        bindings.insert("+".to_owned(), Value::NativeProcedure(NativeProcedure::new (nativelib::arith::add)));
        bindings.insert("-".to_owned(), Value::NativeProcedure(NativeProcedure::new (nativelib::arith::sub)));
        bindings.insert("*".to_owned(), Value::NativeProcedure(NativeProcedure::new (nativelib::arith::mult)));
        bindings.insert("/".to_owned(), Value::NativeProcedure(NativeProcedure::new (nativelib::arith::div)));
        bindings.insert("%".to_owned(), Value::NativeProcedure(NativeProcedure::new (nativelib::arith::modulo)));
    }

    pub fn push_scope(&mut self) {
        let mut current_scope = self.curr_scope.clone();
        if current_scope.lock().unwrap().child.is_none() {
            current_scope.lock().unwrap().child = Some(Arc::new(Mutex::new(VMScope::new(
                Some(current_scope.clone())
            ))));
            mem::replace(&mut self.curr_scope, current_scope.lock().unwrap().child.clone().unwrap().clone());
        } else {
            panic!("Cannot push a scope to the scope stack because the child is already there!");
        }
    }
    
    pub fn pop_scope(&mut self) {
        let mut current_scope = self.curr_scope.clone();
        if current_scope.lock().unwrap().parent.is_some() {
            mem::replace(&mut self.curr_scope, current_scope.lock().unwrap().parent.clone().unwrap().clone());
            mem::replace(&mut self.curr_scope.lock().unwrap().child, None);
        } else {
            panic!("Cannot pop a scope from the scope stack because there is no scopes to pop!");
        }
    }
    
    pub fn get_parent_scope(&self) -> Option<Arc<Mutex<VMScope>>> {
        self.curr_scope.lock().unwrap().parent.clone()
    }
    
    pub fn get_current_scope(&self) -> Arc<Mutex<VMScope>> {
        self.curr_scope.clone()
    }

    pub fn lookup_direct_value(&self, name: &str) -> Option<Value> {
        self.curr_scope.lock().unwrap().lookup_direct_value(name)
    }
    
    pub fn lookup_path(&self, path: &Vec<String>) -> Option<&Value> {
        let mut final_value = None;
        let mut current_scope = self.get_current_scope();
        for part in path {
            if part.is_empty() {
                let parent_scope = self.get_parent_scope();
                match parent_scope {
                    None => return None,
                    Some(scope) => current_scope = scope,
                }
            } else {
                let final_value = self.lookup_direct_value(&part);
                match final_value {
                    None => return None,
                    Some(value) => match value {
                        Value::Procedure(procedure) =>
                            current_scope = procedure.scope.clone(),
                        _ => return None,
                    },
                }
            }
        }
        final_value
    }
}

pub fn eval(
    vm: &Arc<Mutex<VM>>,
    expr: &ExpressionValue) -> Value {
    eval_impl(vm, expr, true)
}

pub fn eval_without_dereferencing(
    vm: &Arc<Mutex<VM>>,
    expr: &ExpressionValue) -> Value {
    eval_impl(vm, expr, false)
}

pub fn eval_impl(
    vm: &Arc<Mutex<VM>>,
    expr: &ExpressionValue,
    reference_variables: bool) -> Value {
    println!("EVAL with referencing = {} on {:?}", reference_variables, expr);
    match expr {
        ExpressionValue::Path(path_components) => {
            println!("Path: {:?}", path_components);
            if reference_variables {
                match vm.lock().unwrap().lookup_path(path_components) {
                    None => Value::Unit,
                    Some(value) => value.clone(),
                }
            } else {
                Value::Path(path_components.clone())
            }
        },
        ExpressionValue::Primitive(primitive_data) => {
            println!("Primitive: {:?}", primitive_data);
            match primitive_data {
                PrimitiveData::Name(name) => if reference_variables {
                    nativelib::shock_get(vec![(name.to_string(), Value::Unit)], vm)
                } else {
                    Value::Primitive(primitive_data.clone())
                },
                _ => Value::Primitive(primitive_data.clone())
            }
        },
        ExpressionValue::Procedure(args, commands) => {
            println!("Procedure...");
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
                                |x| (x.0.clone(), eval_without_dereferencing(vm, &x.1))
                            ).collect(), vm)
                        },
                        Value::Procedure(mut proc) => {
                            vm.lock().unwrap().push_scope();
                            let value = proc.apply(command.args.iter().map(
                                |x| (x.0.clone(), eval(vm, &x.1))
                            ).collect(), vm);
                            vm.lock().unwrap().pop_scope();
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
        ExpressionValue::Block(commands) => {
            println!("Block... ");
            vm.lock().unwrap().push_scope();
            let mut final_value = Value::Unit;
            for command in commands {
                final_value = eval(&vm, &ExpressionValue::Expression(command.clone()));
            }
            vm.lock().unwrap().pop_scope();
            return final_value;
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

