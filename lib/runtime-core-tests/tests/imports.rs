use std::{convert::TryInto, sync::Arc};
use wasmer_runtime_core::{
    compile_with,
    error::RuntimeError,
    imports,
    memory::Memory,
    typed_func::Func,
    types::{FuncSig, MemoryDescriptor, Type, Value},
    units::Pages,
    vm, Instance,
};
use wasmer_runtime_core_tests::{get_compiler, wat2wasm};

macro_rules! call_and_assert {
    ($instance:ident, $function:ident( $( $inputs:ty ),*) -> $output:ty, ( $( $arguments:expr ),* ) == $expected_value:expr) => {
        let $function: Func<( $( $inputs ),* ), $output> = $instance.func(stringify!($function)).expect(concat!("Failed to get the `", stringify!($function), "` export function."));

        let result = $function.call( $( $arguments ),* );

        match (result, $expected_value) {
            (Ok(value), expected_value) => assert_eq!(
                Ok(value),
                expected_value,
                concat!("Expected right when calling `", stringify!($function), "`.")
            ),
            (
                Err(RuntimeError::Error { data }),
                Err(RuntimeError::Error {
                    data: expected_data,
                }),
            ) => {
                if let (Some(data), Some(expected_data)) = (
                    data.downcast_ref::<&str>(),
                    expected_data.downcast_ref::<&str>(),
                ) {
                    assert_eq!(
                        data, expected_data,
                        concat!("Expected right when calling `", stringify!($function), "`.")
                    )
                } else if let (Some(data), Some(expected_data)) = (
                    data.downcast_ref::<String>(),
                    expected_data.downcast_ref::<String>(),
                ) {
                    assert_eq!(
                        data, expected_data,
                        concat!("Expected right when calling `", stringify!($function), "`.")
                    )
                } else {
                    assert!(false, "Unexpected error, cannot compare it.")
                }
            }
            (result, expected_value) => assert!(
                false,
                format!(
                    "Unexpected assertion for `{}`: left = `{:?}`, right = `{:?}`.",
                    stringify!($function),
                    result,
                    expected_value
                )
            ),
        }
    };
}

/// The shift that is set in the instance memory. The value is part of
/// the result returned by the imported functions if the memory is
/// read properly.
const SHIFT: i32 = 10;

/// The shift that is captured in the environment of a closure. The
/// value is part of the result returned by the imported function if
/// the closure captures its environment properly.
#[allow(non_upper_case_globals)]
const shift: i32 = 100;

