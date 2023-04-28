use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum DataType {
    UInt256,
    String,
    Address,
    UInt8,
    Bool,
    #[serde(alias = "uint8[]")]
    UInt8Array,
    #[serde(alias = "uint256[]")]
    UInt256Array,
    Contract(String),
    Enum(String),
    Other(String),
}

fn deserialize_data_type<'de, D>(deserializer: D) -> Result<DataType, D::Error>
where
    D: serde::Deserializer<'de>,
{
    struct DataTypeVisitor;
    impl<'de> serde::de::Visitor<'de> for DataTypeVisitor {
        type Value = DataType;

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(match v {
                "uint256" => DataType::UInt256,
                "string" => DataType::String,
                "address" => DataType::Address,
                "uint8" => DataType::UInt8,
                other => {
                    if other.starts_with("contract") {
                        DataType::Contract(other.replace("contract ", ""))
                    } else if other.starts_with("enum") {
                        DataType::Enum(other.replace("enum ", ""))
                    } else {
                        DataType::Other(other.into())
                    }
                }
            })
        }

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("string, uint256, address, uint8 or an internal type")
        }
    }
    deserializer.deserialize_any(DataTypeVisitor)
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EventInput {
    pub indexed: bool,
    #[serde(alias = "internalType", deserialize_with = "deserialize_data_type")]
    pub internal_type: DataType,
    pub name: String,
    #[serde(alias = "type")]
    pub input_type: DataType,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CtorInput {
    pub name: String,
    #[serde(alias = "internalType")]
    pub internal_type: DataType,
    #[serde(alias = "type")]
    pub input_type: DataType,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum StateMutability {
    NonPayable,
    View,
    Pure,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FuncIO {
    pub name: String,
    #[serde(alias = "type")]
    pub io_type: DataType,
    #[serde(alias = "internalType", deserialize_with = "deserialize_data_type")]
    pub internal_type: DataType,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum AbiEntry {
    Constructor {
        inputs: Vec<CtorInput>,
        #[serde(alias = "stateMutability")]
        mutability: StateMutability,
    },
    Event {
        anonymous: bool,
        name: String,
        inputs: Vec<EventInput>,
    },
    Function {
        name: String,
        #[serde(alias = "stateMutability")]
        mutability: StateMutability,
        #[serde(default)]
        constant: bool,
        inputs: Vec<FuncIO>,
        outputs: Vec<FuncIO>,
    },
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Contract {
    pub abi: Vec<AbiEntry>,
    #[serde(alias = "contractName")]
    pub name: String,
}

impl Contract {
    pub fn from_str(str: &str) -> serde_json::error::Result<Contract> {
        serde_json::from_str(str)
    }
}
