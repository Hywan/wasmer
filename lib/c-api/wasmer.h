// The Wasmer C/C++ header file.

#if !defined(WASMER_H_PRELUDE)

#define WASMER_H_PRELUDE

// Define the `ARCH_X86_X64` constant.
#if defined(MSVC) && defined(_M_AMD64)
#  define ARCH_X86_64
#elif (defined(GCC) || defined(__GNUC__) || defined(__clang__)) && defined(__x86_64__)
#  define ARCH_X86_64
#endif

// Compatibility with non-Clang compilers.
#if !defined(__has_attribute)
#  define __has_attribute(x) 0
#endif

// Compatibility with non-Clang compilers.
#if !defined(__has_declspec_attribute)
#  define __has_declspec_attribute(x) 0
#endif

// Define the `DEPRECATED` macro.
#if defined(GCC) || defined(__GNUC__) || __has_attribute(deprecated)
#  define DEPRECATED(message) __attribute__((deprecated(message)))
#elif defined(MSVC) || __has_declspec_attribute(deprecated)
#  define DEPRECATED(message) __declspec(deprecated(message))
#endif

// The `wasi` feature has been enabled for this build.
#define WASMER_WASI_ENABLED

// This file corresponds to the following Wasmer version.
#define WASMER_VERSION "1.0.1"
#define WASMER_VERSION_MAJOR 1
#define WASMER_VERSION_MINOR 0
#define WASMER_VERSION_PATCH 1
#define WASMER_VERSION_PRE ""

#endif // WASMER_H_PRELUDE


//
// OK, here we go. The code below is automatically generated.
//


#ifndef WASMER_H
#define WASMER_H

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#if defined(WASMER_WASI_ENABLED)
enum Version {
#if defined(WASMER_WASI_ENABLED)
  /**
   * Version cannot be detected or is unknown.
   */
  Unknown = 0,
#endif
#if defined(WASMER_WASI_ENABLED)
  /**
   * Latest version. See `wasmer_wasi::WasiVersion::Latest` to
   * learn more.
   */
  Latest = 1,
#endif
#if defined(WASMER_WASI_ENABLED)
  /**
   * `wasi_unstable`.
   */
  Snapshot0 = 2,
#endif
#if defined(WASMER_WASI_ENABLED)
  /**
   * `wasi_snapshot_preview1`.
   */
  Snapshot1 = 3,
#endif
};
typedef uint8_t Version;
#endif

/**
 * List of export/import kinds.
 */
enum wasmer_import_export_kind {
  /**
   * The export/import is a function.
   */
  WASM_FUNCTION = 0,
  /**
   * The export/import is a global.
   */
  WASM_GLOBAL = 1,
  /**
   * The export/import is a memory.
   */
  WASM_MEMORY = 2,
  /**
   * The export/import is a table.
   */
  WASM_TABLE = 3,
};
typedef uint32_t wasmer_import_export_kind;

/**
 * The `wasmer_result_t` enum is a type that represents either a
 * success, or a failure.
 */
typedef enum {
  /**
   * Represents a success.
   */
  WASMER_OK = 1,
  /**
   * Represents a failure.
   */
  WASMER_ERROR = 2,
} wasmer_result_t;

/**
 * Represents all possibles WebAssembly value types.
 *
 * See `wasmer_value_t` to get a complete example.
 */
enum wasmer_value_tag {
  /**
   * Represents the `i32` WebAssembly type.
   */
  WASM_I32,
  /**
   * Represents the `i64` WebAssembly type.
   */
  WASM_I64,
  /**
   * Represents the `f32` WebAssembly type.
   */
  WASM_F32,
  /**
   * Represents the `f64` WebAssembly type.
   */
  WASM_F64,
};
typedef uint32_t wasmer_value_tag;

typedef struct {

} wasmer_module_t;

/**
 * Opaque pointer to `NamedExportDescriptor`.
 */
typedef struct {

} wasmer_export_descriptor_t;

typedef struct {
  const uint8_t *bytes;
  uint32_t bytes_len;
} wasmer_byte_array;

/**
 * Opaque pointer to `NamedExportDescriptors`.
 */
typedef struct {

} wasmer_export_descriptors_t;

/**
 * Opaque pointer to `wasmer_export_t`.
 */
typedef struct {

} wasmer_export_func_t;

/**
 * Represents a WebAssembly value.
 *
 * This is a [Rust union][rust-union], which is equivalent to the C
 * union. See `wasmer_value_t` to get a complete example.
 *
 * [rust-union]: https://doc.rust-lang.org/reference/items/unions.html
 */
typedef union {
  int32_t I32;
  int64_t I64;
  float F32;
  double F64;
} wasmer_value;

/**
 * Represents a WebAssembly type and value pair,
 * i.e. `wasmer_value_tag` and `wasmer_value`. Since the latter is an
 * union, it's the safe way to read or write a WebAssembly value in
 * C.
 *
 * Example:
 *
 * ```c
 * // Create a WebAssembly value.
 * wasmer_value_t wasm_value = {
 *     .tag = WASM_I32,
 *     .value.I32 = 42,
 * };
 *
 * // Read a WebAssembly value.
 * if (wasm_value.tag == WASM_I32) {
 *     int32_t x = wasm_value.value.I32;
 *     // …
 * }
 * ```
 */
typedef struct {
  /**
   * The value type.
   */
  wasmer_value_tag tag;
  /**
   * The value.
   */
  wasmer_value value;
} wasmer_value_t;

/**
 * Opaque pointer to `ImportType`.
 */
typedef struct {

} wasmer_export_t;

/**
 * Opaque pointer to a `wasmer_vm::Memory` value in Rust.
 *
 * A `wasmer_vm::Memory` represents a WebAssembly memory. It is
 * possible to create one with `wasmer_memory_new()` and pass it as
 * imports of an instance, or to read it from exports of an instance
 * with `wasmer_export_to_memory()`.
 */
