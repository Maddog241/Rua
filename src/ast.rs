use std::fmt;

use crate::token::Token;

// // chunk
// pub struct Chunk {
//     pub block: Block,
// }

// impl fmt::Display for Chunk {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{}", self.block)
//     }
// }

// block
#[derive(Clone)]
pub struct Block {
    pub statements: Vec<Stmt>,
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.statements
            .iter()
            .fold(Ok(()), |_result, stmt| write!(f, "{}", stmt))
    }
}

// statement
#[derive(Clone)]
pub enum Stmt {
    Assign {
        left: VarList,
        right: ExpList,
    },
    LocalAssign {
        left: NameList,
        right: ExpList,
    },
    Break,
    DoBlockEnd {
        block: Block,
    },
    WhileStmt {
        condition: Exp,
        body: Block,
    },
    IfStmt {
        condition: Exp,
        then_branch: Block,
        elseif_branches: Vec<(Exp, Block)>,
        option_else_branch: Option<Block>,
    },
    NumericFor {
        name: Name,
        start: Exp,
        end: Exp,
        step: Exp,
        body: Block,
    },
    GenericFor {
        namelist: NameList,
        explist: ExpList,
        body: Block,
    },
    FuncDecl {
        local: bool,
        name: Name,
        parlist: NameList,
        body: Block,
    },
    FunctionCall {
        prefixexp: Box<Exp>,
        arguments: ExpList,
    },
    RetStmt {
        explist: ExpList,
    },
}

impl fmt::Display for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Assign { left, right } => {
                write!(f, "{} = {}\n", left, right)
            }

            Self::LocalAssign { left, right } => {
                if right.0.is_empty() {
                    write!(f, "local {}\n", left)
                } else {
                    write!(f, "local {} = {}\n", left, right)
                }
            }

            Self::FunctionCall {
                prefixexp,
                arguments,
            } => {
                write!(f, "{}({})\n", prefixexp, arguments)
            }

            Self::Break => {
                write!(f, "break\n")
            }

            Self::DoBlockEnd { block } => {
                write!(f, "{}", block)
            }

            Self::FuncDecl {
                local,
                name,
                parlist,
                body,
            } => {
                if *local {
                    write!(
                        f,
                        "\nFunctionDecl: local {}({}){{\n{}}}\n",
                        name, parlist, body
                    )
                } else {
                    write!(f, "\nFunctionDecl: {}({}){{\n{}}}\n", name, parlist, body)
                }
            }

            Self::IfStmt {
                condition,
                then_branch,
                elseif_branches,
                option_else_branch,
            } => {
                write!(f, "if({}) {{\n{}}} ", condition, then_branch)?;
                for (condition, elseif_branch) in elseif_branches.iter() {
                    write!(f, "elseif({}){{\n{}}}", condition, elseif_branch)?;
                }

                match option_else_branch {
                    Some(else_branch) => {
                        write!(f, "else{{\n{}}}\n", else_branch)
                    }
                    None => {
                        write!(f, "\n")
                    }
                }
            }

            Self::WhileStmt { condition, body } => {
                write!(f, "while({}) {{\n{}}}\n", condition, body)
            }

            Self::NumericFor {
                name,
                start,
                end,
                step,
                body,
            } => {
                write!(
                    f,
                    "NumericFor({}={},{},{}) do {{\n{}}}\n",
                    name, start, end, step, body
                )
            }

            Self::GenericFor {
                namelist,
                explist,
                body,
            } => {
                write!(
                    f,
                    "GenericFor({} = {}) do {{\n{}}}\n",
                    namelist, explist, body
                )
            }

            Self::RetStmt { explist } => {
                write!(f, "return {}\n", explist)
            }
        }
    }
}

#[derive(Clone)]
pub enum Var {
    Name { name: Name },
    TableIndex { prefixexp: Box<Exp>, exp: Box<Exp> },
}

