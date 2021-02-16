#![allow(clippy::float_cmp, clippy::redundant_clone, clippy::unit_cmp)]

use super::*;
use maplit::hashmap;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;

macro_rules! assert_identical_json {
    ($ty:ty, $expr:expr) => {
        assert_identical_json::<$ty>($expr, $expr);
    };
}

/// Assert that the expression is the same whether it is deserialized directly, or deserialized
/// first to json and then to an attribute value
#[track_caller]
fn assert_identical_json<T>(t1: &AttributeValue, t2: &AttributeValue)
where
    T: serde::de::DeserializeOwned,
    T: PartialEq,
    T: std::fmt::Debug,
{
    let direct_result: T = from_attribute_value(t1).unwrap();
    let indirect_result: T = serde_json::from_value(from_attribute_value(t2).unwrap()).unwrap();
    assert_eq!(direct_result, indirect_result);
}

#[test]
fn deserialize_string() {
    let attribute_value = &AttributeValue {
        s: Some(String::from("Value")),
        ..AttributeValue::default()
    };

    let result: String = from_attribute_value(attribute_value).unwrap();

    assert_eq!(result, "Value");

    assert_identical_json!(String, attribute_value);
}

#[test]
fn deserialize_num() {
    macro_rules! deserialize_num {
        ($ty:ty, $n:expr) => {
            let attribute_value = &AttributeValue {
                n: Some(String::from(stringify!($n))),
                ..AttributeValue::default()
            };

            assert_eq!(from_attribute_value::<$ty>(attribute_value).unwrap(), $n);

            assert_identical_json!($ty, attribute_value);
        };
    }

    deserialize_num!(u8, 2);
    deserialize_num!(i8, -2);
    deserialize_num!(u16, 2);
    deserialize_num!(i16, -2);
    deserialize_num!(u32, 2);
    deserialize_num!(i32, -2);
    deserialize_num!(u64, 2);
    deserialize_num!(i64, -2);
    deserialize_num!(f32, 1.1);
    deserialize_num!(f64, 1.1);
}

#[test]
fn deserialize_bool() {
    let attribute_value = &AttributeValue {
        bool: Some(true),
        ..AttributeValue::default()
    };
    let result: bool = from_attribute_value(attribute_value).unwrap();
    assert_eq!(result, true);
    assert_identical_json!(bool, attribute_value);

    let attribute_value = &AttributeValue {
        bool: Some(false),
        ..AttributeValue::default()
    };
    let result: bool = from_attribute_value(&AttributeValue {
        bool: Some(false),
        ..AttributeValue::default()
    })
    .unwrap();
    assert_eq!(result, false);
    assert_identical_json!(bool, attribute_value);
}

#[test]
fn deserialize_char() {
    let attribute_value = &AttributeValue {
        s: Some(String::from("🥳")),
        ..AttributeValue::default()
    };
    let result: char = from_attribute_value(attribute_value).unwrap();
    assert_eq!(result, '🥳');
    assert_identical_json!(char, attribute_value);
}

#[test]
fn deserialize_unit() {
    let attribute_value = &AttributeValue {
        null: Some(true),
        ..AttributeValue::default()
    };
    let result: () = from_attribute_value(attribute_value).unwrap();
    assert_eq!(result, ());
    assert_identical_json!((), attribute_value);
}

#[test]
fn deserialize_option() {
    let attribute_value = &AttributeValue {
        null: Some(true),
        ..AttributeValue::default()
    };
    let result: Option<u8> = from_attribute_value(attribute_value).unwrap();
    assert_eq!(result, None);
    assert_identical_json!(Option<u8>, attribute_value);

    let attribute_value = &AttributeValue {
        n: Some(String::from("1")),
        ..AttributeValue::default()
    };
    let result: Option<u8> = from_attribute_value(attribute_value).unwrap();
    assert_eq!(result, Some(1));
    assert_identical_json!(Option<u8>, attribute_value);
}

#[test]
fn deserialize_struct_with_string() {
    #[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
    struct Subject {
        value: String,
    }

    let attribute_value = &AttributeValue {
        m: Some(hashmap! {
            String::from("value") => AttributeValue {
                s: Some(String::from("Value")),
                ..AttributeValue::default()
            },
        }),
        ..AttributeValue::default()
    };

    let s: Subject = from_attribute_value(attribute_value).unwrap();
    assert_eq!(
        s,
        Subject {
            value: String::from("Value"),
        }
    );
    assert_identical_json!(Subject, attribute_value);
}

