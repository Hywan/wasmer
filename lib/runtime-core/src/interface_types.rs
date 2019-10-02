use crate as core;
use std::{collections::HashMap, mem, ops::Deref};
use wasmer_interface_types::interpreter::wasm::{
    self as wit_wasm,
    values::{InterfaceType, InterfaceValue},
};

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

#[allow(dead_code)]
pub(crate) struct Export {
    inner: core::export::Export,
    inputs: Vec<InterfaceType>,
    outputs: Vec<InterfaceType>,
}

impl From<core::export::Export> for Export {
    fn from(export: core::export::Export) -> Self {
        let inputs = match &export {
            core::export::Export::Function { signature, .. } => {
                signature.params().iter().map(Into::into).collect()
            }
            _ => vec![],
        };
        let outputs = match &export {
            core::export::Export::Function { signature, .. } => {
                signature.returns().iter().map(Into::into).collect()
            }
            _ => vec![],
        };

        Self {
            inner: export,
            inputs,
            outputs,
        }
    }
}

impl Deref for Export {
    type Target = core::export::Export;

    fn deref(&self) -> &core::export::Export {
        &self.inner
    }
}

impl wit_wasm::structures::Export for Export {
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

#[allow(unused)]
pub(crate) struct Instance<'a> {
    inner: &'a core::instance::Instance,
    exports: HashMap<String, Export>,
}

impl<'a> From<&'a core::instance::Instance> for Instance<'a> {
    fn from(instance: &'a core::instance::Instance) -> Self {
        Self {
            inner: instance,
            exports: instance
                .exports()
                .filter_map(|(export_name, export)| match export {
                    core::export::Export::Function { .. } => {
                        Some((export_name, export.clone().into()))
                    }
                    _ => None,
                })
                .collect(),
        }
    }
}

impl Deref for Instance<'_> {
    type Target = core::instance::Instance;

    fn deref(&self) -> &core::instance::Instance {
        self.inner
    }
}

impl<'instance>
    wit_wasm::structures::Instance<
        Export,
        (),
        core::memory::Memory,
        core::memory::MemoryView<'_, u8>,
    > for Instance<'instance>
{
    fn export(&self, export_name: &str) -> Option<&Export> {
        self.exports.get(export_name)
    }

    fn local_or_import<
        I: wit_wasm::structures::TypedIndex + wit_wasm::structures::LocalImportIndex,
    >(
        &self,
        _index: I,
    ) -> Option<&()> {
        None
    }

    fn memory(&self, _index: usize) -> Option<&core::memory::Memory> {
        None
    }
}
