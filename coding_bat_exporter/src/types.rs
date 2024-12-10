use serde::{ser::SerializeSeq, Serialize};

pub enum Type {
    Bool(bool),
    Number(f32),
    String(String),

    Array(Vec<Type>),
    ArrayList(Vec<Type>),
}

#[derive(Serialize)]
pub enum Types {
    Bool,
    Number,
    String,
    Array,
    ArrayList,
}

impl Type {
    pub fn as_types(&self) -> Types {
        match self {
            Type::Bool(_) => Types::Bool,
            Type::Number(_) => Types::Number,
            Type::String(_) => Types::String,
            Type::Array(_) => Types::Array,
            Type::ArrayList(_) => Types::ArrayList,
        }
    }
}

impl Serialize for Type {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Type::Bool(x) => serializer.serialize_bool(*x),
            Type::Number(x) => {
                if x.fract() != 0.0 {
                    serializer.serialize_f32(*x)
                } else {
                    serializer.serialize_u32(*x as u32)
                }
            },
            Type::String(x) => serializer.serialize_str(x),
            Type::Array(x) | Type::ArrayList(x) => {
                let mut seq = serializer.serialize_seq(Some(x.len()))?;
                for e in x {
                    seq.serialize_element(e)?;
                }
                seq.end()
            }
        }
    }
}
