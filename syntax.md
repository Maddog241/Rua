```lua
chunk ::= block
block ::= {stmt} 
stmt ::= ';' |
        ['local'] namelist '=' explist |
        functioncall |
        break| 
        do block end | 
        while exp do block end | 
        if exp then block {elseif exp then block} [else block end] | 
        for Name '=' exp ',' exp [',' exp] do block end | 
        for namelist in explist do block end | 
        ['local'] function Name funcbody | 
        return [explist] [';']
functioncall ::= Name '(' explist ')'
namelist ::= Name {',' Name}
explist ::= exp {',' exp}
exp ::= nil | false | true | Numeral | LiteralString | func_exp |
        functioncall | tableconstructor | exp binop exp | unop exp
func_exp ::= function funcbody
funcbody ::= '(' [parlist] ')' block end
parlist ::= namelist (目前不支持变长参数，可能会修改为 namelist [',' '...'] | '...'])
tableconstructor ::= '{' [fieldlist] '}'
fieldlist ::= field {fieldsep field} 
field ::= Name '=' exp | exp
binop ::= '+' | '-' | '*' | '/' | '//' | '^' | '%' | '..' | 
    '<' | '>' | '>=' | '<=' | '==' | '~=' | and | or
unop ::= '-' | not 
```