typedef struct {

} wasmer_memory_t;

/**
 * Opaque pointer to the opaque structure
 * `crate::deprecated::NamedExports`, which is a wrapper around a
 * vector of the opaque structure `crate::deprecated::NamedExport`.
 *
 * Check the `wasmer_instance_exports()` function to learn more.
 */
typedef struct {

} wasmer_exports_t;

typedef struct {

} wasmer_global_t;

typedef struct {
  bool mutable_;
  wasmer_value_tag kind;
} wasmer_global_descriptor_t;

typedef struct {

} wasmer_import_descriptor_t;

typedef struct {

} wasmer_import_descriptors_t;

typedef struct {

} wasmer_import_func_t;

typedef struct {

} wasmer_import_object_t;

typedef struct {

} wasmer_table_t;

/**
 * Union of import/export value.
 */
typedef union {
  const wasmer_import_func_t *func;
  const wasmer_table_t *table;
  const wasmer_memory_t *memory;
  const wasmer_global_t *global;
} wasmer_import_export_value;

typedef struct {
  wasmer_byte_array module_name;
  wasmer_byte_array import_name;
  wasmer_import_export_kind tag;
  wasmer_import_export_value value;
} wasmer_import_t;

typedef struct {

} wasmer_import_object_iter_t;

/**
 * Opaque pointer to an Instance type plus metadata.
 *
 * This type represents a WebAssembly instance. It
 * is generally generated by the `wasmer_instantiate()` function, or by
 * the `wasmer_module_instantiate()` function for the most common paths.
 */
typedef struct {

} wasmer_instance_t;

/**
 * Opaque pointer to a `wasmer_vm::Ctx` value in Rust.
 *
 * An instance context is passed to any host function (aka imported
 * function) as the first argument. It is necessary to read the
 * instance data or the memory, respectively with the
 * `wasmer_instance_context_data_get()` function, and the
 * `wasmer_instance_context_memory()` function.
 *
 * It is also possible to get the instance context outside a host
 * function by using the `wasmer_instance_context_get()`
 * function. See also `wasmer_instance_context_data_set()` to set the
 * instance context data.
 *
 * Example:
 *
 * ```c
 * // A host function that prints data from the WebAssembly memory to
 * // the standard output.
 * void print(wasmer_instance_context_t *context, int32_t pointer, int32_t length) {
 *     // Use `wasmer_instance_context` to get back the first instance memory.
 *     const wasmer_memory_t *memory = wasmer_instance_context_memory(context, 0);
 *
 *     // Continue…
 * }
 * ```
 */
typedef struct {

} wasmer_instance_context_t;

/**
 * The `wasmer_limit_option_t` struct represents an optional limit
 * for `wasmer_limits_t`.
 */
typedef struct {
  /**
   * Whether the limit is set.
   */
  bool has_some;
  /**
   * The limit value.
   */
  uint32_t some;
} wasmer_limit_option_t;

/**
 * The `wasmer_limits_t` struct is a type that describes the limits of something
 * such as a memory or a table. See the `wasmer_memory_new()` function to get
 * more information.
 */
typedef struct {
  /**
   * The minimum number of allowed pages.
   */
  uint32_t min;
  /**
   * The maximum number of allowed pages.
   */
  wasmer_limit_option_t max;
} wasmer_limits_t;

typedef struct {

} wasmer_serialized_module_t;

#if defined(WASMER_WASI_ENABLED)
/**
 * Opens a directory that's visible to the WASI module as `alias` but
 * is backed by the host file at `host_file_path`
 */
typedef struct {
  /**
   * What the WASI module will see in its virtual root
   */
  wasmer_byte_array alias;
  /**
   * The backing file that the WASI module will interact with via the alias
   */
  wasmer_byte_array host_file_path;
} wasmer_wasi_map_dir_entry_t;
#endif

/**
 * Creates a new Module from the given wasm bytes.
 *
 * Returns `wasmer_result_t::WASMER_OK` upon success.
 *
 * Returns `wasmer_result_t::WASMER_ERROR` upon failure. Use `wasmer_last_error_length`
 * and `wasmer_last_error_message` to get an error message.
 */
wasmer_result_t wasmer_compile(wasmer_module_t **module,
                               uint8_t *wasm_bytes,
                               uint32_t wasm_bytes_len);

/**
 * Gets export descriptor kind
 */
wasmer_import_export_kind wasmer_export_descriptor_kind(wasmer_export_descriptor_t *export_);

/**
 * Gets name for the export descriptor
 */
wasmer_byte_array wasmer_export_descriptor_name(wasmer_export_descriptor_t *export_descriptor);

/**
 * Gets export descriptors for the given module
 *
 * The caller owns the object and should call `wasmer_export_descriptors_destroy` to free it.
 */
void wasmer_export_descriptors(const wasmer_module_t *module,
                               wasmer_export_descriptors_t **export_descriptors);

/**
 * Frees the memory for the given export descriptors
 */
void wasmer_export_descriptors_destroy(wasmer_export_descriptors_t *export_descriptors);

/**
 * Gets export descriptor by index
 */
wasmer_export_descriptor_t *wasmer_export_descriptors_get(wasmer_export_descriptors_t *export_descriptors,
                                                          int idx);

/**
 * Gets the length of the export descriptors
 */
int wasmer_export_descriptors_len(wasmer_export_descriptors_t *exports);

/**
 * Calls a `func` with the provided parameters.
 * Results are set using the provided `results` pointer.
 *
 * Returns `wasmer_result_t::WASMER_OK` upon success.
 *
 * Returns `wasmer_result_t::WASMER_ERROR` upon failure. Use `wasmer_last_error_length`
 * and `wasmer_last_error_message` to get an error message.
 */
