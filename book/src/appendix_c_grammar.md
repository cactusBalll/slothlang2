# 附录 C. 实际文法（EBNF）

下列文法直接从 `sloth-frontend` 的 lexer/parser 提炼，反映**当前实现**（与设计
文档 §3.9 的差异见[附录 A](appendix_a_deviations.md)）。

```ebnf
prog            ::= ( import_decl | decl | stmt )*

import_decl     ::= 'import' STRING ( 'as' IDENT )? ';'

decl            ::= 'pub'? ( func_decl | var_let_decl | class_decl
                           | trait_decl | extern_decl )

var_let_decl    ::= ( 'var' | 'let' ) IDENT ( ':' type )? '=' expr ';'

func_decl       ::= 'func' IDENT type_params? '(' params ')' ret_ann? block
ret_ann         ::= ( ':' | '->' ) type
type_params     ::= '<' IDENT ( ':' IDENT )? ( ',' IDENT ( ':' IDENT )? )* '>'
params          ::= ( param ( ',' param )* ( ',' variadic )? | variadic )?
param           ::= IDENT ( ':' type )?
variadic        ::= IDENT '...' ':' 'Array' '<' type '>'

extern_decl     ::= 'extern' 'type' IDENT ';'
                  | 'extern' 'func' IDENT '(' params ')' ret_ann? ';'
                    (* 无泛型、无可变参、参数必须有类型 *)

class_decl      ::= 'class' IDENT type_params? ( ':' IDENT )?
                    ( 'impl' IDENT ( ',' IDENT )* )?
                    '{' class_item* '}'
class_item      ::= 'pub'? ( field_decl | func_decl )
field_decl      ::= ( 'var' | 'let' ) IDENT ':' type ( '=' expr )? ';'

trait_decl      ::= 'trait' IDENT '{' trait_method* '}'
trait_method    ::= 'func' IDENT '(' params ')' ':' type
                    ( block | ';' )                 (* block = 默认实现 *)

type            ::= type_base '?'?                      (* T?? 报错 *)
type_base       ::= 'unit' | 'int' | 'float' | 'bool' | 'str' | 'range'
                  | 'Array' '<' type '>'
                  | 'Map' '<' type ',' type '>'
                  | 'dyn' IDENT
                  | '(' ( type ( ',' type )* )? ')' '->' type
                  | IDENT type_args?
type_args       ::= '<' type ( ',' type )* '>'

block           ::= '{' stmt* '}'
stmt            ::= block
                  | var_let_decl
                  | if_stmt | while_stmt | for_stmt
                  | 'return' expr? ';'
                  | 'break' ';' | 'continue' ';'
                  | assign_stmt | expr_stmt

if_stmt         ::= 'if' ( '(' expr ')' | expr ) stmt ( 'else' stmt )?
while_stmt      ::= 'while' ( '(' expr ')' | expr ) stmt
for_stmt        ::= 'for' ( '(' 'var' IDENT ':' expr ')' | IDENT 'in' expr ) stmt
assign_stmt     ::= assignable '=' expr ';'
assignable      ::= IDENT ( '.' IDENT | '[' expr ']' )*
expr_stmt       ::= expr ';'

(* 表达式按 Pratt 解析，优先级 低 -> 高 *)
expr            ::= pipe
pipe            ::= elvis ( '|>' elvis )*                 (* 左结合 *)
elvis           ::= or ( '?:' elvis )?                    (* 右结合 *)
or              ::= and ( ( 'or' | '||' ) and )*
and             ::= cmp ( ( 'and' | '&&' ) cmp )*
cmp             ::= range ( ( '==' | '!=' | '<' | '>' | '<=' | '>='
                            | 'is' | 'is' 'not' ) range )?
range           ::= add ( ( '..' | '..=' ) add )?
add             ::= mul ( ( '+' | '-' ) mul )*
mul             ::= unary ( ( '*' | '/' | '%' ) unary )*
unary           ::= ( 'not' | '-' ) unary | postfix
postfix         ::= primary ( '(' args? ')'
                          | '[' expr ']'
                          | '.' IDENT
                          | '<' type ( ',' type )* '>' '(' args? ')' )*
primary         ::= INT | FLOAT | STRING | 'true' | 'false' | 'nil'
                  | IDENT | 'this' | 'super'
                  | list | map | lambda | '(' expr ')'
args            ::= expr ( ',' expr )*
list            ::= '[' ( expr ( ',' expr )* )? ']'
map             ::= '@' '(' ( expr ':' expr ( ',' expr ':' expr )* )? ')'
lambda          ::= ( '||' | '|' params? '|' ) ( '->' type )? block
```

## 词法

```ebnf
IDENT   ::= [A-Za-z_][A-Za-z0-9_]*
INT     ::= [0-9]+
FLOAT   ::= [0-9]+ '.' [0-9]+ ( [eE] [+-]? [0-9]+ )?
          | [0-9]+ [eE] [+-]? [0-9]+
STRING  ::= '"' ( 转义 | '${' expr '}' | 普通字符 )* '"'
comment ::= '//' ... '\n' | '/*' ... '*/'
```

**保留字**：`and or not true false for var let if else while func nil return class
super this break continue is pub trait impl dyn as`。

**上下文关键字**（仅在类型位置有特殊含义，可作标识符）：`int float bool str unit
range Array Map dyn`。

`&&` / `||` 是 `and` / `or` 的同义 token。范围运算符 `..` / `..=`，管道 `|>`，
Elvis `?:` 是独立 token（注意 `|>` 与 `||`、`..` 与 `.` 的区分）。
