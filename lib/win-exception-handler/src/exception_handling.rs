use std::ptr::NonNull;
use wasmer_runtime_core::{
    typed_func::Trampoline,
    vm::{self, Ctx},
};

type CallProtectedResult = Result<(), CallProtectedData>;

#[repr(C)]
pub struct CallProtectedData {
    pub code: u64,
    pub exception_address: u64,
    pub instruction_pointer: u64,
}

extern "C" {
    #[link_name = "callProtected"]
    pub fn __call_protected(
        trampoline: Trampoline,
        vmctx: Option<NonNull<vm::Ctx>>,
        func_env: Option<NonNull<vm::FuncEnv>>,
        func: NonNull<vm::Func>,
        param_vec: *const u64,
        return_vec: *mut u64,
        out_result: *mut CallProtectedData,
    ) -> u8;
}

pub fn _call_protected(
    trampoline: Trampoline,
    vmctx: Option<NonNull<vm::Ctx>>,
    func_env: Option<NonNull<vm::FuncEnv>>,
    func: NonNull<vm::Func>,
    param_vec: *const u64,
    return_vec: *mut u64,
) -> CallProtectedResult {
    let mut out_result = CallProtectedData {
        code: 0,
        exception_address: 0,
        instruction_pointer: 0,
    };
    let result = unsafe {
        __call_protected(
            trampoline,
            vmctx,
            func_env,
            func,
            param_vec,
            return_vec,
            &mut out_result,
        )
    };

    if result == 1 {
        Ok(())
    } else {
        Err(out_result)
    }
}