wasmer_result_t wasmer_export_func_call(const wasmer_export_func_t *func,
                                        const wasmer_value_t *params,
                                        unsigned int params_len,
                                        wasmer_value_t *results,
                                        unsigned int results_len);

/**
 * Sets the params buffer to the parameter types of the given wasmer_export_func_t
 *
 * Returns `wasmer_result_t::WASMER_OK` upon success.
 *
 * Returns `wasmer_result_t::WASMER_ERROR` upon failure. Use `wasmer_last_error_length`
 * and `wasmer_last_error_message` to get an error message.
 */
wasmer_result_t wasmer_export_func_params(const wasmer_export_func_t *func,
                                          wasmer_value_tag *params,
                                          uint32_t params_len);

/**
 * Sets the result parameter to the arity of the params of the wasmer_export_func_t
 *
 * Returns `wasmer_result_t::WASMER_OK` upon success.
 *
 * Returns `wasmer_result_t::WASMER_ERROR` upon failure. Use `wasmer_last_error_length`
 * and `wasmer_last_error_message` to get an error message.
 */
wasmer_result_t wasmer_export_func_params_arity(const wasmer_export_func_t *func, uint32_t *result);

/**
 * Sets the returns buffer to the parameter types of the given wasmer_export_func_t
 *
 * Returns `wasmer_result_t::WASMER_OK` upon success.
 *
 * Returns `wasmer_result_t::WASMER_ERROR` upon failure. Use `wasmer_last_error_length`
 * and `wasmer_last_error_message` to get an error message.
 */
wasmer_result_t wasmer_export_func_returns(const wasmer_export_func_t *func,
                                           wasmer_value_tag *returns,
                                           uint32_t returns_len);

/**
 * Sets the result parameter to the arity of the returns of the wasmer_export_func_t
 *
 * Returns `wasmer_result_t::WASMER_OK` upon success.
 *
 * Returns `wasmer_result_t::WASMER_ERROR` upon failure. Use `wasmer_last_error_length`
 * and `wasmer_last_error_message` to get an error message.
 */
wasmer_result_t wasmer_export_func_returns_arity(const wasmer_export_func_t *func,
                                                 uint32_t *result);

/**
 * Gets wasmer_export kind
 */
wasmer_import_export_kind wasmer_export_kind(wasmer_export_t *export_);

/**
 * Gets name from wasmer_export
 */
wasmer_byte_array wasmer_export_name(wasmer_export_t *export_);

/**
 * Gets export func from export
 */
const wasmer_export_func_t *wasmer_export_to_func(const wasmer_export_t *export_);

/**
 * Gets a memory pointer from an export pointer.
 *
 * Returns `wasmer_result_t::WASMER_OK` upon success.
 *
 * Returns `wasmer_result_t::WASMER_ERROR` upon failure. Use `wasmer_last_error_length`
 * and `wasmer_last_error_message` to get an error message.
 */
wasmer_result_t wasmer_export_to_memory(const wasmer_export_t *export_, wasmer_memory_t **memory);

/**
 * Frees the memory for the given exports.
 *
 * Check the `wasmer_instance_exports()` function to get a complete
 * example.
 *
 * If `exports` is a null pointer, this function does nothing.
 *
 * Example:
 *
 * ```c
 * // Get some exports.
 * wasmer_exports_t *exports = NULL;
 * wasmer_instance_exports(instance, &exports);
 *
 * // Destroy the exports.
 * wasmer_exports_destroy(exports);
 * ```
 */
void wasmer_exports_destroy(wasmer_exports_t *exports);

/**
 * Gets wasmer_export by index
 */
wasmer_export_t *wasmer_exports_get(wasmer_exports_t *exports, int idx);

/**
 * Gets the length of the exports
 */
int wasmer_exports_len(wasmer_exports_t *exports);

/**
 * Frees memory for the given Global
 */
void wasmer_global_destroy(wasmer_global_t *global);

/**
 * Gets the value stored by the given Global
 */
wasmer_value_t wasmer_global_get(wasmer_global_t *global);

/**
 * Returns a descriptor (type, mutability) of the given Global
 */
wasmer_global_descriptor_t wasmer_global_get_descriptor(wasmer_global_t *global);

/**
 * Creates a new Global and returns a pointer to it.
 * The caller owns the object and should call `wasmer_global_destroy` to free it.
 */
wasmer_global_t *wasmer_global_new(wasmer_value_t value, bool mutable_);

/**
 * Sets the value stored by the given Global
 */
void wasmer_global_set(wasmer_global_t *global, wasmer_value_t value);

/**
 * Gets export descriptor kind
 */
wasmer_import_export_kind wasmer_import_descriptor_kind(wasmer_import_descriptor_t *export_);

/**
 * Gets module name for the import descriptor
 */
wasmer_byte_array wasmer_import_descriptor_module_name(wasmer_import_descriptor_t *import_descriptor);

/**
 * Gets name for the import descriptor
 */
wasmer_byte_array wasmer_import_descriptor_name(wasmer_import_descriptor_t *import_descriptor);

/**
 * Gets import descriptors for the given module
 *
 * The caller owns the object and should call `wasmer_import_descriptors_destroy` to free it.
 */
void wasmer_import_descriptors(const wasmer_module_t *module,
                               wasmer_import_descriptors_t **import_descriptors);

/**
 * Frees the memory for the given import descriptors
 */
void wasmer_import_descriptors_destroy(wasmer_import_descriptors_t *import_descriptors);

/**
 * Gets import descriptor by index
 */
wasmer_import_descriptor_t *wasmer_import_descriptors_get(wasmer_import_descriptors_t *import_descriptors,
                                                          unsigned int idx);

/**
 * Gets the length of the import descriptors
 */
unsigned int wasmer_import_descriptors_len(wasmer_import_descriptors_t *exports);

/**
 * Frees memory for the given Func
 */
void wasmer_import_func_destroy(wasmer_import_func_t *func);