fn imported_functions_forms(test: &dyn Fn(&Instance)) {
    const MODULE: &str = r#"
(module
  (type $type (func (param i32) (result i32)))
  (import "env" "memory" (memory 1 1))
  (import "env" "callback_fn" (func $callback_fn (type $type)))
  (import "env" "callback_closure" (func $callback_closure (type $type)))
  (import "env" "callback_closure_with_env" (func $callback_closure_with_env (type $type)))
  (import "env" "callback_fn_with_vmctx" (func $callback_fn_with_vmctx (type $type)))
  (import "env" "callback_closure_with_vmctx" (func $callback_closure_with_vmctx (type $type)))
  (import "env" "callback_closure_with_vmctx_and_env" (func $callback_closure_with_vmctx_and_env (type $type)))
  (import "env" "callback_fn_variadic" (func $callback_fn_variadic (type $type)))
  (import "env" "callback_closure_variadic_0" (func $callback_closure_variadic_0))
  (import "env" "callback_closure_variadic_1" (func $callback_closure_variadic_1 (param i32) (result i32)))
  (import "env" "callback_closure_variadic_2" (func $callback_closure_variadic_2 (param i32 i64) (result i64)))
  (import "env" "callback_closure_variadic_3" (func $callback_closure_variadic_3 (param i32 i64 f32) (result f32)))
  (import "env" "callback_closure_variadic_4" (func $callback_closure_variadic_4 (param i32 i64 f32 f64) (result f64)))
  (import "env" "callback_fn_trap" (func $callback_fn_trap (type $type)))
  (import "env" "callback_closure_trap" (func $callback_closure_trap (type $type)))
  (import "env" "callback_fn_trap_with_vmctx" (func $callback_fn_trap_with_vmctx (type $type)))
  (import "env" "callback_closure_trap_with_vmctx" (func $callback_closure_trap_with_vmctx (type $type)))
  (import "env" "callback_closure_trap_with_vmctx_and_env" (func $callback_closure_trap_with_vmctx_and_env (type $type)))

  (func (export "function_fn") (type $type)
    get_local 0
    call $callback_fn)

  (func (export "function_closure") (type $type)
    get_local 0
    call $callback_closure)

  (func (export "function_closure_with_env") (type $type)
    get_local 0
    call $callback_closure_with_env)

  (func (export "function_fn_with_vmctx") (type $type)
    get_local 0
    call $callback_fn_with_vmctx)

  (func (export "function_closure_with_vmctx") (type $type)
    get_local 0
    call $callback_closure_with_vmctx)

  (func (export "function_closure_with_vmctx_and_env") (type $type)
    get_local 0
    call $callback_closure_with_vmctx_and_env)

  (func (export "function_fn_variadic") (type $type)
    get_local 0
    call $callback_fn_variadic)

  (func (export "function_closure_variadic_0")
    call $callback_closure_variadic_0)

  (func (export "function_closure_variadic_1") (param i32) (result i32)
    get_local 0
    call $callback_closure_variadic_1)

  (func (export "function_closure_variadic_2") (param i32 i64) (result i64)
    get_local 0
    get_local 1
    call $callback_closure_variadic_2)

  (func (export "function_closure_variadic_3") (param i32 i64 f32) (result f32)
    get_local 0
    get_local 1
    get_local 2
    call $callback_closure_variadic_3)

  (func (export "function_closure_variadic_4") (param i32 i64 f32 f64) (result f64)
    get_local 0
    get_local 1
    get_local 2
    get_local 3
    call $callback_closure_variadic_4)

  (func (export "function_fn_trap") (type $type)
    get_local 0
    call $callback_fn_trap)

  (func (export "function_closure_trap") (type $type)
    get_local 0
    call $callback_closure_trap)

  (func (export "function_fn_trap_with_vmctx") (type $type)
    get_local 0
    call $callback_fn_trap_with_vmctx)

  (func (export "function_closure_trap_with_vmctx") (type $type)
    get_local 0
    call $callback_closure_trap_with_vmctx)

  (func (export "function_closure_trap_with_vmctx_and_env") (type $type)
    get_local 0
    call $callback_closure_trap_with_vmctx_and_env))
"#;

    let wasm_binary = wat2wasm(MODULE.as_bytes()).expect("WAST not valid or malformed");
    let module = compile_with(&wasm_binary, &get_compiler()).unwrap();
    let memory_descriptor = MemoryDescriptor::new(Pages(1), Some(Pages(1)), false).unwrap();
    let memory = Memory::new(memory_descriptor).unwrap();

    memory.view()[0].set(SHIFT);

    let import_object = imports! {
        "env" => {
            "memory" => memory.clone(),

            // Regular function.
            "callback_fn" => Func::new(callback_fn),

            // Closure without a captured environment.
            "callback_closure" => Func::new(|n: i32| -> Result<i32, ()> {
                Ok(n + 1)
            }),

            // Closure with a captured environment (a single variable + an instance of `Memory`).
            "callback_closure_with_env" => Func::new(move |n: i32| -> Result<i32, ()> {
                let shift_ = shift + memory.view::<i32>()[0].get();

                Ok(shift_ + n + 1)
            }),

            // Regular function with an explicit `vmctx`.
            "callback_fn_with_vmctx" => Func::new(callback_fn_with_vmctx),

            // Closure without a captured environment but with an explicit `vmctx`.
            "callback_closure_with_vmctx" => Func::new(|vmctx: &mut vm::Ctx, n: i32| -> Result<i32, ()> {
                let memory = vmctx.memory(0);
                let shift_: i32 = memory.view()[0].get();

                Ok(shift_ + n + 1)
            }),

            // Closure with a captured environment (a single variable) and with an explicit `vmctx`.
            "callback_closure_with_vmctx_and_env" => Func::new(move |vmctx: &mut vm::Ctx, n: i32| -> Result<i32, ()> {
                let memory = vmctx.memory(0);
                let shift_ = shift + memory.view::<i32>()[0].get();

                Ok(shift_ + n + 1)
            }),

            // Regular “variadic” function.
            "callback_fn_variadic" => Func::new_variadic(
                callback_fn_variadic,
                1,
                Arc::new(FuncSig::new(vec![Type::I32], vec![Type::I32]))
            ),

            // “Variadic” closures.
            "callback_closure_variadic_0" => Func::new_variadic(
                |_vmctx: &mut vm::Ctx, inputs: &[Value]| -> Result<Vec<Value>, ()> {
                    assert_eq!(inputs.len(), 0);

                    Ok(vec![])

                },
                0,
                Arc::new(FuncSig::new(vec![], vec![])),
            ),
            "callback_closure_variadic_1" => Func::new_variadic(
                move |vmctx: &mut vm::Ctx, inputs: &[Value]| -> Result<Vec<Value>, ()> {
                    assert_eq!(inputs.len(), 1);

                    let memory = vmctx.memory(0);
                    let shift_ = shift + memory.view::<i32>()[0].get();
                    let n: i32 = (&inputs[0]).try_into().unwrap();

                    Ok(vec![Value::I32(shift_ + n)])
                },
                1,
                Arc::new(FuncSig::new(vec![Type::I32], vec![Type::I32])),
            ),
            "callback_closure_variadic_2" => Func::new_variadic(
                move |vmctx: &mut vm::Ctx, inputs: &[Value]| -> Result<Vec<Value>, ()> {
                    assert_eq!(inputs.len(), 2);

                    let memory = vmctx.memory(0);
                    let shift_ = shift + memory.view::<i32>()[0].get();
                    let i: i32 = (&inputs[0]).try_into().unwrap();
                    let j: i64 = (&inputs[1]).try_into().unwrap();

                    Ok(vec![Value::I64(shift_ as i64 + i as i64 + j)])
                },
                2,
                Arc::new(FuncSig::new(vec![Type::I32, Type::I64], vec![Type::I64])),
            ),
            "callback_closure_variadic_3" => Func::new_variadic(
                move |vmctx: &mut vm::Ctx, inputs: &[Value]| -> Result<Vec<Value>, ()> {
                    assert_eq!(inputs.len(), 3);

                    let memory = vmctx.memory(0);
                    let shift_ = shift + memory.view::<i32>()[0].get();
                    let i: i32 = (&inputs[0]).try_into().unwrap();
                    let j: i64 = (&inputs[1]).try_into().unwrap();
                    let k: f32 = (&inputs[2]).try_into().unwrap();

                    Ok(vec![Value::F32(shift_ as f32 + i as f32 + j as f32 + k)])
                },
                3,
                Arc::new(FuncSig::new(vec![Type::I32, Type::I64, Type::F32], vec![Type::F32])),
            ),
            "callback_closure_variadic_4" => Func::new_variadic(
                |vmctx: &mut vm::Ctx, inputs: &[Value]| -> Result<Vec<Value>, ()> {
                    assert_eq!(inputs.len(), 4);

                    let memory = vmctx.memory(0);
                    let shift_ = shift + memory.view::<i32>()[0].get();
                    let i: i32 = (&inputs[0]).try_into().unwrap();
                    let j: i64 = (&inputs[1]).try_into().unwrap();
                    let k: f32 = (&inputs[2]).try_into().unwrap();
                    let l: f64 = (&inputs[3]).try_into().unwrap();

                    Ok(vec![Value::F64(shift_ as f64 + i as f64 + j as f64 + k as f64 + l)])
                },
                4,
                Arc::new(FuncSig::new(vec![Type::I32, Type::I64, Type::F32, Type::F64], vec![Type::F64])),
            ),

            // Trap a regular function.
            "callback_fn_trap" => Func::new(callback_fn_trap),

            // Trap a closure without a captured environment.
            "callback_closure_trap" => Func::new(|n: i32| -> Result<i32, String> {
                Err(format!("bar {}", n + 1))
            }),

            // Trap a regular function with an explicit `vmctx`.
            "callback_fn_trap_with_vmctx" => Func::new(callback_fn_trap_with_vmctx),

            // Trap a closure without a captured environment but with an explicit `vmctx`.
            "callback_closure_trap_with_vmctx" => Func::new(|vmctx: &mut vm::Ctx, n: i32| -> Result<i32, String> {
                let memory = vmctx.memory(0);
                let shift_: i32 = memory.view()[0].get();

                Err(format!("qux {}", shift_ + n + 1))
            }),

            // Trap a closure with a captured environment (a single variable) and with an explicit `vmctx`.
            "callback_closure_trap_with_vmctx_and_env" => Func::new(move |vmctx: &mut vm::Ctx, n: i32| -> Result<i32, String> {
                let memory = vmctx.memory(0);
                let shift_ = shift + memory.view::<i32>()[0].get();

                Err(format!("! {}", shift_ + n + 1))
            }),
        },
    };
    let instance = module.instantiate(&import_object).unwrap();

    test(&instance);
}

