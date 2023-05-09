use crate::ts;

#[test]
fn create_class_code() {
    let out = ts::Script::new()
        .expression()
        .field("foo")
        .dot()
        .field("bar")
        .call()
        .param()
        .field("the")
        .param_end()
        .param()
        .string("wailers")
        .param_end()
        .end()
        .as_statement()
        .class("Foo")
        .constructor()
        .field("bar", ts::Type::String, true, ts::Visibility::Public)
        .param(
            "_useless",
            ts::Type::Union(vec![ts::Type::Number, ts::Type::String, ts::Type::Null]),
        )
        .end()
        .pop()
        .collect();
    assert_eq!(
        "
foo.bar(the, \"wailers\");
class Foo {
  constructor(public readonly bar: string, _useless: number | string | null) {}
}
",
        out
    );
}

#[test]
fn create_class_with_functions() {
    let out = ts::Script::new()
        .class("MyClass")
        .constructor()
        .field(
            "contract",
            ts::Type::Class("AbstractContract".into()),
            true,
            ts::Visibility::Private,
        )
        .end()
        .pop()
        .collect();
    assert_eq!(
        "
import AbstractContract from \"./AbstractContract\";
class MyClass {
  constructor(private readonly contract: AbstractContract) {}
  async myCustomCall(value: number) {
    return await contract.call(\"myCustomCall\", value);
  }
}
",
        out
    );
}