/**
 * Creates new host function, aka imported function. `func` is a
 * function pointer, where the first argument is the famous `vm::Ctx`
 * (in Rust), or `wasmer_instance_context_t` (in C). All arguments
 * must be typed with compatible WebAssembly native types:
 *
 * | WebAssembly type | C/C++ type |
 * | ---------------- | ---------- |
 * | `i32`            | `int32_t`  |
 * | `i64`            | `int64_t`  |
 * | `f32`            | `float`    |
 * | `f64`            | `double`   |
 *
 * The function pointer must have a lifetime greater than the
 * WebAssembly instance lifetime.
 *
 * The caller owns the object and should call
 * `wasmer_import_func_destroy` to free it.
 */
wasmer_import_func_t *wasmer_import_func_new(void (*func)(void *data),
                                             const wasmer_value_tag *params,
                                             unsigned int params_len,
                                             const wasmer_value_tag *returns,
                                             unsigned int returns_len);

/**
 * Sets the params buffer to the parameter types of the given wasmer_import_func_t
 *
 * Returns `wasmer_result_t::WASMER_OK` upon success.
 *
 * Returns `wasmer_result_t::WASMER_ERROR` upon failure. Use `wasmer_last_error_length`
 * and `wasmer_last_error_message` to get an error message.
 */
wasmer_result_t wasmer_import_func_params(const wasmer_import_func_t *func,
                                          wasmer_value_tag *params,
                                          unsigned int params_len);

/**
 * Sets the result parameter to the arity of the params of the wasmer_import_func_t
 *
 * Returns `wasmer_result_t::WASMER_OK` upon success.
 *
 * Returns `wasmer_result_t::WASMER_ERROR` upon failure. Use `wasmer_last_error_length`
 * and `wasmer_last_error_message` to get an error message.
 */
wasmer_result_t wasmer_import_func_params_arity(const wasmer_import_func_t *func, uint32_t *result);

/**
 * Sets the returns buffer to the parameter types of the given wasmer_import_func_t
 *
 * Returns `wasmer_result_t::WASMER_OK` upon success.
 *
 * Returns `wasmer_result_t::WASMER_ERROR` upon failure. Use `wasmer_last_error_length`
 * and `wasmer_last_error_message` to get an error message.
 */
wasmer_result_t wasmer_import_func_returns(const wasmer_import_func_t *func,
                                           wasmer_value_tag *returns,
                                           unsigned int returns_len);

/**
 * Sets the result parameter to the arity of the returns of the wasmer_import_func_t
 *
 * Returns `wasmer_result_t::WASMER_OK` upon success.
 *
 * Returns `wasmer_result_t::WASMER_ERROR` upon failure. Use `wasmer_last_error_length`
 * and `wasmer_last_error_message` to get an error message.
 */
wasmer_result_t wasmer_import_func_returns_arity(const wasmer_import_func_t *func,
                                                 uint32_t *result);

/**
 * Frees memory of the given ImportObject
 */
void wasmer_import_object_destroy(wasmer_import_object_t *import_object);

/**
 * Extends an existing import object with new imports
 */
wasmer_result_t wasmer_import_object_extend(wasmer_import_object_t *import_object,
                                            const wasmer_import_t *imports,
                                            unsigned int imports_len);

/**
 * Gets an entry from an ImportObject at the name and namespace.
 * Stores `name`, `namespace`, and `import_export_value` in `import`.
 * Thus these must remain valid for the lifetime of `import`.
 *
 * The caller owns all data involved.
 * `import_export_value` will be written to based on `tag`.
 */
wasmer_result_t wasmer_import_object_get_import(const wasmer_import_object_t *import_object,
                                                wasmer_byte_array namespace_,
                                                wasmer_byte_array name,
                                                wasmer_import_t *import,
                                                wasmer_import_export_value *import_export_value,
                                                uint32_t tag);

/**
 * Frees the memory allocated in `wasmer_import_object_iter_next`
 *
 * This function does not free the memory in `wasmer_import_object_t`;
 * it only frees memory allocated while querying a `wasmer_import_object_t`.
 */
void wasmer_import_object_imports_destroy(wasmer_import_t *imports, uint32_t imports_len);

/**
 * Returns true if further calls to `wasmer_import_object_iter_next` will
 * not return any new data
 */
bool wasmer_import_object_iter_at_end(wasmer_import_object_iter_t *import_object_iter);

/**
 * Frees the memory allocated by `wasmer_import_object_iterate_functions`
 */
void wasmer_import_object_iter_destroy(wasmer_import_object_iter_t *import_object_iter);

/**
 * Writes the next value to `import`.  `WASMER_ERROR` is returned if there
 * was an error or there's nothing left to return.
 *
 * To free the memory allocated here, pass the import to `wasmer_import_object_imports_destroy`.
 * To check if the iterator is done, use `wasmer_import_object_iter_at_end`.
 */
wasmer_result_t wasmer_import_object_iter_next(wasmer_import_object_iter_t *import_object_iter,
                                               wasmer_import_t *import);

/**
 * Create an iterator over the functions in the import object.
 * Get the next import with `wasmer_import_object_iter_next`
 * Free the iterator with `wasmer_import_object_iter_destroy`
 */
wasmer_import_object_iter_t *wasmer_import_object_iterate_functions(const wasmer_import_object_t *import_object);

/**
 * Creates a new empty import object.
 * See also `wasmer_import_object_append`
 */
wasmer_import_object_t *wasmer_import_object_new(void);

