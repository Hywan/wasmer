use crate as core;
use std::mem;
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

impl wit_wasm::structures::Export for core::export::Export {
    fn inputs_cardinality(&self) -> usize {
        match self {
            core::export::Export::Function { signature, .. } => signature.params().len(),
            _ => 0,
        }
    }

    fn outputs_cardinality(&self) -> usize {
        match self {
            core::export::Export::Function { signature, .. } => signature.returns().len(),
            _ => 0,
        }
    }

    fn inputs(&self) -> &[InterfaceType] {
        &[]
    }

    fn outputs(&self) -> &[InterfaceType] {
        &[]
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
