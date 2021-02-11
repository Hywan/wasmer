//! Unstable non-standard Wasmer-specific types for the
//! `wasm_engine_t` and siblings.

use super::super::engine::wasm_config_t;
use super::target_lexicon::wasmer_target_t;

/// Unstable non-standard Wasmer-specific API to update the
/// configuration to specify a particular target for the engine.
///
/// # Example
///
/// ```rust
/// # use inline_c::assert_c;
/// # fn main() {
/// #    (assert_c! {
/// # #include "tests/wasmer_wasm.h"
/// #
/// int main() {
///     // Create the configuration.
///     wasm_config_t* config = wasm_config_new();
///
///     // Set the target.
///     {
///         wasmer_triple_t* triple = wasmer_triple_new_from_host();
///         wasmer_cpu_features_t* cpu_features = wasmer_cpu_features_new();
///         wasmer_target_t* target = wasmer_target_new(triple, cpu_features);
///
///         wasmer_config_set_target(config, target);
///     }
///
///     // Create the engine.
///     wasm_engine_t* engine = wasmer_engine_new_with_config(config);
///
///     // Check we have an engine!
///     assert(engine);
///
///     // Free everything.
///     wasm_engine_delete(engine);
///
///     return 0;
/// }
/// #    })
/// #    .success();
/// # }
/// ```
#[no_mangle]
pub extern "C" fn wasmer_config_set_target(
    config: &mut wasm_config_t,
    target: Box<wasmer_target_t>,
) {
    config.target = Some(target);
}