/**
 * Calls an exported function of a WebAssembly instance by `name`
 * with the provided parameters. The exported function results are
 * stored on the provided `results` pointer.
 *
 * This function returns `wasmer_result_t::WASMER_OK` upon success,
 * `wasmer_result_t::WASMER_ERROR` otherwise. You can use
 * `wasmer_last_error_message()` to get the generated error message.
 *
 * Potential errors are the following:
 *
 *   * `instance` is a null pointer,
 *   * `name` is a null pointer,
 *   * `params` is a null pointer.
 *
 * Example of calling an exported function that needs two parameters, and returns one value:
 *
 * ```c
 * // First argument.
 * wasmer_value_t argument_one = {
 *     .tag = WASM_I32,
 *     .value.I32 = 3,
 * };
 *
 * // Second argument.
 * wasmer_value_t argument_two = {
 *     .tag = WASM_I32,
 *     .value.I32 = 4,
 * };
 *
 * // First result.
 * wasmer_value_t result_one;
 *
 * // All arguments and results.
 * wasmer_value_t arguments[] = {argument_one, argument_two};
 * wasmer_value_t results[]   = {result_one};
 *
 * wasmer_result_t call_result = wasmer_instance_call(
 *     instance,  // instance pointer
 *     "sum",     // the exported function name
 *     arguments, // the arguments
 *     2,         // the number of arguments
 *     results,   // the results
 *     1          // the number of results
 * );
 *
 * if (call_result == WASMER_OK) {
 *     printf("Result is: %d\n", results[0].value.I32);
 * }
 * ```
 */
wasmer_result_t wasmer_instance_call(wasmer_instance_t *instance,
                                     const char *name,
                                     const wasmer_value_t *params,
                                     uint32_t params_len,
                                     wasmer_value_t *results,
                                     uint32_t results_len);

/**
 * Gets the data that can be hold by an instance.
 *
 * This function is complementary of
 * `wasmer_instance_context_data_set()`. Please read its
 * documentation. You can also read the documentation of
 * `wasmer_instance_context_t` to get other examples.
 *
 * This function returns nothing if `ctx` is a null pointer.
 */
void *wasmer_instance_context_data_get(const wasmer_instance_context_t *ctx);

/**
 * Sets the data that can be hold by an instance context.
 *
 * An instance context (represented by the opaque
 * `wasmer_instance_context_t` structure) can hold user-defined
 * data. This function sets the data. This function is complementary
 * of `wasmer_instance_context_data_get()`.
 *
 * This function does nothing if `instance` is a null pointer.
 *
 * Example:
 *
 * ```c
 * // Define your own data.
 * typedef struct {
 *     // …
 * } my_data;
 *
 * // Allocate them and set them on the given instance.
 * my_data *data = malloc(sizeof(my_data));
 * data->… = …;
 * wasmer_instance_context_data_set(instance, (void*) data);
 *
 * // You can read your data.
 * {
 *     my_data *data = (my_data*) wasmer_instance_context_data_get(wasmer_instance_context_get(instance));
 *     // …
 * }
 * ```
 */
void wasmer_instance_context_data_set(wasmer_instance_t *instance,
                                      void *data_ptr);

/**
 * Returns the instance context. Learn more by looking at the
 * `wasmer_instance_context_t` struct.
 *
 * This function returns `null` if `instance` is a null pointer.
 *
 * Example:
 *
 * ```c
 * const wasmer_instance_context_get *context = wasmer_instance_context_get(instance);
 * my_data *data = (my_data *) wasmer_instance_context_data_get(context);
 * // Do something with `my_data`.
 * ```
 *
 * It is often useful with `wasmer_instance_context_data_set()`.
 */
const wasmer_instance_context_t *wasmer_instance_context_get(wasmer_instance_t *instance);

/**
 * Gets the `memory_idx`th memory of the instance.
 *
 * Note that the index is always `0` until multiple memories are supported.
 *
 * This function is mostly used inside host functions (aka imported
 * functions) to read the instance memory.
 *
 * Example of a _host function_ that reads and prints a string based on a pointer and a length:
 *
 * ```c
 * void print_string(const wasmer_instance_context_t *context, int32_t pointer, int32_t length) {
 *     // Get the 0th memory.
 *     const wasmer_memory_t *memory = wasmer_instance_context_memory(context, 0);
 *
 *     // Get the memory data as a pointer.
 *     uint8_t *memory_bytes = wasmer_memory_data(memory);
 *
 *     // Print what we assumed to be a string!
 *     printf("%.*s", length, memory_bytes + pointer);
 * }
 * ```
 */
const wasmer_memory_t *wasmer_instance_context_memory(const wasmer_instance_context_t *ctx,
                                                      uint32_t _memory_idx);

/**
 * Frees memory for the given `wasmer_instance_t`.
 *
 * Check the `wasmer_instantiate()` function to get a complete
 * example.
 *
 * If `instance` is a null pointer, this function does nothing.
 *
 * Example:
 *
 * ```c
 * // Get an instance.
 * wasmer_instance_t *instance = NULL;
 * wasmer_instantiate(&instance, bytes, bytes_length, imports, 0);
 *
 * // Destroy the instance.
 * wasmer_instance_destroy(instance);
 * ```
 */
void wasmer_instance_destroy(wasmer_instance_t *instance);

/**
 * Gets all the exports of the given WebAssembly instance.
 *
 * This function stores a Rust vector of exports into `exports` as an
 * opaque pointer of kind `wasmer_exports_t`.
 *
 * As is, you can do anything with `exports` except using the
 * companion functions, like `wasmer_exports_len()`,
 * `wasmer_exports_get()` or `wasmer_export_kind()`. See the example below.
 *
 * **Warning**: The caller owns the object and should call
 * `wasmer_exports_destroy()` to free it.
 *
 * Example:
 *
 * ```c
 * // Get the exports.
 * wasmer_exports_t *exports = NULL;
 * wasmer_instance_exports(instance, &exports);
 *
 * // Get the number of exports.
 * int exports_length = wasmer_exports_len(exports);
 * printf("Number of exports: %d\n", exports_length);
 *
 * // Read the first export.
 * wasmer_export_t *export = wasmer_exports_get(exports, 0);
 *
 * // Get the kind of the export.
 * wasmer_import_export_kind export_kind = wasmer_export_kind(export);
 *
 * // Assert it is a function (why not).
 * assert(export_kind == WASM_FUNCTION);
 *
 * // Read the export name.
 * wasmer_byte_array name_bytes = wasmer_export_name(export);
 *
 * assert(name_bytes.bytes_len == sizeof("sum") - 1);
 * assert(memcmp(name_bytes.bytes, "sum", sizeof("sum") - 1) == 0);
 *
 * // Destroy the exports.
 * wasmer_exports_destroy(exports);
 * ```
 */