#[test]
fn deserialize_bytes() {
    #[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
    struct Subject {
        #[serde(with = "serde_bytes")]
        value: Vec<u8>,
    }

    let attribute_value = &AttributeValue {
        m: Some(hashmap! {
            String::from("value") => AttributeValue {
                b: Some(vec![116, 101, 115, 116, 0, 0, 0, 0].into()),
                ..AttributeValue::default()
            },
        }),
        ..AttributeValue::default()
    };

    let s: Subject = from_attribute_value(attribute_value).unwrap();
    assert_eq!(
        s,
        Subject {
            value: vec![116, 101, 115, 116, 0, 0, 0, 0],
        }
    );
}

#[test]
fn deserialize_byte_arrays() {
    #[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
    struct Subject {
        value: Vec<serde_bytes::ByteBuf>,
    }

    let attribute_value = &AttributeValue {
        m: Some(hashmap! {
            String::from("value") => AttributeValue {
                bs: Some(vec![
                    vec![116, 101, 115, 116, 0, 0, 0, 0].into(),
                    vec![2].into(),
                    vec![0, 0, 0, 0].into(),
                ]),
                ..AttributeValue::default()
            },
        }),
        ..AttributeValue::default()
    };

    let s: Subject = from_attribute_value(attribute_value).unwrap();
    assert_eq!(
        s,
        Subject {
            value: vec![
                serde_bytes::ByteBuf::from(vec![116, 101, 115, 116, 0, 0, 0, 0]),
                serde_bytes::ByteBuf::from(vec![2]),
                serde_bytes::ByteBuf::from(vec![0, 0, 0, 0]),
            ],
        }
    );
}

#[test]
fn deserialize_struct_with_aws_extra_data() {
    #[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
    struct Subject {
        id: String,
        value: u64,
    }

    let attribute_value = &AttributeValue {
        m: Some(hashmap! {
            String::from("id") => AttributeValue {
                s: Some(String::from("test-4")),
                ..AttributeValue::default()
            },
            String::from("value") => AttributeValue {
                n: Some(String::from("42")),
                ..AttributeValue::default()
            },
            String::from("aws:rep:deleting") => AttributeValue {
                bool: Some(false),
                ..AttributeValue::default()
            },
            String::from("aws:rep:updateregion") => AttributeValue {
                s: Some(String::from("us-west-2")),
                ..AttributeValue::default()
            },
            String::from("aws:rep:updatetime") => AttributeValue {
                n: Some(String::from("1565723640.315001")),
                ..AttributeValue::default()
            },
        }),
        ..AttributeValue::default()
    };

    let s: Subject = from_attribute_value(attribute_value).unwrap();
    assert_eq!(
        s,
        Subject {
            id: String::from("test-4"),
            value: 42,
        }
    );
    assert_identical_json!(Subject, attribute_value);
}

#[test]
fn deserialize_array_of_struct_with_string() {
    #[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
    struct Subject {
        value: String,
    }

    let attribute_value = &AttributeValue {
        l: Some(vec![
            AttributeValue {
                m: Some(hashmap! {
                    String::from("value") => AttributeValue { s: Some(String::from("1")), ..AttributeValue::default() },
                }),
                ..AttributeValue::default()
            },
            AttributeValue {
                m: Some(hashmap! {
                    String::from("value") => AttributeValue { s: Some(String::from("2")), ..AttributeValue::default() },
                }),
                ..AttributeValue::default()
            },
            AttributeValue {
                m: Some(hashmap! {
                    String::from("value") => AttributeValue { s: Some(String::from("3")), ..AttributeValue::default() },
                }),
                ..AttributeValue::default()
            },
        ]),
        ..AttributeValue::default()
    };

    let s: Vec<Subject> = from_attribute_value(attribute_value).unwrap();
    assert_eq!(
        s,
        vec![
            Subject {
                value: String::from("1"),
            },
            Subject {
                value: String::from("2"),
            },
            Subject {
                value: String::from("3"),
            },
        ]
    );
    assert_identical_json!(Vec<Subject>, attribute_value);
}

