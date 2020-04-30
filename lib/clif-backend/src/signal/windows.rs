use crate::{
    relocation::{TrapCode, TrapData},
    signal::HandlerData,
};
use std::{
    cell::Cell,
    ffi::c_void,
    ptr::{self, NonNull},
};
use wasmer_runtime_core::{
    backend::ExceptionCode,
    error::InvokeError,
    typed_func::Trampoline,
    vm::{Ctx, Func},
};
use wasmer_win_exception_handler::CallProtectedData;
pub use wasmer_win_exception_handler::_call_protected;
use winapi::{
    shared::minwindef::DWORD,
    um::minwinbase::{
        EXCEPTION_ACCESS_VIOLATION, EXCEPTION_ARRAY_BOUNDS_EXCEEDED, EXCEPTION_BREAKPOINT,
        EXCEPTION_DATATYPE_MISALIGNMENT, EXCEPTION_FLT_DENORMAL_OPERAND,
        EXCEPTION_FLT_DIVIDE_BY_ZERO, EXCEPTION_FLT_INEXACT_RESULT,
        EXCEPTION_FLT_INVALID_OPERATION, EXCEPTION_FLT_OVERFLOW, EXCEPTION_FLT_STACK_CHECK,
        EXCEPTION_FLT_UNDERFLOW, EXCEPTION_GUARD_PAGE, EXCEPTION_ILLEGAL_INSTRUCTION,
        EXCEPTION_INT_DIVIDE_BY_ZERO, EXCEPTION_INT_OVERFLOW, EXCEPTION_INVALID_HANDLE,
        EXCEPTION_IN_PAGE_ERROR, EXCEPTION_NONCONTINUABLE_EXCEPTION, EXCEPTION_POSSIBLE_DEADLOCK,
        EXCEPTION_PRIV_INSTRUCTION, EXCEPTION_SINGLE_STEP, EXCEPTION_STACK_OVERFLOW,
    },
};

thread_local! {
    pub static CURRENT_EXECUTABLE_BUFFER: Cell<*const c_void> = Cell::new(ptr::null());
}

fn get_signal_name(code: DWORD) -> &'static str {
    match code {
        EXCEPTION_FLT_DENORMAL_OPERAND
        | EXCEPTION_FLT_DIVIDE_BY_ZERO
        | EXCEPTION_FLT_INEXACT_RESULT
        | EXCEPTION_FLT_INVALID_OPERATION
        | EXCEPTION_FLT_OVERFLOW
        | EXCEPTION_FLT_STACK_CHECK
        | EXCEPTION_FLT_UNDERFLOW => "floating-point exception",
        EXCEPTION_ILLEGAL_INSTRUCTION => "illegal instruction",
        EXCEPTION_ACCESS_VIOLATION => "segmentation violation",
        EXCEPTION_DATATYPE_MISALIGNMENT => "datatype misalignment",
        EXCEPTION_BREAKPOINT => "breakpoint",
        EXCEPTION_SINGLE_STEP => "single step",
        EXCEPTION_ARRAY_BOUNDS_EXCEEDED => "array bounds exceeded",
        EXCEPTION_INT_DIVIDE_BY_ZERO => "integer division by zero",
        EXCEPTION_INT_OVERFLOW => "integer overflow",
        EXCEPTION_PRIV_INSTRUCTION => "privileged instruction",
        EXCEPTION_IN_PAGE_ERROR => "in page error",
        EXCEPTION_NONCONTINUABLE_EXCEPTION => "non continuable exception",
        EXCEPTION_STACK_OVERFLOW => "stack overflow",
        EXCEPTION_GUARD_PAGE => "guard page",
        EXCEPTION_INVALID_HANDLE => "invalid handle",
        EXCEPTION_POSSIBLE_DEADLOCK => "possible deadlock",
        _ => "unknown exception code",
    }
}

pub fn call_protected(
    handler_data: &HandlerData,
    trampoline: Trampoline,
    ctx: *mut Ctx,
    func: NonNull<Func>,
    param_vec: *const u64,
    return_vec: *mut u64,
) -> Result<(), InvokeError> {
    // TODO: trap early
    // user code error
    //    if let Some(msg) = super::TRAP_EARLY_DATA.with(|cell| cell.replace(None)) {
    //        return Err(RuntimeError::User { msg });
    //    }

    let result = _call_protected(trampoline, ctx, func, param_vec, return_vec);

    if let Ok(_) = result {
        return Ok(());
    }

    let CallProtectedData {
        code,
        exception_address,
        instruction_pointer,
    } = result.unwrap_err();

    if let Some(TrapData { trapcode, srcloc }) = handler_data.lookup(instruction_pointer as _) {
        let exception_code = match code as DWORD {
            EXCEPTION_ACCESS_VIOLATION => ExceptionCode::MemoryOutOfBounds,
            EXCEPTION_ILLEGAL_INSTRUCTION => match trapcode {
                TrapCode::BadSignature => ExceptionCode::IncorrectCallIndirectSignature,
                TrapCode::IndirectCallToNull => ExceptionCode::CallIndirectOOB,
                TrapCode::HeapOutOfBounds => ExceptionCode::MemoryOutOfBounds,
                TrapCode::TableOutOfBounds => ExceptionCode::CallIndirectOOB,
                TrapCode::UnreachableCodeReached => ExceptionCode::Unreachable,
                _ => {
                    return Err(InvokeError::UnknownTrapCode {
                        trap_code: format!("{}", code as DWORD),
                        srcloc,
                    })
                }
            },
            EXCEPTION_STACK_OVERFLOW => ExceptionCode::MemoryOutOfBounds,
            EXCEPTION_INT_DIVIDE_BY_ZERO | EXCEPTION_INT_OVERFLOW => {
                ExceptionCode::IllegalArithmetic
            }
            _ => {
                let signal = get_signal_name(code as DWORD);
                return Err(InvokeError::UnknownTrap {
                    address: exception_address as usize,
                    signal,
                });
            }
        };
        return Err(InvokeError::TrapCode {
            srcloc,
            code: exception_code,
        });
    } else {
        let signal = get_signal_name(code as DWORD);

        Err(InvokeError::UnknownTrap {
            address: exception_address as usize,
            signal,
        })
    }
}

pub unsafe fn trigger_trap() -> ! {
    // TODO
    unimplemented!("windows::trigger_trap");
}