fn callback_fn(n: i32) -> Result<i32, ()> {
    Ok(n + 1)
}

fn callback_fn_variadic(vmctx: &mut vm::Ctx, inputs: &[Value]) -> Result<Vec<Value>, ()> {
    let memory = vmctx.memory(0);
    let shift_: i32 = memory.view()[0].get();
    let n: i32 = (&inputs[0]).try_into().unwrap();

    Ok(vec![Value::I32(shift_ + n + 1)])
}

fn callback_fn_with_vmctx(vmctx: &mut vm::Ctx, n: i32) -> Result<i32, ()> {
    let memory = vmctx.memory(0);
    let shift_: i32 = memory.view()[0].get();

    Ok(shift_ + n + 1)
}

fn callback_fn_trap(n: i32) -> Result<i32, String> {
    Err(format!("foo {}", n + 1))
}

fn callback_fn_trap_with_vmctx(vmctx: &mut vm::Ctx, n: i32) -> Result<i32, String> {
    let memory = vmctx.memory(0);
    let shift_: i32 = memory.view()[0].get();

    Err(format!("baz {}", shift_ + n + 1))
}

macro_rules! test {
    ($test_name:ident, $function:ident( $( $inputs:ty ),* ) -> $output:ty, ( $( $arguments:expr ),* ) == $expected_value:expr) => {
        #[test]
        fn $test_name() {
            imported_functions_forms(&|instance| {
                call_and_assert!(instance, $function( $( $inputs ),* ) -> $output, ( $( $arguments ),* ) == $expected_value);
            });
        }
    };
}

