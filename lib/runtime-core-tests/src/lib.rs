#[cfg(test)]
mod tests {
    use std::{convert::TryInto, fs};
    use wasmer_clif_backend::CraneliftCompiler;
    use wasmer_runtime_core::{
        self as core, func, imports,
        interface_types::{
            self as wit, ast, decoders,
            interpreter::{Instruction, Interpreter},
            InterfaceValue,
        },
        memory,
        types::MemoryDescriptor,
        units::Pages,
        vm::Ctx,
    };

    fn get_module() -> core::Module {
        core::compile_with(
            fs::read("tests/assets/hello_world.wasm")
                .expect("Failed to read `tests/assets/hello_world.wasm`.")
                .as_slice(),
            &CraneliftCompiler::new(),
        )
        .expect("Failed to parse the `hello_world.wasm` module.")
    }

    #[test]
    fn test_has_custom_section() {
        let module = get_module();
        let custom_section = module.info().custom_sections.get("interface-types");

        assert!(custom_section.is_some());
    }

    #[test]
    fn test_parse_binary_from_custom_section() {
        let module = get_module();
        let custom_section_bytes = module
            .info()
            .custom_sections
            .get("interface-types")
            .unwrap()
            .as_slice();

        match decoders::binary::parse::<()>(custom_section_bytes) {
            Ok((remainder, interfaces)) => {
                assert!(remainder.is_empty());
                assert_eq!(
                    interfaces,
                    ast::Interfaces {
                        exports: vec![
                            ast::Export {
                                name: "strlen",
                                input_types: vec![ast::InterfaceType::I32],
                                output_types: vec![ast::InterfaceType::I32]
                            },
                            ast::Export {
                                name: "write_null_byte",
                                input_types: vec![ast::InterfaceType::I32, ast::InterfaceType::I32],
                                output_types: vec![ast::InterfaceType::I32],
                            }
                        ],
                        types: vec![],
                        imported_functions: vec![
                            ast::ImportedFunction {
                                namespace: "host",
                                name: "console_log",
                                input_types: vec![ast::InterfaceType::String],
                                output_types: vec![],
                            },
                            ast::ImportedFunction {
                                namespace: "host",
                                name: "document_title",
                                input_types: vec![],
                                output_types: vec![ast::InterfaceType::String],
                            }
                        ],
                        adapters: vec![
                            ast::Adapter::Import {
                                namespace: "host",
                                name: "console_log",
                                input_types: vec![ast::InterfaceType::I32],
                                output_types: vec![],
                                instructions: vec![
                                    Instruction::ArgumentGet { index: 0 },
                                    Instruction::ArgumentGet { index: 0 },
                                    Instruction::CallExport {
                                        export_name: "strlen"
                                    },
                                    Instruction::ReadUtf8,
                                    Instruction::Call { function_index: 0 },
                                ]
                            },
                            ast::Adapter::Import {
                                namespace: "host",
                                name: "document_title",
                                input_types: vec![],
                                output_types: vec![ast::InterfaceType::I32],
                                instructions: vec![
                                    Instruction::Call { function_index: 1 },
                                    Instruction::WriteUtf8 {
                                        allocator_name: "alloc"
                                    },
                                    Instruction::CallExport {
                                        export_name: "write_null_byte"
                                    },
                                ]
                            }
                        ],
                        forwards: vec![ast::Forward { name: "main" }]
                    }
                );

                let wat = String::from(&interfaces);

                assert_eq!(
                    wat,
                    r#";; Interfaces

;; Interface, Export strlen
(@interface export "strlen"
  (param i32)
  (result i32))

;; Interface, Export write_null_byte
(@interface export "write_null_byte"
  (param i32 i32)
  (result i32))

;; Interface, Imported function host.console_log
(@interface func $host_console_log (import "host" "console_log")
  (param String))

;; Interface, Imported function host.document_title
(@interface func $host_document_title (import "host" "document_title")
  (result String))

;; Interface, Adapter host.console_log
(@interface adapt (import "host" "console_log")
  (param i32)
  arg.get 0
  arg.get 0
  call-export "strlen"
  read-utf8
  call 0)

;; Interface, Adapter host.document_title
(@interface adapt (import "host" "document_title")
  (result i32)
  call 1
  write-utf8 "alloc"
  call-export "write_null_byte")

;; Interface, Forward main
(@interface forward (export "main"))"#,
                );
            }

            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_interpreter() {
        let module = get_module();

        let memory = memory::Memory::new(
            MemoryDescriptor::new(Pages(256), Some(Pages(256)), false).unwrap(),
        )
        .expect("Failed to create a memory.");

        let imports = imports! {
            "host" => {
                "console_log" => func!(console_log),
                "document_title" => func!(document_title),
            },
            "env" => {
                "memory" => memory,
            },
        };

        let instance = module
            .instantiate(&imports)
            .expect("Failed to instantiate the module.");
        let mut instance: wit::Instance = (&instance).into();

        let custom_section_bytes = module
            .info()
            .custom_sections
            .get("interface-types")
            .expect("Failed to find the custom section `interface-types`.")
            .as_slice();

        let (_, interfaces) = decoders::binary::parse::<()>(custom_section_bytes)
            .expect("Failed to parse the `interface-types` custom section.");

        let instructions = interfaces
            .adapters
            .iter()
            .find_map(|adapter| match adapter {
                ast::Adapter::Import {
                    namespace: "host",
                    name: "console_log",
                    instructions,
                    ..
                } => Some(instructions),
                _ => None,
            })
            .expect("Failed to find the instructions of the `host.console_log` import adapter.");

        let interpreter: Interpreter<
            wit::Instance,
            wit::Export,
            wit::LocalImport,
            core::memory::Memory,
            core::memory::MemoryView<'_, u8>,
        > = instructions.try_into().unwrap();

        let invocation_inputs = vec![InterfaceValue::I32(7), InterfaceValue::I32(42)];

        let run = interpreter.run(&invocation_inputs, &mut instance);

        //assert!(run.is_ok());

        assert_eq!("foo", run.unwrap_err());
    }

    fn console_log(_: &mut Ctx, pointer: i32) {
        println!("in console_log");
    }

    fn document_title(_: &mut Ctx) -> i32 {
        println!("in document_title");
        0
    }
}