#[test]
fn deserialize_list() {
    let attribute_value = &AttributeValue {
        l: Some(vec![
            AttributeValue {
                s: Some(String::from("1")),
                ..AttributeValue::default()
            },
            AttributeValue {
                s: Some(String::from("2")),
                ..AttributeValue::default()
            },
            AttributeValue {
                s: Some(String::from("3")),
                ..AttributeValue::default()
            },
        ]),
        ..AttributeValue::default()
    };

    let s: Vec<String> = from_attribute_value(attribute_value).unwrap();
    assert_eq!(s, vec!["1", "2", "3"]);
    assert_identical_json!(Vec<String>, attribute_value);
}

#[test]
fn deserialize_string_list() {
    let attribute_value = &AttributeValue {
        ss: Some(vec![
            String::from("1"),
            String::from("2"),
            String::from("3"),
        ]),
        ..AttributeValue::default()
    };

    let v: Vec<String> = from_attribute_value(attribute_value).unwrap();
    assert_eq!(v, vec!["1", "2", "3"]);
    assert_identical_json!(Vec<String>, attribute_value);
}

#[test]
fn deserialize_int_list() {
    let attribute_value = &AttributeValue {
        ns: Some(vec![
            String::from("1"),
            String::from("2"),
            String::from("3"),
        ]),
        ..AttributeValue::default()
    };

    let v: Vec<u64> = from_attribute_value(attribute_value).unwrap();
    assert_eq!(v, vec![1, 2, 3]);
    assert_identical_json!(Vec<u64>, attribute_value);
}

#[test]
fn deserialize_float_list() {
    let attribute_value = &AttributeValue {
        ns: Some(vec![
            String::from("1"),
            String::from("2"),
            String::from("0.5"),
        ]),
        ..AttributeValue::default()
    };

    let v: Vec<f64> = from_attribute_value(attribute_value).unwrap();
    assert_eq!(v.len(), 3);
    assert!(0.9 < v[0] && v[0] < 1.1);
    assert!(1.9 < v[1] && v[1] < 2.1);
    assert!(0.4 < v[2] && v[2] < 0.6);
}

#[test]
fn deserialize_unit_struct() {
    #[derive(Debug, Deserialize, Eq, PartialEq)]
    struct Subject;

    let attribute_value = &AttributeValue {
        null: Some(true),
        ..AttributeValue::default()
    };

    let s: Subject = from_attribute_value(attribute_value).unwrap();
    assert_eq!(s, Subject);

    assert_identical_json!(Subject, attribute_value)
}

#[test]
fn deserialize_newtype_struct() {
    #[derive(Debug, Deserialize, Eq, PartialEq)]
    struct Subject(u8);

    let attribute_value = &AttributeValue {
        n: Some(String::from("1")),
        ..AttributeValue::default()
    };

    let s: Subject = from_attribute_value(attribute_value).unwrap();
    assert_eq!(s, Subject(1));

    assert_identical_json!(Subject, attribute_value)
}

#[test]
fn deserialize_tuple_struct() {
    #[derive(Debug, Deserialize, Eq, PartialEq)]
    struct Subject(u8, u8);

    let attribute_value = &AttributeValue {
        l: Some(vec![
            AttributeValue {
                n: Some(String::from("1")),
                ..AttributeValue::default()
            },
            AttributeValue {
                n: Some(String::from("2")),
                ..AttributeValue::default()
            },
        ]),
        ..AttributeValue::default()
    };

    let s: Subject = from_attribute_value(attribute_value).unwrap();
    assert_eq!(s, Subject(1, 2));

    assert_identical_json!(Subject, attribute_value)
}

#[test]
fn deserialize_tuple() {
    let attribute_value = &AttributeValue {
        l: Some(vec![
            AttributeValue {
                n: Some(String::from("1")),
                ..AttributeValue::default()
            },
            AttributeValue {
                n: Some(String::from("2")),
                ..AttributeValue::default()
            },
        ]),
        ..AttributeValue::default()
    };

    let s: (usize, usize) = from_attribute_value(attribute_value).unwrap();
    assert_eq!(s, (1, 2));

    assert_identical_json!((usize, usize), attribute_value)
}

#[test]
fn deserialize_map_with_strings() {
    let attribute_value = &AttributeValue {
        m: Some(hashmap! {
            String::from("one") => AttributeValue {
                n: Some(String::from("1")),
                ..AttributeValue::default()
            },
            String::from("two") => AttributeValue {
                n: Some(String::from("2")),
                ..AttributeValue::default()
            },
        }),
        ..AttributeValue::default()
    };

    let s: HashMap<String, usize> = from_attribute_value(attribute_value).unwrap();
    assert_eq!(
        s,
        hashmap! { String::from("one") => 1, String::from("two") => 2 }
    );

    assert_identical_json!(HashMap<String, usize>, attribute_value)
}

