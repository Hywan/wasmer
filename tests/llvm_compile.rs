#![cfg(all(feature = "backend-llvm", test))]
#![deny(
    dead_code,
    nonstandard_style,
    unused_imports,
    unused_mut,
    unused_variables,
    unused_unsafe,
    unreachable_patterns
)]

mod llvm {
    use wabt::wat2wasm;
    use wasmer::compiler::Compiler;
    use wasmer::compiler::CompilerConfig;
    use wasmer::compiler::{compile_with, compile_with_config_with, BackendCompilerConfig};
    use wasmer::imports;
    use wasmer_llvm_backend::LLVMCompiler;
    use wasmer_llvm_backend::{InkwellModule, LLVMBackendConfig, LLVMCallbacks};

    use std::cell::RefCell;
    use std::rc::Rc;

    pub fn get_compiler() -> impl Compiler {
        LLVMCompiler::new()
    }

    #[test]
    fn crash_return_with_float_on_stack() {
        const MODULE: &str = r#"
    (module
    (type (func))
    (type (func (param f64) (result f64)))
    (func $_start (type 0))
    (func $fmod (type 1) (param f64) (result f64)
        local.get 0
        f64.const 0x0p+0
        f64.mul
        return))
    "#;
        let wasm_binary = wat2wasm(MODULE.as_bytes()).expect("WAST not valid or malformed");
        let module = compile_with(&wasm_binary, &get_compiler()).unwrap();
        module.instantiate(&imports! {}).unwrap();
    }

    #[derive(Debug, Default)]
    pub struct RecordPreOptIR {
        preopt_ir: String,
    }

    impl LLVMCallbacks for RecordPreOptIR {
        fn preopt_ir_callback(&mut self, module: &InkwellModule) {
            self.preopt_ir = module.print_to_string().to_string();
        }
    }

    #[test]
    fn crash_select_with_mismatched_pending() {
        const WAT: &str = r#"
    (module
    (func (param f64) (result f64)
        f64.const 0x0p+0
        local.get 0
        f64.add
        f64.const 0x0p+0
        i32.const 0
        select))
    "#;
        let record_pre_opt_ir = Rc::new(RefCell::new(RecordPreOptIR::default()));
        let compiler_config = CompilerConfig {
            backend_specific_config: Some(BackendCompilerConfig(Box::new(LLVMBackendConfig {
                callbacks: Some(record_pre_opt_ir.clone()),
            }))),
            ..Default::default()
        };
        let wasm_binary = wat2wasm(WAT.as_bytes()).expect("WAST not valid or malformed");
        let module =
            compile_with_config_with(&wasm_binary, compiler_config, &get_compiler()).unwrap();
        module.instantiate(&imports! {}).unwrap();
        const LLVM: &str = r#"
  %s3 = fadd double 0.000000e+00, %s2
  %nan = fcmp uno double %s3, 0.000000e+00
  %2 = select i1 %nan, double 0x7FF8000000000000, double %s3
  %s5 = select i1 false, double %2, double 0.000000e+00
  br label %return
"#;
        // println!("Compiler IR {}", record_pre_opt_ir.borrow().preopt_ir);
        assert!(&record_pre_opt_ir.borrow().preopt_ir.contains(LLVM));
    }
}
