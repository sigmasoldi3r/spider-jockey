use crate::parser::{AbiEntry, Contract, DataType, FuncIO, StateMutability};

#[derive(Debug)]
pub enum EmitError {
    UnexpectedIoType(DataType),
}

fn stringify_type(data_type: DataType) -> Result<String, EmitError> {
    match data_type {
        DataType::UInt256 => Ok("number".into()),
        DataType::UInt8 => Ok("number".into()),
        DataType::UInt8Array => Ok("number[]".into()),
        DataType::UInt256Array => Ok("number[]".into()),
        DataType::String => Ok("string".into()),
        DataType::Address => Ok("string".into()),
        DataType::Bool => Ok("boolean".into()),
        DataType::Contract(_) => Err(EmitError::UnexpectedIoType(data_type)),
        DataType::Enum(_) => Err(EmitError::UnexpectedIoType(data_type)),
        DataType::Other(_) => Err(EmitError::UnexpectedIoType(data_type)),
    }
}

/// The code emitter grabs the ABI data and generates code based on
/// the information provided by it.
pub struct CodeEmitter;
impl CodeEmitter {
    /// Emits a single class function
    pub fn emit_function(
        &self,
        name: &String,
        mutability: &StateMutability,
        constant: &bool,
        inputs: &Vec<FuncIO>,
        outputs: &Vec<FuncIO>,
    ) -> Result<String, EmitError> {
        if constant.clone() && !name.starts_with("get") {
            return Ok("".into());
        }
        let mut output = "async ".to_string();
        output.push_str(name);
        output.push_str("(");
        let mut out: Vec<String> = vec![];
        for input in inputs {
            let data_type = stringify_type(input.io_type.clone())?;
            let mut field = input.name.clone();
            field.push_str(": ");
            field.push_str(data_type.as_str());
            out.push(field);
        }
        output.push_str(out.join(", ").as_str());
        output.push_str("): ");
        if let Some(entry) = outputs.get(0) {
            output.push_str("Promise<");
            output.push_str(&stringify_type(entry.io_type.clone())?);
            output.push_str(">");
        } else {
            output.push_str("Promise<void>");
        }
        output.push_str(" {\n");
        output.push_str(&format!(
            "  const method = this.contract.methods[\"{}\"];\n",
            name
        ));
        let args: Vec<String> = inputs.iter().map(|x| x.name.clone()).collect();
        output.push_str(&format!(
            "  const callAction = await method({});\n",
            args.join(", ")
        ));
        match mutability {
            StateMutability::View => {
                output.push_str("  return await callAction.call();\n");
            }
            _ => {
                output.push_str("  return await callAction.send();\n");
            }
        }
        output.push_str("}");
        Ok(output)
    }

    /// Emits the whole class code
    pub fn emit(self, contract: &Contract) -> Result<String, EmitError> {
        let mut out = "import ethers from \"ethers\";\n\nexport default class ".to_string();
        out.push_str(&contract.name);
        out.push_str(" {\n");
        out.push_str("  constructor(private readonly contract: ethers.Contract) {}\n");
        for field in contract.abi.iter().map(|field| {
            if let AbiEntry::Function {
                name,
                mutability,
                constant,
                inputs,
                outputs,
            } = field
            {
                self.emit_function(name, mutability, constant, inputs, outputs)
            } else {
                Ok("".into())
            }
        }) {
            let field = field?;
            if field.len() > 0 {
                let func = indent::indent_all_by(2, field);
                out.push_str(&func);
                out.push_str("\n");
            }
        }
        out.push_str("}\n");
        Ok(out)
    }
}
