//! Parser for plain text ndb.
//! TODO: space between '=' and values
//! TODO: Parse multiple statements

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{
        alpha1,
        alphanumeric1,
        char,
        // digit1, multispace0
    },
    // combinator::map_res,
    combinator::{map, value},
    sequence::separated_pair,
    // sequence::Tuple,
    IResult,
    // Parser,
};

#[cfg(feature = "serde")]
use {
    serde::{Deserialize, Serialize, Serializer},
    std::collections::HashMap,
};

/// Struct representing an ast for a ndb database.
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Debug, PartialEq, Clone)]
pub struct Ndb {
    /// Chronological order list of statement
    pub statements: Vec<NdbStmt>,
}

/// struct representing a ndb statement
/// A statement consists of 2 parts, the key and the value.
///
/// name = glenda
/// ^^^^
/// name is the key
///
/// name = glenda
///        ^^^^^^
///        glenda is the value
/// Values are of type [`NdbValue`]

#[cfg_attr(feature = "serde", derive(Deserialize))]
#[derive(Debug, PartialEq, Clone)]
pub struct NdbStmt {
    /// $key = $value
    pub key: String,
    /// $key = $value
    pub value: NdbValue,
}

#[cfg(feature = "serde")]
impl Serialize for NdbStmt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = HashMap::new();
        map.insert(&self.key, &self.value);
        map.serialize(serializer)
    }
}

/// `NdbValue`
/// basically an ident
#[cfg_attr(feature = "serde", derive(Deserialize))]
#[derive(Debug, PartialEq, Clone)]
pub enum NdbValue {
    /// String value . e.g.
    /// name = glenda
    ///        ^^^^^^
    ///        this is a string
    /// Strings are not quoted.
    /// ndb has no quoted strings as far as I know.
    /// name = 9glenda
    ///        ^^^^^^^
    ///        this is also a string
    /// If there is a character in the value It becomes a string.
    String(String),

    /// Int value
    /// age = 21
    ///       ^^
    ///       int
    Int(i64),

    /// Bool
    /// switch = true
    ///          ^^^^
    ///          bool
    Bool(bool),
}

#[cfg(feature = "serde")]
impl Serialize for NdbValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            NdbValue::Int(value) => serializer.serialize_i64(*value),
            NdbValue::String(value) => serializer.serialize_str(value),
            NdbValue::Bool(value) => serializer.serialize_bool(*value),
        }
    }
}

/// parse a ndb value [`NdbValue`]
///
/// # Errors
/// Invalid syntax will error.
pub fn parse_value(input: &str) -> IResult<&str, NdbValue> {
    alt((
        value(NdbValue::Bool(true), tag("true")),
        value(NdbValue::Bool(false), tag("false")),
        // 9glenda
        // ^^^^^^^
        // string
        // the ident (ends with if a new word starts) is a string if there is any letter in it.
        // therefore parse_string has to be first.
        //
        // TODO
        // 9
        // ^
        // int
        parse_string_or_int,
    ))(input)
}
/// This function matches alphanumeric1 and tries to covert the result into an i64.
/// If the conversion is successful it returns the int.
/// Else it returns the alphanumeric1 as a string
fn parse_string_or_int(input: &str) -> IResult<&str, NdbValue> {
    map(alphanumeric1, |s: &str| s.parse::<i64>().map_or_else(|_| NdbValue::String(s.to_string()), NdbValue::Int))(input)
}

/// Parse a ndb statement
/// # Errors
/// Invalid syntax will error.
/// # Examples
/// ```rust
/// use rust_ndb::parser;
/// assert_eq!(
///     parser::ndb_stmt("name=glenda"),
///     Ok((
///         "",
///         parser::NdbStmt {
///             key: "name".to_string(),
///             value: parser::NdbValue::String("glenda".to_string()),
///         }
///     ))
/// );
/// ```
pub fn ndb_stmt(input: &str) -> IResult<&str, NdbStmt> {
    let mut stmt = map(
        separated_pair(alpha1, char('='), parse_value),
        |(key, value)| NdbStmt {
            key: String::from(key),
            value,
        },
    );
    stmt(input)
}

#[test]
fn parse_ndb() {
    assert_eq!(
        ndb_stmt("name=hello"),
        Ok((
            "",
            NdbStmt {
                key: "name".to_string(),
                value: NdbValue::String("hello".to_string()),
            }
        ))
    );

    assert_eq!(
        ndb_stmt("name=9glenda"),
        Ok((
            "",
            NdbStmt {
                key: "name".to_string(),
                value: NdbValue::String("9glenda".to_string()),
            }
        ))
    );

    assert_eq!(
        ndb_stmt("age=9"),
        Ok((
            "",
            NdbStmt {
                key: "age".to_string(),
                value: NdbValue::Int(9),
            }
        ))
    );

    assert_eq!(
        ndb_stmt("age=01"),
        Ok((
            "",
            NdbStmt {
                key: "age".to_string(),
                value: NdbValue::Int(1),
            }
        ))
    );
}

#[cfg(feature = "serde")]
#[test]
fn test_serde() {
    assert_eq!(
        serde_json::to_string(&NdbStmt {
            key: "name".to_string(),
            value: NdbValue::String("9glenda".to_string()),
        })
        .unwrap(),
        r#"{"name":"9glenda"}"#
    )
}