#[derive(Clone)]
pub struct VarList {
    pub vars: Vec<Var>,
}

impl fmt::Display for Var {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Name { name } => write!(f, "{}", name),
            Self::TableIndex { prefixexp, exp } => write!(f, "{}[{}]", prefixexp, exp),
        }
    }
}

impl fmt::Display for VarList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut count = 0;
        write!(f, "Varlist(")?;
        self.vars.iter().fold(Ok(()), |result, name| {
            result.and_then(|_| {
                count += 1;
                if count == self.vars.len() {
                    write!(f, "{})", name)
                } else {
                    write!(f, "{}, ", name)
                }
            })
        })
    }
}

// name and namelist
pub type Name = String;

#[derive(Clone)]
pub struct NameList(pub Vec<Name>);

impl fmt::Display for NameList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut count = 0;
        write!(f, "Namelist(")?;
        self.0.iter().fold(Ok(()), |result, name| {
            result.and_then(|_| {
                count += 1;
                if count == self.0.len() {
                    write!(f, "{})", name)
                } else {
                    write!(f, "{}, ", name)
                }
            })
        })
    }
}

// expression and explist
#[derive(Clone)]
pub enum Exp {
    Literal {
        // nil, false, true, numeral, literal string
        value: Token,
    },
    Unary {
        operator: Token,
        right: Box<Exp>,
    },
    Binary {
        left: Box<Exp>,
        operator: Token,
        right: Box<Exp>,
    },
    Function {
        funcbody: FuncBody,
    },
    // prefix exp
    Var {
        var: Var,
    },
    FunctionCall {
        prefixexp: Box<Exp>,
        arguments: ExpList,
    },
    Grouping {
        exp: Box<Exp>,
    },
    TableConstructor {
        fieldlist: FieldList,
    },
}

#[derive(Clone)]
pub struct ExpList(pub Vec<Exp>);

impl fmt::Display for Exp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Literal { value } => write!(f, "{}", value.tok_type),
            Self::Unary { operator, right } => write!(f, "({} {})", operator.tok_type, right),
            Self::Binary {
                left,
                operator,
                right,
            } => write!(f, "({} {} {})", left, operator.tok_type, right),
            Self::Function { funcbody } => write!(f, "{}", funcbody),
            Self::Var { var } => write!(f, "{}", var),
            Self::FunctionCall {
                prefixexp,
                arguments,
            } => {
                write!(f, "{}({})", prefixexp, arguments)
            }
            Self::Grouping { exp } => write!(f, "{}", exp),
            Self::TableConstructor { fieldlist } => {
                write!(f, "Table{{{}}}", fieldlist)
            },
        }
    }
}

impl fmt::Display for ExpList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut count = 0;
        write!(f, "ExpList(")?;
        self.0.iter().fold(Ok(()), |result, name| {
            result.and_then(|_| {
                count += 1;
                if count == self.0.len() {
                    write!(f, "{})", name)
                } else {
                    write!(f, "{}, ", name)
                }
            })
        })
    }
}

// funcbody
#[derive(Clone)]
pub struct FuncBody {
    pub parlist: NameList,
    pub block: Block,
}

impl fmt::Display for FuncBody {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "function({}){{{}}}", self.parlist, self.block)
    }
}

// field and fieldlist
#[derive(Clone)]
pub struct Field {
    pub name: Option<Name>,
    pub exp: Exp,
}

#[derive(Clone)]
pub struct FieldList (pub Vec<Field>);

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.name {
            Some(name) => {
                write!(f, "{} = {}", name, self.exp)
            }
            None => {
                write!(f, "{}", self.exp)
            }
        }
    }
}

impl fmt::Display for FieldList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut count = 0;
        self.0.iter().fold(Ok(()), |result, name| {
            result.and_then(|_| {
                count += 1;
                if count == self.0.len() {
                    write!(f, "{}", name)
                } else {
                    write!(f, "{}, ", name)
                }
            })
        })
    }
}