void wasmer_instance_exports(wasmer_instance_t *instance, wasmer_exports_t **exports);

/**
 * Creates a new WebAssembly instance from the given bytes and imports.
 *
 * The result is stored in the first argument `instance` if
 * successful, i.e. when the function returns
 * `wasmer_result_t::WASMER_OK`. Otherwise
 * `wasmer_result_t::WASMER_ERROR` is returned, and
 * `wasmer_last_error_length()` with `wasmer_last_error_message()` must
 * be used to read the error message.
 *
 * The caller is responsible to free the instance with
 * `wasmer_instance_destroy()`.
 *
 * Example:
 *
 * ```c
 * // 1. Read a WebAssembly module from a file.
 * FILE *file = fopen("sum.wasm", "r");
 * fseek(file, 0, SEEK_END);
 * long bytes_length = ftell(file);
 * uint8_t *bytes = malloc(bytes_length);
 * fseek(file, 0, SEEK_SET);
 * fread(bytes, 1, bytes_length, file);
 * fclose(file);
 *
 * // 2. Declare the imports (here, none).
 * wasmer_import_t imports[] = {};
 *
 * // 3. Instantiate the WebAssembly module.
 * wasmer_instance_t *instance = NULL;
 * wasmer_result_t result = wasmer_instantiate(&instance, bytes, bytes_length, imports, 0);
 *
 * // 4. Check for errors.
 * if (result != WASMER_OK) {
 *     int error_length = wasmer_last_error_length();
 *     char *error = malloc(error_length);
 *     wasmer_last_error_message(error, error_length);
 *     // Do something with `error`…
 * }
 *
 * // 5. Free the memory!
 * wasmer_instance_destroy(instance);
 * ```
 */
wasmer_result_t wasmer_instantiate(wasmer_instance_t **instance,
                                   uint8_t *wasm_bytes,
                                   uint32_t wasm_bytes_len,
                                   wasmer_import_t *imports,
                                   int imports_len);

/**
 * Gets the length in bytes of the last error if any, zero otherwise.
 *
 * This can be used to dynamically allocate a buffer with the correct number of
 * bytes needed to store a message.
 *
 * # Example
 *
 * See this module's documentation to get a complete example.
 */
int wasmer_last_error_length(void);

/**
 * Gets the last error message if any into the provided buffer
 * `buffer` up to the given `length`.
 *
 * The `length` parameter must be large enough to store the last
 * error message. Ideally, the value should come from
 * [`wasmer_last_error_length`].
 *
 * The function returns the length of the string in bytes, `-1` if an
 * error occurs. Potential errors are:
 *
 *  * The `buffer` is a null pointer,
 *  * The `buffer` is too small to hold the error message.
 *
 * Note: The error message always has a trailing NUL character.
 *
 * Important note: If the provided `buffer` is non-null, once this
 * function has been called, regardless whether it fails or succeeds,
 * the error is cleared.
 *
 * # Example
 *
 * See this module's documentation to get a complete example.
 */
int wasmer_last_error_message(char *buffer, int length);

/**
 * Gets a pointer to the beginning of the contiguous memory data
 * bytes.
 *
 * The function returns `NULL` if `memory` is a null pointer.
 *
 * Note that when the memory grows, it can be reallocated, and thus
 * the returned pointer can be invalidated.
 *
 * Example:
 *
 * ```c
 * uint8_t *memory_data = wasmer_memory_data(memory);
 * char *str = (char*) malloc(sizeof(char) * 7);
 *
 * for (uint32_t nth = 0; nth < 7; ++nth) {
 *     str[nth] = (char) memory_data[nth];
 * }
 * ```
 */
uint8_t *wasmer_memory_data(const wasmer_memory_t *memory);

/**
 * Gets the size in bytes of the memory data.
 *
 * This function returns 0 if `memory` is a null pointer.
 *
 * Example:
 *
 * ```c
 * uint32_t memory_data_length = wasmer_memory_data_length(memory);
 * ```
 */
uint32_t wasmer_memory_data_length(const wasmer_memory_t *memory);

/**
 * Frees memory for the given `wasmer_memory_t`.
 *
 * Check the `wasmer_memory_new()` function to get a complete
 * example.
 *
 * If `memory` is a null pointer, this function does nothing.
 *
 * Example:
 *
 * ```c
 * // Get a memory.
 * wasmer_memory_t *memory = NULL;
 * wasmer_result_t result = wasmer_memory_new(&memory, memory_descriptor);
 *
 * // Destroy the memory.
 * wasmer_memory_destroy(memory);
 * ```
 */
void wasmer_memory_destroy(wasmer_memory_t *memory);

/**
 * Grows a memory by the given number of pages (of 65Kb each).
 *
 * The functions return `wasmer_result_t::WASMER_OK` upon success,
 * `wasmer_result_t::WASMER_ERROR` otherwise. Use
 * `wasmer_last_error_length()` with `wasmer_last_error_message()` to
 * read the error message.
 *
 * Example:
 *
 * ```c
 * wasmer_result_t result = wasmer_memory_grow(memory, 10);
 *
 * if (result != WASMER_OK) {
 *     // …
 * }
 * ```
 */
wasmer_result_t wasmer_memory_grow(wasmer_memory_t *memory, uint32_t delta);

