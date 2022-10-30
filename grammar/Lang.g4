grammar Lang;

prog:   stat* EOF;

stat
 : expr EOS
 ;

expr
 : op=(PLUS|MINUS) right=expr                            # unary_expr
 | left=expr op=(MULTIPLY | DIVIDE | MODULUS) right=expr # binary_expr
 | left=expr op=(PLUS | MINUS) right=expr                # binary_expr
 | '(' ch=expr ')'                                       # parenthesized_expr
 | value=NUMBER                                          # numeric_literal
 ;

MULTIPLY: '*' ;
DIVIDE: '/' ;
MODULUS: '%' ;

PLUS: '+' ;
MINUS: '-' ;

NUMBER : ('0' .. '9') + ('.' ('0' .. '9') +)? ;

EOS: ';' | '\n';

WS : [ \r\n\t] + -> skip ;
