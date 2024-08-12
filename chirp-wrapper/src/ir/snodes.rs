use chirp_sys::taichi::lang;
use autocxx::prelude::*;
use std::ptr::null_mut;

pub struct SNode {
    inner: UniquePtr<lang::SNode>,
}

pub enum SNodeType {
    Root,
    Dense,
    Dynamic,
    Pointer,
    Bitmasked,
    Hash,
    Place,
    BitStruct,
    QuantArray,
    Undefined,
}

impl SNodeType {
    pub(crate) fn to_sys(&self) -> lang::SNodeType {
        match self {
            SNodeType::Root => lang::SNodeType::root,
            SNodeType::Dense => lang::SNodeType::dense,
            SNodeType::Dynamic => lang::SNodeType::dynamic,
            SNodeType::Pointer => lang::SNodeType::pointer,
            SNodeType::Bitmasked => lang::SNodeType::bitmasked,
            SNodeType::Hash => lang::SNodeType::hash,
            SNodeType::Place => lang::SNodeType::place,
            SNodeType::BitStruct => lang::SNodeType::bit_struct,
            SNodeType::QuantArray => lang::SNodeType::quant_array,
            SNodeType::Undefined => lang::SNodeType::undefined,
        }
    }
}

impl SNode {
    pub fn new(depth: i32, ty: SNodeType,) -> Self {
        let inner = unsafe { lang::SNode::new1(c_int(depth), ty.to_sys(), null_mut(), null_mut()).within_unique_ptr() };
        SNode { inner }
    }

    pub fn into_inner(self) -> UniquePtr<lang::SNode> {
        self.inner
    }
}