test!(test_fn, function_fn(i32) -> i32, (1) == Ok(2));
test!(test_closure, function_closure(i32) -> i32, (1) == Ok(2));
test!(
    test_closure_with_env,
    function_closure_with_env(i32) -> i32,
    (1) == Ok(2 + shift + SHIFT)
);
test!(test_fn_with_vmctx, function_fn_with_vmctx(i32) -> i32, (1) == Ok(2 + SHIFT));
test!(
    test_closure_with_vmctx,
    function_closure_with_vmctx(i32) -> i32,
    (1) == Ok(2 + SHIFT)
);
test!(
    test_closure_with_vmctx_and_env,
    function_closure_with_vmctx_and_env(i32) -> i32,
    (1) == Ok(2 + shift + SHIFT)
);
test!(test_fn_variadic, function_fn_variadic(i32) -> i32, (1) == Ok(2 + SHIFT));
test!(
    test_closure_variadic_arity_0,
    function_closure_variadic_0(()) -> (),
    () == Ok(())
);
test!(
    test_closure_variadic_arity_1,
    function_closure_variadic_1(i32) -> (i32),
    (1) == Ok(1 + shift + SHIFT)
);
test!(
    test_closure_variadic_arity_2,
    function_closure_variadic_2(i32, i64) -> (i64),
    (1, 2) == Ok(1 + 2 + shift as i64 + SHIFT as i64)
);
/*
test!(
    test_closure_variadic_arity_3,
    function_closure_variadic_3(i32, i64, f32) -> (f32),
    (1, 2, 3.0) == Ok(1.0 + 2.0 + 3.0 + shift as f32 + SHIFT as f32)
);
*/
/*
test!(
    test_closure_variadic_arity_4,
    function_closure_variadic_4(i32, i64, f32, f64) -> (f64),
    (1, 2, 3.0, 4.0) == Ok(1.0 + 2.0 + 3.0 + 4.0 + shift as f64 + SHIFT as f64)
);
*/
test!(
    test_fn_trap,
    function_fn_trap(i32) -> i32,
    (1) == Err(RuntimeError::Error {
        data: Box::new(format!("foo {}", 2))
    })
);
test!(
    test_closure_trap,
    function_closure_trap(i32) -> i32,
    (1) == Err(RuntimeError::Error {
        data: Box::new(format!("bar {}", 2))
    })
);
test!(
    test_fn_trap_with_vmctx,
    function_fn_trap_with_vmctx(i32) -> i32,
    (1) == Err(RuntimeError::Error {
        data: Box::new(format!("baz {}", 2 + SHIFT))
    })
);
test!(
    test_closure_trap_with_vmctx,
    function_closure_trap_with_vmctx(i32) -> i32,
    (1) == Err(RuntimeError::Error {
        data: Box::new(format!("qux {}", 2 + SHIFT))
    })
);
test!(
    test_closure_trap_with_vmctx_and_env,
    function_closure_trap_with_vmctx_and_env(i32) -> i32,
    (1) == Err(RuntimeError::Error {
        data: Box::new(format!("! {}", 2 + shift + SHIFT))
    })
);
