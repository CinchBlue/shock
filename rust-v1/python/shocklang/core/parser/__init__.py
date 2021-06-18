import grako

GRAMMAR = '''
@@grammar::Shocklang

start = {WS} @+:Expression {{WS} @+:Expression}* $ ;

WS = /[ \t]/ ;
NL = /[;\r\n]/ ;

Integer = /[0-9]+/ ;
Float = /[-+]?[0-9]+[.]?[0-9]*([eE][-+]?[0-9]+)?/ ;
String = /\"(\\.|[^\"])*\"/ ;
Character = /\'(\\.|[^\"])*\'/ ;
VariableName = /[a-z][A-z0-9]*/ ;
TypeName = /[A-Z][A-z0-9]*/ ;
BoundValue = '_' ;

Literal
    = 
    | float:Float
    | integer:Integer
    | string:String
    | character:Character
    | variable_name:VariableName
    | type_name:TypeName
    | bound_value:BoundValue
    ;

Expression    
    =
    | Literal
    | List
    | Block
    ;
    
LabeledExpression
    = VariableName ':' {WS}+ Expression ;
    
List 
    = '[' {WS} @+:Expression {{WS}+ @+:Expression}* {WS} ']' ;
    
Block
    = '{' {WS} @+:Expression {{WS} {NL} {WS} @+:Expression}* {WS} '}' ;
    
Struct
    = '{' {WS} @+:LabeledExpression {{WS}     
'''



parser = grako.compile(GRAMMAR)
