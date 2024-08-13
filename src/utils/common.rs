pub use databroker_proto::kuksa::val::v1::{datapoint::Value, DataType, Datapoint, Error};

#[derive(Debug, Clone)]
pub enum ClientError {
    Connection(String),
    Status(tonic::Status),
    Function(Vec<Error>),
}

// convert a str to Value
pub fn str_to_value(input: &str, datatype: DataType) -> Result<Value, ()> {
    // eg: (Float, "10.1") --> 10.1

    match datatype {
        DataType::String => Ok(Value::String(input.to_owned())),
        DataType::Boolean => match input.parse::<bool>() {
            Ok(value) => Ok(Value::Bool(value)),
            Err(_err) => {
                // return error parse Boolean
                Ok(Value::String("Parse Boolean error".to_string()))
            },
        },
        DataType::Int8 => match input.parse::<i8>() {
            Ok(value) => Ok(Value::Int32(value as i32)),
            Err(_err) => {
                // return error parse i8
                Ok(Value::String("Parse i8 error".to_string()))
            }
        },
        DataType::Int16 => match input.parse::<i16>() {
            Ok(value) => Ok(Value::Int32(value as i32)),
            Err(_err) => {
                // return error parse i16
                Ok(Value::String("Parse i16 error".to_string()))
            }
        },
        DataType::Int32 => match input.parse::<i32>() {
            Ok(value) => Ok(Value::Int32(value)),
            Err(_err) => {
                // return error parse i32
                Ok(Value::String("Parse i32 error".to_string()))
            }
        },
        DataType::Int64 => match input.parse::<i64>() {
            Ok(value) => Ok(Value::Int64(value)),
            Err(_err) => {
                // return error parse i64
                Ok(Value::String("Parse i64 error".to_string()))
            }
        },
        DataType::Uint8 => match input.parse::<u8>() {
            Ok(value) => Ok(Value::Uint32(value as u32)),
            Err(_err) => {
                // return error parse u8
                Ok(Value::String("Parse u8 error".to_string()))
            }
        },
        DataType::Uint16 => match input.parse::<u16>() {
            Ok(value) => Ok(Value::Uint32(value as u32)),
            Err(_err) => {
                // return error parse u16
                Ok(Value::String("Parse u16 error".to_string()))
            }
        },
        DataType::Uint32 => match input.parse::<u32>() {
            Ok(value) => Ok(Value::Uint32(value)),
            Err(_err) => {
                // return error parse u32
                Ok(Value::String("Parse u32 error".to_string()))
            }
        },
        DataType::Uint64 => match input.parse::<u64>() {
            Ok(value) => Ok(Value::Uint64(value)),
            Err(_err) => {
                // return error parse u64
                Ok(Value::String("Parse u64 error".to_string()))
            }
        },
        DataType::Float => match input.parse::<f32>() {
            Ok(value) => Ok(Value::Float(value)),
            Err(_err) => {
                // return error parse float
                Ok(Value::String("Parse float error".to_string()))
            }
        },
        DataType::Double => match input.parse::<f64>() {
            Ok(value) => Ok(Value::Double(value)),
            Err(_err) => {
                // return error parse double
                Ok(Value::String("Parse double error".to_string()))
            }
        },

        DataType::Unspecified => todo!(),
        DataType::Timestamp => todo!(),
        DataType::StringArray => todo!(),
        DataType::BooleanArray => todo!(),
        DataType::Int8Array => todo!(),
        DataType::Int16Array => todo!(),
        DataType::Int32Array => todo!(),
        DataType::Int64Array => todo!(),
        DataType::Uint8Array => todo!(),
        DataType::Uint16Array => todo!(),
        DataType::Uint32Array => todo!(),
        DataType::Uint64Array => todo!(),
        DataType::FloatArray => todo!(),
        DataType::DoubleArray => todo!(),
        DataType::TimestampArray => todo!(),
    }
}

pub fn value_from_option_datapoint(datapoint: Option<Datapoint>) -> Value {
    if let Some(data) = datapoint {
        if let Some(value) = data.value {
            return value;
        }
    }
    return Value::String("NotAvailable".to_string());
}