#[test]
fn deserialize_enum_unit() {
    #[derive(Debug, Deserialize, Eq, PartialEq)]
    enum Subject {
        Unit,
    }

    let attribute_value = &AttributeValue {
        s: Some(String::from("Unit")),
        ..AttributeValue::default()
    };

    let s: Subject = from_attribute_value(attribute_value).unwrap();
    assert_eq!(s, Subject::Unit);

    assert_identical_json!(Subject, attribute_value)
}

#[test]
fn deserialize_enum_newtype() {
    #[derive(Debug, Deserialize, Eq, PartialEq)]
    enum Subject {
        Newtype(u8),
    }

    let attribute_value = &AttributeValue {
        m: Some(hashmap! {
            String::from("Newtype") => AttributeValue {
                n: Some(String::from("1")),
                ..AttributeValue::default()
            },
        }),
        ..AttributeValue::default()
    };

    let s: Subject = from_attribute_value(attribute_value).unwrap();
    assert_eq!(s, Subject::Newtype(1));

    assert_identical_json!(Subject, attribute_value)
}

#[test]
fn deserialize_enum_tuple() {
    #[derive(Debug, Deserialize, Eq, PartialEq)]
    enum Subject {
        Tuple(u8, u8),
    }

    let attribute_value = &AttributeValue {
        m: Some(hashmap! {
            String::from("Tuple") => AttributeValue {
                l: Some(vec![
                    AttributeValue {
                        n: Some(String::from("1")),
                        ..AttributeValue::default()
                    },
                    AttributeValue {
                        n: Some(String::from("2")),
                        ..AttributeValue::default()
                    },
                ]),
                ..AttributeValue::default()
            },
        }),
        ..AttributeValue::default()
    };

    let s: Subject = from_attribute_value(attribute_value).unwrap();
    assert_eq!(s, Subject::Tuple(1, 2));

    assert_identical_json!(Subject, attribute_value)
}

#[test]
fn deserialize_enum_struct_variant() {
    #[derive(Debug, Deserialize, Eq, PartialEq)]
    enum Subject {
        Structy { one: u8, two: u8 },
    }

    let attribute_value = &AttributeValue {
        m: Some(hashmap! {
            String::from("Structy") => AttributeValue {
                m: Some(hashmap! {
                    String::from("one") => AttributeValue {
                        n: Some(String::from("1")),
                        ..AttributeValue::default()
                    },
                    String::from("two") => AttributeValue {
                        n: Some(String::from("2")),
                        ..AttributeValue::default()
                    },
                }),
                ..AttributeValue::default()
            },
        }),
        ..AttributeValue::default()
    };

    let s: Subject = from_attribute_value(attribute_value).unwrap();
    assert_eq!(s, Subject::Structy { one: 1, two: 2 });

    assert_identical_json!(Subject, attribute_value)
}

#[test]
fn deserialize_internally_tagged_enum() {
    #[derive(Debug, Deserialize, Eq, PartialEq)]
    #[serde(tag = "type")]
    enum Subject {
        One { one: u64 },
        Two { two: u8 },
    }

    let attribute_value = &AttributeValue {
        m: Some(hashmap! {
            String::from("type") => AttributeValue {
                s: Some(String::from("One")),
                ..AttributeValue::default()
            },
            String::from("one") => AttributeValue {
                n: Some(String::from("1")),
                ..AttributeValue::default()
            },
        }),
        ..AttributeValue::default()
    };

    let s: Subject = from_attribute_value(attribute_value).unwrap();
    assert_eq!(s, Subject::One { one: 1 });

    assert_identical_json!(Subject, attribute_value)
}

#[test]
fn deserialize_chrono_datetime() {
    use chrono::{DateTime, Utc};

    let attribute_value = &AttributeValue {
        s: Some(String::from("1985-04-21T18:34:13.449057039Z")),
        ..AttributeValue::default()
    };

    let s: DateTime<Utc> = from_attribute_value(attribute_value).unwrap();
    assert_eq!(
        s,
        DateTime::parse_from_rfc3339("1985-04-21T18:34:13.449057039Z")
            .unwrap()
            .with_timezone(&Utc)
    );

    assert_identical_json!(DateTime<Utc>, attribute_value)
}
