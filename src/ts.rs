use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Type {
    Array(Box<Type>),
    Number,
    String,
    Object,
    Any,
    Unknown,
    Never,
    Partial(Box<Type>),
    Interface(HashMap<String, Type>),
    Record(Box<Type>, Box<Type>),
    Tuple(Vec<Type>),
    Null,
    Union(Vec<Type>),
    Class(String),
}
impl Type {
    pub fn to_string(self) -> String {
        match self {
            Type::Class(name) => name,
            Type::Array(t) => format!("Array<{}>", t.to_string()),
            Type::Number => "number".to_owned(),
            Type::String => "string".to_owned(),
            Type::Object => "object".to_owned(),
            Type::Any => "any".to_owned(),
            Type::Unknown => "unknown".to_owned(),
            Type::Never => "never".to_owned(),
            Type::Partial(t) => format!("Partial<{}>", t.to_string()),
            Type::Interface(fields) => format!(
                "{{ {} }}",
                fields
                    .iter()
                    .map(|pair| format!(
                        "\"{}\": {}",
                        pair.0.to_string(),
                        pair.1.clone().to_string()
                    ))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            Type::Record(k, v) => format!("Record<{}, {}>", k.to_string(), v.to_string()),
            Type::Tuple(fields) => format!(
                "[{}]",
                fields
                    .iter()
                    .map(|x| x.clone().to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            Type::Null => "null".to_owned(),
            Type::Union(fields) => fields
                .iter()
                .map(|x| x.clone().to_string())
                .collect::<Vec<_>>()
                .join(" | "),
        }
    }
}

pub enum Visibility {
    Public,
    Protected,
    Private,
}
impl Visibility {
    pub fn to_string(self) -> String {
        match self {
            Visibility::Public => "public",
            Visibility::Protected => "protected",
            Visibility::Private => "private",
        }
        .to_string()
    }
}

pub struct Builder {
    output: String,
    level: u8,
    indent: String,
}
impl Builder {
    pub fn new() -> Self {
        Builder {
            output: "".into(),
            indent: "  ".into(),
            level: 0,
        }
    }
    pub fn push(self) -> Self {
        Builder {
            output: self.output,
            level: self.level + 1,
            indent: self.indent,
        }
    }
    pub fn pop(self) -> Self {
        Builder {
            output: self.output,
            level: self.level - 1,
            indent: self.indent,
        }
    }
    pub fn add<S>(self, str: S) -> Self
    where
        S: ToString,
    {
        Builder {
            output: format!("{}{}", self.output, str.to_string()),
            level: self.level,
            indent: self.indent,
        }
    }
    pub fn line(self) -> Self {
        Builder {
            output: format!(
                "{}\n{}",
                self.output,
                self.indent.repeat(self.level as usize)
            ),
            level: self.level,
            indent: self.indent,
        }
    }
}

pub struct Script(Builder);
impl Script {
    pub fn new() -> Self {
        Script(Builder::new())
    }
    pub fn class<S>(self, str: S) -> Class
    where
        S: ToString,
    {
        Class(self.0.line().add("class ").add(str).add(" {").push())
    }
    pub fn collect(self) -> String {
        self.0.add("\n").output
    }
    pub fn if_statement(self) -> Script {
        todo!("Not implemented yet")
    }
    pub fn expression(self) -> Expression {
        Expression(self.0.line())
    }
}

pub struct Expression(Builder);
impl Expression {
    pub fn this(self) -> Expression {
        Expression(self.0.add("this"))
    }
    pub fn dot(self) -> Expression {
        Expression(self.0.add("."))
    }
    pub fn field<S>(self, name: S) -> Expression
    where
        S: ToString,
    {
        Expression(self.0.add(name.to_string()))
    }
    pub fn call(self) -> CallExpression {
        CallExpression(self.0.add("("))
    }
    pub fn as_statement(self) -> Script {
        Script(self.0.add(";"))
    }
    pub fn param_end(self) -> CallExpression {
        CallExpression(self.0)
    }
    pub fn string<S>(self, value: S) -> Expression
    where
        S: ToString,
    {
        Expression(self.0.add(format!("\"{}\"", value.to_string())))
    }
    pub fn number<N>(self, value: N) -> Expression
    where
        N: ToString,
    {
        Expression(self.0.add(value.to_string()))
    }
}

pub struct CallExpression(Builder);
impl CallExpression {
    pub fn param(self) -> Expression {
        Expression(if self.0.output.ends_with("(") {
            self.0
        } else {
            self.0.add(", ")
        })
    }
    pub fn end(self) -> Expression {
        Expression(self.0.add(")"))
    }
}

pub struct Class(Builder);
impl Class {
    pub fn pop(self) -> Script {
        Script(self.0.pop().line().add("}"))
    }
    pub fn constructor(self) -> ClassConstructor {
        ClassConstructor(self.0.line().add("constructor("))
    }
}

pub struct ClassConstructor(Builder);
impl ClassConstructor {
    pub fn param<S>(self, name: S, kind: Type) -> Self
    where
        S: ToString,
    {
        ClassConstructor(if self.0.output.ends_with("(") {
            self.0.add(name).add(": ").add(kind.to_string())
        } else {
            self.0.add(", ").add(name).add(": ").add(kind.to_string())
        })
    }
    pub fn field<S>(self, name: S, kind: Type, readonly: bool, visibility: Visibility) -> Self
    where
        S: ToString,
    {
        let readonly = if readonly {
            format!("readonly ")
        } else {
            "".to_owned()
        };
        ClassConstructor(if self.0.output.ends_with("(") {
            self.0
                .add(format!("{} {}", visibility.to_string(), readonly))
                .add(name)
                .add(": ")
                .add(kind.to_string())
        } else {
            self.0
                .add(format!(", {} {}", visibility.to_string(), readonly))
                .add(name)
                .add(": ")
                .add(kind.to_string())
        })
    }
    pub fn end(self) -> Class {
        Class(self.0.add(") {}"))
    }
    pub fn body(self) -> ClassScript {
        todo!()
    }
}

pub struct ClassScript(Script);
impl ClassScript {
    pub fn end(self) -> Class {
        Class(self.0 .0.line().pop().add("}"))
    }
}
