//! Tests for our own Debug implementation.
//!
//! The tests check against expected output. This may be a bit fragile, but it is likely OK for
//! actual use.

// Borrow some types from other places.
use ::message_encoding::{Basic, BasicEnumeration};

/// Some real-life message
#[test]
fn basic() {
    let mut basic = Basic::default();
    assert_eq!(format!("{:?}", basic),
               "Basic { \
                    int32: 0, \
                    bools: [], \
                    string: \"\", \
                    optional_string: None, \
                    enumeration: BasicEnumeration(0), \
                    enumeration_map: {}, \
                    string_map: {}, \
                    enumeration_btree_map: {}, \
                    string_btree_map: {}, \
                    oneof: None \
                }");
    basic.enumeration_map.insert(0, BasicEnumeration::TWO);
    basic.enumeration = BasicEnumeration::from(42);
    assert_eq!(format!("{:?}", basic),
               "Basic { \
                    int32: 0, \
                    bools: [], \
                    string: \"\", \
                    optional_string: None, \
                    enumeration: BasicEnumeration(42), \
                    enumeration_map: {0: BasicEnumeration(2)}, \
                    string_map: {}, \
                    enumeration_btree_map: {}, \
                    string_btree_map: {}, \
                    oneof: None \
                }");
}

/*
TODO(danburkert/prost#56):

/// A special case with a tuple struct
#[test]
fn tuple_struct() {
    #[derive(Clone, PartialEq, Message)]
    struct NewType(
        #[prost(enumeration="BasicEnumeration", tag="5")]
        BasicEnumeration,
    );
    assert_eq!(format!("{:?}", NewType(BasicEnumeration::TWO)), "NewType(TWO)");
    assert_eq!(format!("{:?}", NewType(42)), "NewType(42)");
}
*/

#[derive(Clone, PartialEq, Oneof)]
pub enum OneofWithEnum {
    #[prost(int32, tag="8")]
    Int(i32),
    #[prost(string, tag="9")]
    String(String),
    #[prost(enumeration="BasicEnumeration", tag="10")]
    Enumeration(BasicEnumeration),
}

#[derive(Clone, PartialEq, Message)]
struct MessageWithOneof {
    #[prost(oneof="OneofWithEnum", tags="8, 9, 10")]
    of: Option<OneofWithEnum>,
}

/// Enumerations inside oneofs
#[test]
fn oneof_with_enum() {
    let msg = MessageWithOneof {
        of: Some(OneofWithEnum::Enumeration(BasicEnumeration::TWO))
    };
    assert_eq!(format!("{:?}", msg), "MessageWithOneof { of: Some(Enumeration(BasicEnumeration(2))) }");
}
