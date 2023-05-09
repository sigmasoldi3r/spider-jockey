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
        .call_end()
        .expression_end()
        .class("Foo")
        .constructor()
        .field("bar", ts::Type::String, true, ts::Visibility::Public)
        .param(
            "_useless",
            ts::Type::Union(vec![ts::Type::Number, ts::Type::String, ts::Type::Null]),
        )
        .method_end()
        .class_end()
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
        .import()
        .by_default("AbstractContract")
        .from("./AbstractContract")
        .import_end()
        .class("MyClass")
        .constructor()
        .field(
            "contract",
            ts::Type::Class("AbstractContract".into()),
            true,
            ts::Visibility::Private,
        )
        .constructor_end()
        .method("myCustomCall", true, ts::Visibility::Public)
        .param("value", ts::Type::Number)
        .body()
        .expression()
        .do_return()
        .do_await()
        .field("this")
        .dot()
        .field("contract")
        .dot()
        .field("call")
        .call()
        .param()
        .string("myCustomCall")
        .param_end()
        .param()
        .field("value")
        .param_end()
        .call_end()
        .expression_end()
        .method_end()
        .class_end()
        .collect();
    assert_eq!(
        "
import AbstractContract from \"./AbstractContract\";
class MyClass {
  constructor(private readonly contract: AbstractContract) {}
  public async myCustomCall(value: number) {
    return await this.contract.call(\"myCustomCall\", value);
  }
}
",
        out
    );
}
