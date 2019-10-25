#ifndef WASMER_EXCEPTION_HANDLING_H
#define WASMER_EXCEPTION_HANDLING_H

#include <stdint.h>

typedef struct vmctx_t vmctx_t;
typedef struct funcenv_t funcenv_t;
typedef struct func_t func_t;

typedef void(*trampoline_t)(const vmctx_t*, const funcenv_t*,  const func_t*, const uint64_t*, uint64_t*);

typedef struct call_protected_result_t {
    uint64_t code;
    uint64_t exception_address;
    uint64_t instruction_pointer;
} call_protected_result_t;

uint8_t callProtected(
    trampoline_t trampoline,
    const vmctx_t* vmctx,
    const funcenv_t* func_env,
    const func_t* func,
    const uint64_t* param_vec,
    uint64_t* return_vec,
    call_protected_result_t* out_result
);

#endif //WASMER_EXCEPTION_HANDLING_H
