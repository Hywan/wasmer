use crate::{self as core, structures::TypedIndex};
use std::{collections::HashMap, mem, ops::Deref, sync::Arc};
pub use wasmer_interface_types::interpreter::wasm::values::{InterfaceType, InterfaceValue};
use wasmer_interface_types::{self as wit, interpreter::wasm as wit_wasm};

pub mod interpreter {
    pub use wasmer_interface_types::interpreter::{Instruction, Interpreter};
}

pub mod ast {
    pub use wasmer_interface_types::ast::*;
}

pub mod decoders {
    pub use wasmer_interface_types::decoders::*;
}

pub mod encoders {
    pub use wasmer_interface_types::encoders::*;
}

impl From<&core::types::Type> for InterfaceType {
    fn from(ty: &core::types::Type) -> Self {
        match ty {
            core::types::Type::I32 => Self::I32,
            core::types::Type::I64 => Self::I64,
            core::types::Type::F32 => Self::F32,
            core::types::Type::F64 => Self::F64,
            core::types::Type::V128 => unimplemented!(),
        }
    }
}

impl From<&InterfaceValue> for core::types::Value {
    fn from(value: &InterfaceValue) -> Self {
        match value {
            InterfaceValue::I32(v) => Self::I32(*v),
            InterfaceValue::I64(v) => Self::I64(*v),
            InterfaceValue::F32(v) => Self::F32(*v),
            InterfaceValue::F64(v) => Self::F64(*v),
            _ => unimplemented!(),
        }
    }
}

impl From<&core::types::Value> for InterfaceValue {
    fn from(value: &core::types::Value) -> Self {
        match value {
            core::types::Value::I32(v) => Self::I32(*v),
            core::types::Value::I64(v) => Self::I64(*v),
            core::types::Value::F32(v) => Self::F32(*v),
            core::types::Value::F64(v) => Self::F64(*v),
            _ => unimplemented!(),
        }
    }
}

pub struct Export<'function> {
    inner: core::instance::DynFunc<'function>,
    inputs: Vec<InterfaceType>,
    outputs: Vec<InterfaceType>,
}

impl<'function, 'interfaces> Export<'function> {
    fn new(
        export_callable: core::instance::DynFunc<'function>,
        wit_export: &'interfaces wit::ast::Export<'interfaces>,
    ) -> Self {
        Self {
            inner: export_callable,
            inputs: wit_export.input_types.clone(),
            outputs: wit_export.output_types.clone(),
        }
    }
}

impl<'function> Deref for Export<'function> {
    type Target = core::instance::DynFunc<'function>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<'function> wit_wasm::structures::Export for Export<'function> {
    fn inputs_cardinality(&self) -> usize {
        self.inputs.len()
    }

    fn outputs_cardinality(&self) -> usize {
        self.outputs.len()
    }

    fn inputs(&self) -> &[InterfaceType] {
        &self.inputs
    }

    fn outputs(&self) -> &[InterfaceType] {
        &self.outputs
    }

    fn call(&self, arguments: &[InterfaceValue]) -> Result<Vec<InterfaceValue>, ()> {
        self.inner
            .call(
                &arguments
                    .iter()
                    .map(Into::into)
                    .collect::<Vec<core::types::Value>>(),
            )
            .map(|results| results.iter().map(Into::into).collect())
            .map_err(|_| ())
    }
}

pub struct LocalImport {
    #[allow(unused)]
    function_index: core::types::FuncIndex,
    #[allow(unused)]
    module: Arc<core::module::ModuleInner>,
    inputs: Vec<InterfaceType>,
    outputs: Vec<InterfaceType>,
}

impl LocalImport {
    fn new(
        function_index: core::types::FuncIndex,
        module: Arc<core::module::ModuleInner>,
        wit_import: Option<&wit::ast::ImportedFunction>,
    ) -> Result<Self, &'static str> {
        let (inputs, outputs) = match wit_import {
            Some(wit_import) => (
                wit_import.input_types.clone(),
                wit_import.output_types.clone(),
            ),
            None => {
                let signature_index = module
                    .info
                    .func_assoc
                    .get(function_index)
                    .ok_or_else(|| "Invalid function index.")?;
                let signature = core::sig_registry::SigRegistry
                    .lookup_signature_ref(&module.info.signatures[*signature_index]);
                let inputs = signature.params().iter().map(Into::into).collect();
                let outputs = signature.returns().iter().map(Into::into).collect();

                (inputs, outputs)
            }
        };

        Ok(Self {
            function_index,
            module,
            inputs,
            outputs,
        })
    }
}

impl wit_wasm::structures::LocalImport for LocalImport {
    fn inputs_cardinality(&self) -> usize {
        self.inputs.len()
    }

