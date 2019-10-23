use wasmer_runtime_core::{
    compile_with, imports, memory::Memory, typed_func::Func, types::MemoryDescriptor, units::Pages,
    vm,
};
use wasmer_runtime_core_tests::{get_compiler, wat2wasm};

#[test]
fn imported_functions_forms() {
    let wasm_binary = wat2wasm(MODULE.as_bytes()).expect("WAST not valid or malformed");
    let module = compile_with(&wasm_binary, &get_compiler()).unwrap();
    let memory_descriptor = MemoryDescriptor::new(Pages(1), Some(Pages(1)), false).unwrap();
    let memory = Memory::new(memory_descriptor).unwrap();

    const SHIFT: i32 = 10;
    memory.view()[0].set(SHIFT);

    let import_object = imports! {
        "env" => {
            "memory" => memory.clone(),
            "callback1_fn" => Func::new(callback1_fn),
            "callback1_closure" => Func::new(|n: i32| -> Result<i32, ()> {
                Ok(n + 1)
            }),
            "callback1_fn_with_vmctx" => Func::new(callback1_fn_with_vmctx),
            "callback1_closure_with_env" => Func::new(move |n: i32| -> Result<i32, ()> {
                let shift: i32 = memory.view()[0].get();

                Ok(shift + n + 1)
            }),
        },
    };
    let instance = module.instantiate(&import_object).unwrap();

    macro_rules! call_and_assert {
        ($function:ident, $expected_value:expr) => {
            let $function: Func<i32, i32> = instance.func(stringify!($function)).unwrap();

            let result = $function.call(1);

            assert_eq!(
                result, $expected_value,
                concat!("Expected right when calling `", stringify!($function), "`.")
            );
        };
    }

    call_and_assert!(function1_fn, Ok(2));
    call_and_assert!(function1_closure, Ok(2));
    call_and_assert!(function1_fn_with_vmctx, Ok(2 + SHIFT));
    call_and_assert!(function1_closure_with_env, Ok(2 + SHIFT));
}

fn callback1_fn(n: i32) -> Result<i32, ()> {
    Ok(n + 1)
}

fn callback1_fn_with_vmctx(vmctx: &mut vm::Ctx, n: i32) -> Result<i32, ()> {
    let memory = vmctx.memory(0);
    let shift: i32 = memory.view()[0].get();

    Ok(shift + n + 1)
}

static MODULE: &str = r#"
(module
  (type $type (func (param i32) (result i32)))
  (import "env" "memory" (memory 1 1))
  (import "env" "callback1_fn" (func $callback1_fn (type $type)))
  (import "env" "callback1_closure" (func $callback1_closure (type $type)))
  (import "env" "callback1_fn_with_vmctx" (func $callback1_fn_with_vmctx (type $type)))
  (import "env" "callback1_closure_with_env" (func $callback1_closure_with_env (type $type)))
  (func (export "function1_fn") (type $type)
    get_local 0
    call $callback1_fn)
  (func (export "function1_closure") (type $type)
    get_local 0
    call $callback1_closure)
  (func (export "function1_fn_with_vmctx") (type $type)
    get_local 0
    call $callback1_fn_with_vmctx)
  (func (export "function1_closure_with_env") (type $type)
    get_local 0
    call $callback1_closure_with_env))
"#;