/**
 * Reads the current length (in pages) of the given memory.
 *
 * The function returns zero if `memory` is a null pointer.
 *
 * Example:
 *
 * ```c
 * uint32_t memory_length = wasmer_memory_length(memory);
 *
 * printf("Memory pages length: %d\n", memory_length);
 * ```
 */
uint32_t wasmer_memory_length(const wasmer_memory_t *memory);

/**
 * Creates a new empty WebAssembly memory for the given descriptor.
 *
 * The result is stored in the first argument `memory` if successful,
 * i.e. when the function returns
 * `wasmer_result_t::WASMER_OK`. Otherwise,
 * `wasmer_result_t::WASMER_ERROR` is returned, and
 * `wasmer_last_error_length()` with `wasmer_last_error_message()`
 * must be used to read the error message.
 *
 * The caller owns the memory and is responsible to free it with
 * `wasmer_memory_destroy()`.
 *
 * Example:
 *
 * ```c
 * // 1. The memory object.
 * wasmer_memory_t *memory = NULL;
 *
 * // 2. The memory descriptor.
 * wasmer_limits_t memory_descriptor = {
 *     .min = 10,
 *     .max = {
 *         .has_some = true,
 *         .some = 15,
 *     },
 * };
 *
 * // 3. Initialize the memory.
 * wasmer_result_t result = wasmer_memory_new(&memory, memory_descriptor);
 *
 * if (result != WASMER_OK) {
 *     int error_length = wasmer_last_error_length();
 *     char *error = malloc(error_length);
 *     wasmer_last_error_message(error, error_length);
 *     // Do something with `error`…
 * }
 *
 * // 4. Free the memory!
 * wasmer_memory_destroy(memory);
 * ```
 */
wasmer_result_t wasmer_memory_new(wasmer_memory_t **memory, wasmer_limits_t limits);

/**
 * Deserialize the given serialized module.
 *
 * Returns `wasmer_result_t::WASMER_OK` upon success.
 *
 * Returns `wasmer_result_t::WASMER_ERROR` upon failure. Use `wasmer_last_error_length`
 * and `wasmer_last_error_message` to get an error message.
 */
wasmer_result_t wasmer_module_deserialize(wasmer_module_t **module,
                                          const wasmer_serialized_module_t *serialized_module);

/**
 * Frees memory for the given Module
 */
void wasmer_module_destroy(wasmer_module_t *module);

/**
 * Given:
 * * A prepared `wasmer` import-object
 * * A compiled wasmer module
 *
 * Instantiates a wasmer instance
 */
wasmer_result_t wasmer_module_import_instantiate(wasmer_instance_t **instance,
                                                 const wasmer_module_t *module,
                                                 const wasmer_import_object_t *import_object);

/**
 * Creates a new Instance from the given module and imports.
 *
 * Returns `wasmer_result_t::WASMER_OK` upon success.
 *
 * Returns `wasmer_result_t::WASMER_ERROR` upon failure. Use `wasmer_last_error_length`
 * and `wasmer_last_error_message` to get an error message.
 */
wasmer_result_t wasmer_module_instantiate(const wasmer_module_t *module,
                                          wasmer_instance_t **instance,
                                          wasmer_import_t *imports,
                                          int imports_len);

/**
 * Serialize the given Module.
 *
 * The caller owns the object and should call `wasmer_serialized_module_destroy` to free it.
 *
 * Returns `wasmer_result_t::WASMER_OK` upon success.
 *
 * Returns `wasmer_result_t::WASMER_ERROR` upon failure. Use `wasmer_last_error_length`
 * and `wasmer_last_error_message` to get an error message.
 */
wasmer_result_t wasmer_module_serialize(wasmer_serialized_module_t **serialized_module_out,
                                        const wasmer_module_t *module);

/**
 * Get bytes of the serialized module.
 */
wasmer_byte_array wasmer_serialized_module_bytes(const wasmer_serialized_module_t *serialized_module);

/**
 * Frees memory for the given serialized Module.
 */
void wasmer_serialized_module_destroy(wasmer_serialized_module_t *serialized_module);

/**
 * Transform a sequence of bytes into a serialized module.
 *
 * The caller owns the object and should call `wasmer_serialized_module_destroy` to free it.
 *
 * Returns `wasmer_result_t::WASMER_OK` upon success.
 *
 * Returns `wasmer_result_t::WASMER_ERROR` upon failure. Use `wasmer_last_error_length`
 * and `wasmer_last_error_message` to get an error message.
 */
wasmer_result_t wasmer_serialized_module_from_bytes(wasmer_serialized_module_t **serialized_module,
                                                    const uint8_t *serialized_module_bytes,
                                                    uint32_t serialized_module_bytes_length);

/**
 * Frees memory for the given Table
 */
void wasmer_table_destroy(wasmer_table_t *table);

/**
 * Grows a Table by the given number of elements.
 *
 * Returns `wasmer_result_t::WASMER_OK` upon success.
 *
 * Returns `wasmer_result_t::WASMER_ERROR` upon failure. Use `wasmer_last_error_length`
 * and `wasmer_last_error_message` to get an error message.
 */
wasmer_result_t wasmer_table_grow(wasmer_table_t *table, uint32_t delta);

/**
 * Returns the current length of the given Table
 */
uint32_t wasmer_table_length(wasmer_table_t *table);

/**
 * Creates a new Table for the given descriptor and initializes the given
 * pointer to pointer to a pointer to the new Table.
 *
 * The caller owns the object and should call `wasmer_table_destroy` to free it.
 *
 * Returns `wasmer_result_t::WASMER_OK` upon success.
 *
 * Returns `wasmer_result_t::WASMER_ERROR` upon failure. Use `wasmer_last_error_length`
 * and `wasmer_last_error_message` to get an error message.
 */
wasmer_result_t wasmer_table_new(wasmer_table_t **table, wasmer_limits_t limits);