    fn outputs_cardinality(&self) -> usize {
        self.outputs.len()
    }

    fn inputs(&self) -> &[InterfaceType] {
        &self.inputs
    }

    fn outputs(&self) -> &[InterfaceType] {
        &self.outputs
    }

    fn call(&self, _arguments: &[InterfaceValue]) -> Result<Vec<InterfaceValue>, ()> {
        Err(())
    }
}

impl wit_wasm::structures::MemoryView for core::memory::MemoryView<'_, u8> {}

impl<'a> wit_wasm::structures::Memory<core::memory::MemoryView<'a, u8>> for core::memory::Memory {
    fn view(&self) -> core::memory::MemoryView<'a, u8> {
        let core::vm::LocalMemory { base, .. } = unsafe { *self.vm_local_memory() };

        let length = self.size().bytes().0 / mem::size_of::<u8>();

        unsafe { core::memory::MemoryView::new(base as _, length as u32) }
    }
}

pub struct Instance<'instance, 'interfaces> {
    inner: &'instance core::instance::Instance,
    exports: HashMap<String, Export<'instance>>,
    locals_imports: HashMap<usize, LocalImport>,
    memories: Vec<core::memory::Memory>,
    #[allow(unused)]
    interfaces: &'interfaces wit::ast::Interfaces<'interfaces>,
}

impl<'instance, 'interfaces> Instance<'instance, 'interfaces> {
    pub fn new(
        core_instance: &'instance core::instance::Instance,
        interfaces: &'interfaces wit::ast::Interfaces,
    ) -> Result<Self, &'static str> {
        let module_info = &core_instance.module.info;

        let exports = module_info
            .exports
            .iter()
            .filter_map(|(export_name, export_index)| match export_index {
                core::module::ExportIndex::Func(..) => Some((
                    export_name.to_owned(),
                    Export::new(
                        core_instance.dyn_func(export_name).expect(&format!(
                            "Failed to get a dynamic function for `{}`.",
                            export_name
                        )),
                        interfaces
                            .exports
                            .iter()
                            .find(|export| export.name == export_name)?,
                    ),
                )),
                _ => None,
            })
            .collect();

        let imports = module_info
            .imported_functions
            .iter()
            .filter_map(|(imported_index, import_name_indexes)| {
                let namespace = module_info
                    .namespace_table
                    .get(import_name_indexes.namespace_index);
                let name = module_info.name_table.get(import_name_indexes.name_index);

                match interfaces
                    .imported_functions
                    .iter()
                    .find(|imported_function| {
                        imported_function.namespace == namespace && imported_function.name == name
                    }) {
                    Some(interface_imported_function) => Some((
                        imported_index.index(),
                        LocalImport::new(
                            imported_index.convert_up(module_info),
                            core_instance.module.clone(),
                            Some(interface_imported_function),
                        )
                        .ok()?,
                    )),
                    None => None,
                }
            })
            .collect();

        let mut memories = core_instance
            .exports()
            .filter_map(|(_, export)| match export {
                core::export::Export::Memory(memory) => Some(memory.to_owned()),
                _ => None,
            })
            .collect::<Vec<core::memory::Memory>>();

        if let Some(core::export::Export::Memory(memory)) = core_instance
            .import_object
            .maybe_with_namespace("env", |env| env.get_export("memory"))
        {
            memories.push(memory);
        }

        Ok(Self {
            inner: core_instance,
            exports,
            memories,
            locals_imports: imports,
            interfaces,
        })
    }
}

impl Deref for Instance<'_, '_> {
    type Target = core::instance::Instance;

    fn deref(&self) -> &Self::Target {
        self.inner
    }
}

impl<'instance, 'interfaces>
    wit_wasm::structures::Instance<
        Export<'instance>,
        LocalImport,
        core::memory::Memory,
        core::memory::MemoryView<'_, u8>,
    > for Instance<'instance, 'interfaces>
{
    fn export(&self, export_name: &str) -> Option<&Export<'instance>> {
        self.exports.get(export_name)
    }

    fn local_or_import<
        I: wit_wasm::structures::TypedIndex + wit_wasm::structures::LocalImportIndex,
    >(
        &mut self,
        index: I,
    ) -> Option<&LocalImport> {
        let index = index.index();

        if !self.locals_imports.contains_key(&index) {
            let function_index = core::types::FuncIndex::new(index);
            let local_import =
                LocalImport::new(function_index, self.inner.module.clone(), None).ok()?;

            self.locals_imports.insert(index, local_import);
        }

        self.locals_imports.get(&index)
    }

    fn memory(&self, index: usize) -> Option<&core::memory::Memory> {
        if index >= self.memories.len() {
            None
        } else {
            Some(&self.memories[index])
        }
    }
}
