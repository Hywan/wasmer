//! Parse the WIT textual representation into an AST.

#![allow(unused)]

use crate::ast::*;
use wast::parser::{Cursor, Parse, Parser, Peek, Result};

mod kw {
    pub use wast::{
        custom_keyword,
        kw::{anyref, export, f32, f64, i32, i64, param, result},
    };

    custom_keyword!(adapt);
    custom_keyword!(Int);
    custom_keyword!(Float);
    custom_keyword!(Any);
    custom_keyword!(String);
    custom_keyword!(Seq);
}

struct AtInterface;

impl Peek for AtInterface {
    fn peek(cursor: Cursor<'_>) -> bool {
        cursor.reserved().map(|(string, _)| string) == Some("@interface")
    }

    fn display() -> &'static str {
        "`@interface`"
    }
}

impl Parse<'_> for AtInterface {
    fn parse(parser: Parser<'_>) -> Result<Self> {
        parser.step(|cursor| {
            if let Some(("@interface", rest)) = cursor.reserved() {
                return Ok((AtInterface, rest));
            }

            Err(cursor.error("expected `@interface`"))
        })
    }
}

#[derive(PartialEq, Debug)]
enum Interface<'a> {
    Export(Export<'a>),
    Type(Type<'a>),
    Import(Import<'a>),
    Adapter(Adapter<'a>),
    Forward(Forward<'a>),
}

impl<'a> Parse<'a> for Interface<'a> {
    fn parse(parser: Parser<'a>) -> Result<Self> {
        parser.parens(|parser| {
            let mut lookahead = parser.lookahead1();

            if lookahead.peek::<AtInterface>() {
                parser.parse::<AtInterface>()?;

                let mut lookahead = parser.lookahead1();

                if lookahead.peek::<kw::export>() {
                    Ok(Interface::Export(parser.parse()?))
                } else {
                    Err(lookahead.error())
                }
            } else {
                Err(lookahead.error())
            }
        })
    }
}

enum FunctionType {
    Input(Vec<InterfaceType>),
    Output(Vec<InterfaceType>),
}

impl Parse<'_> for FunctionType {
    fn parse(parser: Parser<'_>) -> Result<Self> {
        parser.parens(|parser| {
            let mut lookahead = parser.lookahead1();

            if lookahead.peek::<kw::param>() {
                parser.parse::<kw::param>()?;

                let mut inputs = vec![];

                while !parser.is_empty() {
                    inputs.push(parser.parse()?);
                }

                Ok(FunctionType::Input(inputs))
            } else if lookahead.peek::<kw::result>() {
                parser.parse::<kw::result>()?;

                let mut outputs = vec![];

                while !parser.is_empty() {
                    outputs.push(parser.parse()?);
                }

                Ok(FunctionType::Output(outputs))
            } else {
                Err(lookahead.error())
            }
        })
    }
}

impl<'a> Parse<'a> for Export<'a> {
    fn parse(parser: Parser<'a>) -> Result<Self> {
        parser.parse::<kw::export>()?;
        let name = parser.parse()?;

        let mut input_types = vec![];
        let mut output_types = vec![];

        while !parser.is_empty() {
            let function_type = parser.parse::<FunctionType>()?;

            match function_type {
                FunctionType::Input(mut inputs) => input_types.append(&mut inputs),
                FunctionType::Output(mut outputs) => output_types.append(&mut outputs),
            }
        }

        Ok(Export {
            name,
            input_types,
            output_types,
        })
    }
}

impl Parse<'_> for InterfaceType {
    fn parse(parser: Parser<'_>) -> Result<Self> {
        let mut lookahead = parser.lookahead1();

        if lookahead.peek::<kw::Int>() {
            parser.parse::<kw::Int>()?;

            Ok(InterfaceType::Int)
        } else if lookahead.peek::<kw::Float>() {
            parser.parse::<kw::Float>()?;

            Ok(InterfaceType::Float)
        } else if lookahead.peek::<kw::Any>() {
            parser.parse::<kw::Any>()?;

            Ok(InterfaceType::Any)
        } else if lookahead.peek::<kw::String>() {
            parser.parse::<kw::String>()?;

            Ok(InterfaceType::String)
        } else if lookahead.peek::<kw::Seq>() {
            parser.parse::<kw::Seq>()?;

            Ok(InterfaceType::Seq)
        } else if lookahead.peek::<kw::i32>() {
            parser.parse::<kw::i32>()?;

            Ok(InterfaceType::I32)
        } else if lookahead.peek::<kw::i64>() {
            parser.parse::<kw::i64>()?;

            Ok(InterfaceType::I64)
        } else if lookahead.peek::<kw::f32>() {
            parser.parse::<kw::f32>()?;

            Ok(InterfaceType::F32)
        } else if lookahead.peek::<kw::f64>() {
            parser.parse::<kw::f64>()?;

            Ok(InterfaceType::F64)
        } else if lookahead.peek::<kw::anyref>() {
            parser.parse::<kw::anyref>()?;

            Ok(InterfaceType::AnyRef)
        } else {
            Err(lookahead.error())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wast::parser::{self, ParseBuffer};

    #[test]
    fn test_foo() {
        let input = r#"(@interface export "foo" (param i32 i64) (result i32))"#;
        let output = Interface::Export(Export {
            name: "foo",
            input_types: vec![InterfaceType::I32, InterfaceType::I64],
            output_types: vec![InterfaceType::I32],
        });

        let buffer = ParseBuffer::new(input).expect("Failed to build the parser buffer.");

        assert_eq!(parser::parse::<Interface>(&buffer).unwrap(), output);
    }

    /*
    #[test]
    fn test_interface_type() {
        let inputs = vec![
            "Int", "Float", "Any", "String", "Seq", "i32", "i64", "f32", "f64", "anyref",
        ];
        let outputs = vec![
            InterfaceType::Int,
            InterfaceType::Float,
            InterfaceType::Any,
            InterfaceType::String,
            InterfaceType::Seq,
            InterfaceType::I32,
            InterfaceType::I64,
            InterfaceType::F32,
            InterfaceType::F64,
            InterfaceType::AnyRef,
        ];

        assert_eq!(inputs.len(), outputs.len());

        for (nth, input) in inputs.iter().enumerate() {
            assert_eq!(interface_type::<()>(input), Ok(("", outputs[nth])));
        }
    }

    #[test]
    fn test_param_empty() {
        let input = "(param)";
        let output = vec![];

        assert_eq!(param::<()>(input), Ok(("", output)));
    }

    #[test]
    fn test_param() {
        let input = "(param i32 String)";
        let output = vec![InterfaceType::I32, InterfaceType::String];

        assert_eq!(param::<()>(input), Ok(("", output)));
    }

    #[test]
    fn test_result_empty() {
        let input = "(result)";
        let output = vec![];

        assert_eq!(result::<()>(input), Ok(("", output)));
    }

    #[test]
    fn test_result() {
        let input = "(result i32 String)";
        let output = vec![InterfaceType::I32, InterfaceType::String];

        assert_eq!(result::<()>(input), Ok(("", output)));
    }

    #[test]
    fn test_export_with_no_param_no_result() {
        let input = r#"(@interface export "foo")"#;
        let output = Export {
            name: "foo",
            input_types: vec![],
            output_types: vec![],
        };

        assert_eq!(export::<()>(input), Ok(("", output)));
    }

    #[test]
    fn test_export_with_some_param_no_result() {
        let input = r#"(@interface export "foo" (param i32))"#;
        let output = Export {
            name: "foo",
            input_types: vec![InterfaceType::I32],
            output_types: vec![],
        };

        assert_eq!(export::<()>(input), Ok(("", output)));
    }

    #[test]
    fn test_export_with_no_param_some_result() {
        let input = r#"(@interface export "foo" (result i32))"#;
        let output = Export {
            name: "foo",
            input_types: vec![],
            output_types: vec![InterfaceType::I32],
        };

        assert_eq!(export::<()>(input), Ok(("", output)));
    }

    #[test]
    fn test_export_with_some_param_some_result() {
        let input = r#"(@interface export "foo" (param String) (result i32 i32))"#;
        let output = Export {
            name: "foo",
            input_types: vec![InterfaceType::String],
            output_types: vec![InterfaceType::I32, InterfaceType::I32],
        };

        assert_eq!(export::<()>(input), Ok(("", output)));
    }

    #[test]
    fn test_export_escaped_name() {
        let input = r#"(@interface export "fo\"o")"#;
        let output = Export {
            name: r#"fo\"o"#,
            input_types: vec![],
            output_types: vec![],
        };

        assert_eq!(export::<()>(input), Ok(("", output)));
    }

    #[test]
    fn test_import_qualifier() {
        let input = r#"(import "ns" "name")"#;
        let output = ("ns", "name");

        assert_eq!(import_qualifier::<()>(input), Ok(("", output)));
    }

    #[test]
    fn test_export_qualifier() {
        let input = r#"(export "name")"#;
        let output = "name";

        assert_eq!(export_qualifier::<()>(input), Ok(("", output)));
    }

    #[test]
    fn test_import_with_no_param_no_result() {
        let input = r#"(@interface func $ns_foo (import "ns" "foo"))"#;
        let output = Import {
            namespace: "ns",
            name: "foo",
            input_types: vec![],
            output_types: vec![],
        };

        assert_eq!(import::<()>(input), Ok(("", output)));
    }

    #[test]
    fn test_import_with_no_index_variable_no_param_no_result() {
        let input = r#"(@interface func (import "ns" "foo"))"#;
        let output = Import {
            namespace: "ns",
            name: "foo",
            input_types: vec![],
            output_types: vec![],
        };

        assert_eq!(import::<()>(input), Ok(("", output)));
    }

    #[test]
    fn test_import_with_some_param_no_result() {
        let input = r#"(@interface func $ns_foo (import "ns" "foo") (param i32))"#;
        let output = Import {
            namespace: "ns",
            name: "foo",
            input_types: vec![InterfaceType::I32],
            output_types: vec![],
        };

        assert_eq!(import::<()>(input), Ok(("", output)));
    }

    #[test]
    fn test_import_with_no_param_some_result() {
        let input = r#"(@interface func $ns_foo (import "ns" "foo") (result i32))"#;
        let output = Import {
            namespace: "ns",
            name: "foo",
            input_types: vec![],
            output_types: vec![InterfaceType::I32],
        };

        assert_eq!(import::<()>(input), Ok(("", output)));
    }

    #[test]
    fn test_import_with_some_param_some_result() {
        let input =
            r#"(@interface func $ns_foo (import "ns" "foo") (param String) (result i32 i32))"#;
        let output = Import {
            namespace: "ns",
            name: "foo",
            input_types: vec![InterfaceType::String],
            output_types: vec![InterfaceType::I32, InterfaceType::I32],
        };

        assert_eq!(import::<()>(input), Ok(("", output)));
    }

    #[test]
    fn test_adapter_import() {
        let input = r#"(@interface adapt (import "ns" "foo") (param i32 i32) (result i32))"#;
        let output = Adapter::Import {
            namespace: "ns",
            name: "foo",
            input_types: vec![InterfaceType::I32, InterfaceType::I32],
            output_types: vec![InterfaceType::I32],
            instructions: vec![],
        };

        assert_eq!(adapter::<()>(input), Ok(("", output)));
    }

    #[test]
    fn test_adapter_export() {
        let input = r#"(@interface adapt (export "foo") (param i32 i32) (result i32))"#;
        let output = Adapter::Export {
            name: "foo",
            input_types: vec![InterfaceType::I32, InterfaceType::I32],
            output_types: vec![InterfaceType::I32],
            instructions: vec![],
        };

        assert_eq!(adapter::<()>(input), Ok(("", output)));
    }
    */
}
