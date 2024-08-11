use autocxx::prelude::*;
use chirp_sys::taichi::lang;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrimTy {
    F16,
    F32,
    F64,
    I8,
    I16,
    I32,
    I64,
    U1,
    U8,
    U16,
    U32,
    U64,
    Gen,
    Unknown,
}

pub struct DataType {
    inner: UniquePtr<lang::DataType>,
}

impl From<PrimTy> for DataType {
    fn from(prim_ty: PrimTy) -> Self {
        let primitive_type_id = match prim_ty {
            PrimTy::F16 => lang::PrimitiveTypeID::f16,
            PrimTy::F32 => lang::PrimitiveTypeID::f32_,
            PrimTy::F64 => lang::PrimitiveTypeID::f64_,
            PrimTy::I8 => lang::PrimitiveTypeID::i8_,
            PrimTy::I16 => lang::PrimitiveTypeID::i16_,
            PrimTy::I32 => lang::PrimitiveTypeID::i32_,
            PrimTy::I64 => lang::PrimitiveTypeID::i64_,
            PrimTy::U1 => lang::PrimitiveTypeID::u1,
            PrimTy::U8 => lang::PrimitiveTypeID::u8_,
            PrimTy::U16 => lang::PrimitiveTypeID::u16_,
            PrimTy::U32 => lang::PrimitiveTypeID::u32_,
            PrimTy::U64 => lang::PrimitiveTypeID::u64_,
            PrimTy::Gen => lang::PrimitiveTypeID::gen,
            PrimTy::Unknown => lang::PrimitiveTypeID::unknown,
        };

        DataType {
            inner: lang::PrimitiveType::get(primitive_type_id).within_unique_ptr(),
        }
    }
}

impl DataType {
    pub fn as_value_param(&self) -> impl ValueParam<lang::DataType> + '_ {
        &*self.inner
    }
}
