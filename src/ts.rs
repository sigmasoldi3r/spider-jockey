use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Type {
    Array(Box<Type>),
    Number,
    String,
    Boolean,
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
    Promise(Box<Type>),
}
impl Type {
    pub fn to_string(self) -> String {
        match self {
            Type::Promise(awaited) => format!("Promise<{}>", awaited.to_string()),
            Type::Boolean => "boolean".to_owned(),
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
    NotSpecified,
}
impl Visibility {
    pub fn to_string(self) -> String {
        match self {
            Visibility::Public => "public",
            Visibility::Protected => "protected",
            Visibility::Private => "private",
            Visibility::NotSpecified => "",
        }
        .to_string()
    }
}

pub enum Export {
    Private,
    Named,
    Default,
}
impl Export {
    pub fn to_string(self) -> String {
        match self {
            Export::Private => "",
            Export::Named => "export ",
            Export::Default => "export default ",
        }
        .to_string()
    }
}

pub enum ClassType {
    Interface,
    Abstract,
    Normal,
}
impl ClassType {
    pub fn to_string(self) -> String {
        match self {
            ClassType::Interface => "interface",
            ClassType::Abstract => "abstract class",
            ClassType::Normal => "class",
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
    pub fn class<S>(self, str: S, export: Export, abstraction: ClassType) -> Class
    where
        S: ToString,
    {
        let export = export.to_string();
        Class(
            self.0
                .line()
                .add(export)
                .add(format!("{} ", abstraction.to_string()))
                .add(str)
                .add(" {")
                .push(),
        )
    }
    pub fn collect(self) -> String {
        self.0.add("\n").output
    }
    pub fn expression(self) -> Expression {
        Expression(self.0.line())
    }
    pub fn method_end(self) -> Class {
        Class(self.0.pop().line().add("}"))
    }
    pub fn import(self) -> Import {
        Import(self.0.line())
    }
}

pub struct Import(Builder);
impl Import {
    pub fn by_default<S>(self, name: S) -> Import
    where
        S: ToString,
    {
        Import(self.0.add("import ").add(name).add(" from \""))
    }
    pub fn by_all<S>(self, alias: S) -> Import
    where
        S: ToString,
    {
        Import(self.0.add("import * as ").add(alias).add(" from \""))
    }
    pub fn from<S>(self, path: S) -> Import
    where
        S: ToString,
    {
        Import(self.0.add(path))
    }
    pub fn import_end(self) -> Script {
        Script(self.0.add("\";"))
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
    pub fn expression_end(self) -> Script {
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
    pub fn do_return(self) -> Expression {
        Expression(self.0.add("return "))
    }
    pub fn do_await(self) -> Expression {
        Expression(self.0.add("await "))
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
    pub fn call_end(self) -> Expression {
        Expression(self.0.add(")"))
    }
}

pub struct Class(Builder);
impl Class {
    pub fn class_end(self) -> Script {
        Script(self.0.pop().line().add("}"))
    }
    pub fn constructor(self) -> Method {
        Method(self.0.line().add("constructor("))
    }
    pub fn method<S>(self, name: S, is_async: bool, visibility: Visibility) -> Method
    where
        S: ToString,
    {
        let is_async = if is_async { "async " } else { "" };
        Method(self.0.line().add(format!(
            "{} {}{}(",
            visibility.to_string(),
            is_async,
            name.to_string()
        )))
    }
}

pub struct Method(Builder);
impl Method {
    pub fn param<S>(self, name: S, kind: Type) -> Self
    where
        S: ToString,
    {
        Method(if self.0.output.ends_with("(") {
            self.0.add(name).add(": ").add(kind.to_string())
        } else {
            self.0.add(", ").add(name).add(": ").add(kind.to_string())
        })
    }
    pub fn rest_param(self, name: &str, kind: Type) -> Self {
        Method(if self.0.output.ends_with("(") {
            self.0.add("...").add(name).add(": ").add(kind.to_string())
        } else {
            self.0
                .add(", ...")
                .add(name)
                .add(": ")
                .add(kind.to_string())
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
        Method(if self.0.output.ends_with("(") {
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
    pub fn method_end(self) -> Class {
        Class(self.0.add(") {}"))
    }
    pub fn method_end_abstract(self, return_type: Type) -> Class {
        Class(self.0.add(format!("): {};", return_type.to_string())))
    }
    pub fn constructor_end(self) -> Class {
        self.method_end()
    }
    pub fn body(self) -> Script {
        Script(self.0.add(") {").push())
    }
}
