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

#[allow(dead_code)]
pub(crate) struct Export<'function> {
    inner: core::instance::DynFunc<'function>,
    inputs: Vec<InterfaceType>,
    outputs: Vec<InterfaceType>,
}

impl<'function> From<core::instance::DynFunc<'function>> for Export<'function> {
    fn from(export: core::instance::DynFunc<'function>) -> Self {
        let inputs = export.signature.params().iter().map(Into::into).collect();
        let outputs = export.signature.returns().iter().map(Into::into).collect();

        Self {
            inner: export,
            inputs,
            outputs,
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
    exports: HashMap<String, Export<'a>>,
}

impl<'a> From<&'a core::instance::Instance> for Instance<'a> {
    fn from(instance: &'a core::instance::Instance) -> Self {
        Self {
            inner: instance,
            exports: instance
                .module
                .info
                .exports
                .iter()
                .filter_map(|(export_name, export_index)| match export_index {
                    core::module::ExportIndex::Func(..) => Some((
                        export_name.to_owned(),
                        instance
                            .dyn_func(export_name)
                            .expect(&format!(
                                "Failed to get a dynamic function for `{}`.",
                                export_name
                            ))
                            .into(),
                    )),
                    _ => None,
                })
                .collect(),
        }
    }
}

impl Deref for Instance<'_> {
    type Target = core::instance::Instance;

    fn deref(&self) -> &Self::Target {
        self.inner
    }
}

impl<'instance>
    wit_wasm::structures::Instance<
        Export<'instance>,
        (),
        core::memory::Memory,
        core::memory::MemoryView<'_, u8>,
    > for Instance<'instance>
{
    fn export(&self, export_name: &str) -> Option<&Export<'instance>> {
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
