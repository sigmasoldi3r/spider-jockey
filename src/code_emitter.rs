use crate::{
    parser::{AbiEntry, Contract, DataType},
    ts,
};

fn translate_type(io_type: DataType) -> ts::Type {
    match io_type {
        DataType::UInt256 => ts::Type::Number,
        DataType::UInt8 => ts::Type::Number,
        DataType::UInt8Array => ts::Type::Array(Box::new(ts::Type::Number)),
        DataType::UInt256Array => ts::Type::Array(Box::new(ts::Type::Number)),
        DataType::String => ts::Type::String,
        DataType::Address => ts::Type::String,
        DataType::Bool => ts::Type::Boolean,
        DataType::Contract(_) => todo!(),
        DataType::Enum(_) => todo!(),
        DataType::Other(_) => todo!(),
    }
}

/// The code emitter grabs the ABI data and generates code based on
/// the information provided by it.
pub struct CodeEmitter;
impl CodeEmitter {
    pub fn emit_contract_abstraction(self) -> String {
        ts::Script::new()
            .class(
                "AbstractContract",
                ts::Export::Default,
                ts::ClassType::Interface,
            )
            .method("call", false, ts::Visibility::NotSpecified)
            .param("target", ts::Type::String)
            .rest_param("args", ts::Type::Array(Box::new(ts::Type::Any)))
            .method_end_abstract(ts::Type::Promise(Box::new(ts::Type::Unknown)))
            .class_end()
            .collect()
    }
    /// Emits the whole class code
    pub fn emit(self, contract: &Contract) -> Result<String, ()> {
        let builder = ts::Script::new()
            .import()
            .by_default("AbstractContract")
            .from("./AbstractContract")
            .import_end();
        let builder = builder
            .class(
                contract.name.clone(),
                ts::Export::Default,
                ts::ClassType::Normal,
            )
            .constructor()
            .field(
                "contract",
                ts::Type::Class("AbstractContract".into()),
                true,
                ts::Visibility::Private,
            )
            .constructor_end();
        let builder = contract.abi.iter().fold(builder, |builder, entry| {
            if let AbiEntry::Function {
                name,
                mutability: _,
                constant: _,
                inputs,
                outputs: _,
            } = entry
            {
                let builder = builder.method(name.clone(), true, ts::Visibility::Public);
                let mut i = -1;
                let builder = inputs
                    .iter()
                    .fold(builder, |builder, io| {
                        builder.param(
                            if io.name.is_empty() {
                                i += 1;
                                format!("_param{}", i)
                            } else {
                                io.name.clone()
                            },
                            translate_type(io.io_type.clone()),
                        )
                    })
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
                    .string(name.clone())
                    .param_end();
                i = -1;
                inputs
                    .iter()
                    .fold(builder, |builder, io| {
                        builder
                            .param()
                            .field(if io.name.is_empty() {
                                i += 1;
                                format!("_param{}", i)
                            } else {
                                io.name.clone()
                            })
                            .param_end()
                    })
                    .call_end()
                    .expression_end()
                    .method_end()
            } else {
                builder
            }
        });
        let builder = builder.class_end();
        Ok(builder.collect())
    }
}