/**
 * Stop the execution of a host function, aka imported function. The
 * function must be used _only_ inside a host function.
 *
 * The pointer to `wasmer_instance_context_t` is received by the host
 * function as its first argument. Just passing it to `ctx` is fine.
 *
 * The error message must have a greater lifetime than the host
 * function itself since the error is read outside the host function
 * with `wasmer_last_error_message`.
 *
 * This function returns `wasmer_result_t::WASMER_ERROR` if `ctx` or
 * `error_message` are null.
 *
 * This function never returns otherwise.
 */
wasmer_result_t wasmer_trap(const wasmer_instance_context_t *_ctx, const char *error_message);

/**
 * Validates a sequence of bytes hoping it represents a valid WebAssembly module.
 *
 * The function returns true if the bytes are valid, false otherwise.
 *
 * Example:
 *
 * ```c
 * bool result = wasmer_validate(bytes, bytes_length);
 *
 * if (false == result) {
 *     // Do something…
 * }
 * ```
 */
bool wasmer_validate(const uint8_t *wasm_bytes, uint32_t wasm_bytes_len);

/**
 * Get the version of the Wasmer C API.
 *
 * The `.h` files already define variables like `WASMER_VERSION*`,
 * but if this file is unreachable, one can use this function to
 * retrieve the full semver version of the Wasmer C API.
 *
 * The returned string is statically allocated. It must _not_ be
 * freed!
 *
 * # Example
 *
 * See the module's documentation.
 */
const char *wasmer_version(void);

/**
 * Get the major version of the Wasmer C API.
 *
 * See [`wasmer_version`] to learn more.
 *
 * # Example
 *
 * ```rust
 * # use inline_c::assert_c;
 * # fn main() {
 * #    (assert_c! {
 * # #include "tests/wasmer_wasm.h"
 * #
 * int main() {
 *     // Get and print the version components.
 *     uint8_t version_major = wasmer_version_major();
 *     uint8_t version_minor = wasmer_version_minor();
 *     uint8_t version_patch = wasmer_version_patch();
 *
 *     printf("%d.%d.%d", version_major, version_minor, version_patch);
 *
 *     return 0;
 * }
 * #    })
 * #    .success()
 * #    .stdout(
 * #         format!(
 * #             "{}.{}.{}",
 * #             env!("CARGO_PKG_VERSION_MAJOR"),
 * #             env!("CARGO_PKG_VERSION_MINOR"),
 * #             env!("CARGO_PKG_VERSION_PATCH")
 * #         )
 * #     );
 * # }
 * ```
 */
uint8_t wasmer_version_major(void);

/**
 * Get the minor version of the Wasmer C API.
 *
 * See [`wasmer_version_major`] to learn more and get an example.
 */
uint8_t wasmer_version_minor(void);

/**
 * Get the patch version of the Wasmer C API.
 *
 * See [`wasmer_version_major`] to learn more and get an example.
 */
uint8_t wasmer_version_patch(void);

/**
 * Get the minor version of the Wasmer C API.
 *
 * See [`wasmer_version_major`] to learn more.
 *
 * The returned string is statically allocated. It must _not_ be
 * freed!
 *
 * # Example
 *
 * ```rust
 * # use inline_c::assert_c;
 * # fn main() {
 * #    (assert_c! {
 * # #include "tests/wasmer_wasm.h"
 * #
 * int main() {
 *     // Get and print the pre version.
 *     const char* version_pre = wasmer_version_pre();
 *     printf("%s", version_pre);
 *
 *     // No need to free the string. It's statically allocated on
 *     // the Rust side.
 *
 *     return 0;
 * }
 * #    })
 * #    .success()
 * #    .stdout(env!("CARGO_PKG_VERSION_PRE"));
 * # }
 * ```
 */
const char *wasmer_version_pre(void);

#if defined(WASMER_WASI_ENABLED)
/**
 * Convenience function that creates a WASI import object with no arguments,
 * environment variables, preopened files, or mapped directories.
 *
 * This function is the same as calling [`wasmer_wasi_generate_import_object`] with all
 * empty values.
 */
wasmer_import_object_t *wasmer_wasi_generate_default_import_object(void);
#endif

#if defined(WASMER_WASI_ENABLED)
/**
 * Creates a WASI import object.
 *
 * This function treats null pointers as empty collections.
 * For example, passing null for a string in `args`, will lead to a zero
 * length argument in that position.
 */
wasmer_import_object_t *wasmer_wasi_generate_import_object(const wasmer_byte_array *args,
                                                           unsigned int args_len,
                                                           const wasmer_byte_array *envs,
                                                           unsigned int envs_len,
                                                           const wasmer_byte_array *preopened_files,
                                                           unsigned int preopened_files_len,
                                                           const wasmer_wasi_map_dir_entry_t *mapped_dirs,
                                                           unsigned int mapped_dirs_len);
#endif

#if defined(WASMER_WASI_ENABLED)
/**
 * Creates a WASI import object for a specific version.
 *
 * This function is similar to `wasmer_wasi_generate_import_object`
 * except that the first argument describes the WASI version.
 *
 * The version is expected to be of kind `Version`.
 */
wasmer_import_object_t *wasmer_wasi_generate_import_object_for_version(unsigned char version,
                                                                       const wasmer_byte_array *args,
                                                                       unsigned int args_len,
                                                                       const wasmer_byte_array *envs,
                                                                       unsigned int envs_len,
                                                                       const wasmer_byte_array *preopened_files,
                                                                       unsigned int preopened_files_len,
                                                                       const wasmer_wasi_map_dir_entry_t *mapped_dirs,
                                                                       unsigned int mapped_dirs_len);
#endif

#if defined(WASMER_WASI_ENABLED)
/**
 * Find the version of WASI used by the module.
 *
 * In case of error, the returned version is `Version::Unknown`.
 */
Version wasmer_wasi_get_version(const wasmer_module_t *module);
#endif

#endif /* WASMER_H */
