use super::stmts::*;
use super::Mesh;
use super::SNode;
use crate::types;
use crate::Block;
use crate::CVec;
use autocxx::c_int;
use autocxx::WithinBox;
use chirp_sys::taichi::lang;
use std::pin::Pin;

pub struct IRBuilder {
    inner: Pin<Box<lang::IRBuilder>>,
}

pub struct IfGuard {
    inner: Pin<Box<lang::IRBuilder_IfGuard>>,
}

impl IfGuard {
    pub fn new(inner: Pin<Box<lang::IRBuilder_IfGuard>>) -> Self {
        IfGuard { inner }
    }

    pub fn pin(&mut self) -> Pin<&mut lang::IRBuilder_IfGuard> {
        self.inner.as_mut()
    }
}

impl IRBuilder {
    pub fn new() -> Self {
        let inner = lang::IRBuilder::new().within_box();
        IRBuilder { inner }
    }

    pub fn extract_ir(&mut self) -> Block {
        Block::new(self.pin().extract_ir().into_raw())
    }

    pub fn set_insertion_point_to_after(&mut self, mut stmt: impl Stmt) {
        unsafe { self.pin().set_insertion_point_to_after(stmt.cast()) }
    }

    pub fn set_insertion_point_to_before(&mut self, mut stmt: impl Stmt) {
        unsafe { self.pin().set_insertion_point_to_before(stmt.cast()) }
    }

    pub fn set_insertion_point_to_true_branch(&mut self, if_stmt: &IfStmt) {
        unsafe {
            self.pin()
                .set_insertion_point_to_true_branch(*if_stmt.raw())
        }
    }

    pub fn set_insertion_point_to_false_branch(&mut self, if_stmt: &IfStmt) {
        unsafe {
            self.pin()
                .set_insertion_point_to_false_branch(*if_stmt.raw())
        }
    }

    pub fn get_if_guard(&mut self, if_stmt: &IfStmt, true_branch: bool) -> IfGuard {
        unsafe {
            IfGuard::new(
                self.pin()
                    .get_if_guard(*if_stmt.raw(), true_branch)
                    .within_box(),
            )
        }
    }

    pub fn create_range_for(
        &mut self,
        mut begin: impl Stmt,
        mut end: impl Stmt,
        is_bit_vectorized: bool,
        num_cpu_threads: i32,
        block_dim: i32,
        strictly_serialized: bool,
    ) -> RangeForStmt {
        unsafe {
            RangeForStmt::new(self.pin().create_range_for(
                begin.cast(),
                end.cast(),
                is_bit_vectorized,
                c_int(num_cpu_threads),
                c_int(block_dim),
                strictly_serialized,
            ))
        }
    }

    pub fn create_struct_for(
        &mut self,
        snode: &mut SNode,
        is_bit_vectorized: bool,
        num_cpu_threads: i32,
        block_dim: i32,
    ) -> StructForStmt {
        unsafe {
            StructForStmt::new(self.pin().create_struct_for(
                *snode.raw(),
                is_bit_vectorized,
                c_int(num_cpu_threads),
                c_int(block_dim),
            ))
        }
    }

    pub fn create_mesh_for(
        &mut self,
        mesh: Mesh,
    ) -> MeshForStmt {
        todo!()
    }

    pub fn create_while_true(&mut self) -> WhileStmt {
        WhileStmt::new(self.pin().create_while_true())
    }

    pub fn create_if(&mut self, mut cond: impl Stmt) -> IfStmt {
        IfStmt::new(unsafe { self.pin().create_if(cond.cast()) })
    }

    pub fn create_break(&mut self) -> WhileControlStmt {
        WhileControlStmt::new(self.pin().create_break())
    }

    pub fn create_continue(&mut self) -> ContinueStmt {
        ContinueStmt::new(self.pin().create_continue())
    }

    pub fn create_func_call(&mut self) -> FuncCallStmt {
        // FuncCallStmt::new(self.pin().create_func_call())
        todo!()
    }

    pub fn get_loop_index(&mut self, mut looop: impl Stmt, index: i32) -> LoopIndexStmt {
        LoopIndexStmt::new(unsafe { self.pin().get_loop_index(looop.cast(), c_int(index)) })
    }

    pub fn get_int32(&mut self, value: i32) -> ConstStmt {
        ConstStmt::new(self.pin().get_int32(value))
    }

    pub fn get_int64(&mut self, value: i64) -> ConstStmt {
        ConstStmt::new(self.pin().get_int64(value))
    }

    pub fn get_uint32(&mut self, value: u32) -> ConstStmt {
        ConstStmt::new(self.pin().get_uint32(value))
    }

    pub fn get_uint64(&mut self, value: u64) -> ConstStmt {
        ConstStmt::new(self.pin().get_uint64(value))
    }

    pub fn get_float32(&mut self, value: f32) -> ConstStmt {
        ConstStmt::new(self.pin().get_float32(value))
    }

    pub fn get_float64(&mut self, value: f64) -> ConstStmt {
        ConstStmt::new(self.pin().get_float64(value))
    }

    pub fn create_local_var(&mut self, dt: types::DataType) -> AllocaStmt {
        AllocaStmt::new(self.pin().create_local_var(dt.uni_ptr()))
    }

    pub fn create_local_store(&mut self, ptr: &AllocaStmt, mut data: impl Stmt) {
        unsafe {
            self.pin().create_local_store(*ptr.raw(), data.cast());
        }
    }

    pub fn create_local_load(&mut self, ptr: &AllocaStmt) -> LocalLoadStmt {
        unsafe { LocalLoadStmt::new(self.pin().create_local_load(*ptr.raw())) }
    }

    pub fn create_return(&mut self, mut value: impl Stmt) -> ReturnStmt {
        unsafe { ReturnStmt::new(self.pin().create_return(value.cast())) }
    }

    pub fn create_cast(
        &mut self,
        mut value: impl Stmt,
        output_type: types::DataType,
    ) -> UnaryOpStmt {
        unsafe { UnaryOpStmt::new(self.pin().create_cast(value.cast(), output_type.uni_ptr())) }
    }

    pub fn create_bit_cast(
        &mut self,
        mut value: impl Stmt,
        output_type: types::DataType,
    ) -> UnaryOpStmt {
        unsafe {
            UnaryOpStmt::new(
                self.pin()
                    .create_bit_cast(value.cast(), output_type.uni_ptr()),
            )
        }
    }

    pub fn create_neg(&mut self, mut value: impl Stmt) -> UnaryOpStmt {
        unsafe { UnaryOpStmt::new(self.pin().create_neg(value.cast())) }
    }

    pub fn create_not(&mut self, mut value: impl Stmt) -> UnaryOpStmt {
        unsafe { UnaryOpStmt::new(self.pin().create_not(value.cast())) }
    }

    pub fn create_logical_not(&mut self, mut value: impl Stmt) -> UnaryOpStmt {
        unsafe { UnaryOpStmt::new(self.pin().create_logical_not(value.cast())) }
    }

    pub fn create_round(&mut self, mut value: impl Stmt) -> UnaryOpStmt {
        unsafe { UnaryOpStmt::new(self.pin().create_round(value.cast())) }
    }

    pub fn create_floor(&mut self, mut value: impl Stmt) -> UnaryOpStmt {
        unsafe { UnaryOpStmt::new(self.pin().create_floor(value.cast())) }
    }

    pub fn create_ceil(&mut self, mut value: impl Stmt) -> UnaryOpStmt {
        unsafe { UnaryOpStmt::new(self.pin().create_ceil(value.cast())) }
    }

    pub fn create_abs(&mut self, mut value: impl Stmt) -> UnaryOpStmt {
        unsafe { UnaryOpStmt::new(self.pin().create_abs(value.cast())) }
    }

    pub fn create_sgn(&mut self, mut value: impl Stmt) -> UnaryOpStmt {
        unsafe { UnaryOpStmt::new(self.pin().create_sgn(value.cast())) }
    }

    pub fn create_sqrt(&mut self, mut value: impl Stmt) -> UnaryOpStmt {
        unsafe { UnaryOpStmt::new(self.pin().create_sqrt(value.cast())) }
    }

    pub fn create_rsqrt(&mut self, mut value: impl Stmt) -> UnaryOpStmt {
        unsafe { UnaryOpStmt::new(self.pin().create_rsqrt(value.cast())) }
    }

    pub fn create_sin(&mut self, mut value: impl Stmt) -> UnaryOpStmt {
        unsafe { UnaryOpStmt::new(self.pin().create_sin(value.cast())) }
    }

    pub fn create_asin(&mut self, mut value: impl Stmt) -> UnaryOpStmt {
        unsafe { UnaryOpStmt::new(self.pin().create_asin(value.cast())) }
    }

    pub fn create_cos(&mut self, mut value: impl Stmt) -> UnaryOpStmt {
        unsafe { UnaryOpStmt::new(self.pin().create_cos(value.cast())) }
    }

    pub fn create_acos(&mut self, mut value: impl Stmt) -> UnaryOpStmt {
        unsafe { UnaryOpStmt::new(self.pin().create_acos(value.cast())) }
    }

    pub fn create_tan(&mut self, mut value: impl Stmt) -> UnaryOpStmt {
        unsafe { UnaryOpStmt::new(self.pin().create_tan(value.cast())) }
    }

    pub fn create_tanh(&mut self, mut value: impl Stmt) -> UnaryOpStmt {
        unsafe { UnaryOpStmt::new(self.pin().create_tanh(value.cast())) }
    }

    pub fn create_exp(&mut self, mut value: impl Stmt) -> UnaryOpStmt {
        unsafe { UnaryOpStmt::new(self.pin().create_exp(value.cast())) }
    }

    pub fn create_log(&mut self, mut value: impl Stmt) -> UnaryOpStmt {
        unsafe { UnaryOpStmt::new(self.pin().create_log(value.cast())) }
    }

    pub fn create_popcnt(&mut self, mut value: impl Stmt) -> UnaryOpStmt {
        unsafe { UnaryOpStmt::new(self.pin().create_popcnt(value.cast())) }
    }

    pub fn create_clz(&mut self, mut value: impl Stmt) -> UnaryOpStmt {
        unsafe { UnaryOpStmt::new(self.pin().create_clz(value.cast())) }
    }

    pub fn create_add(&mut self, mut l: impl Stmt, mut r: impl Stmt) -> BinaryOpStmt {
        unsafe { BinaryOpStmt::new(self.pin().create_add(l.cast(), r.cast())) }
    }

    pub fn create_sub(&mut self, mut l: impl Stmt, mut r: impl Stmt) -> BinaryOpStmt {
        unsafe { BinaryOpStmt::new(self.pin().create_sub(l.cast(), r.cast())) }
    }

    pub fn create_mul(&mut self, mut l: impl Stmt, mut r: impl Stmt) -> BinaryOpStmt {
        unsafe { BinaryOpStmt::new(self.pin().create_mul(l.cast(), r.cast())) }
    }

    pub fn create_div(&mut self, mut l: impl Stmt, mut r: impl Stmt) -> BinaryOpStmt {
        unsafe { BinaryOpStmt::new(self.pin().create_div(l.cast(), r.cast())) }
    }

    pub fn create_floordiv(&mut self, mut l: impl Stmt, mut r: impl Stmt) -> BinaryOpStmt {
        unsafe { BinaryOpStmt::new(self.pin().create_floordiv(l.cast(), r.cast())) }
    }

    pub fn create_truediv(&mut self, mut l: impl Stmt, mut r: impl Stmt) -> BinaryOpStmt {
        unsafe { BinaryOpStmt::new(self.pin().create_truediv(l.cast(), r.cast())) }
    }

    pub fn create_mod(&mut self, mut l: impl Stmt, mut r: impl Stmt) -> BinaryOpStmt {
        unsafe { BinaryOpStmt::new(self.pin().create_mod(l.cast(), r.cast())) }
    }

    pub fn create_max(&mut self, mut l: impl Stmt, mut r: impl Stmt) -> BinaryOpStmt {
        unsafe { BinaryOpStmt::new(self.pin().create_max(l.cast(), r.cast())) }
    }

    pub fn create_min(&mut self, mut l: impl Stmt, mut r: impl Stmt) -> BinaryOpStmt {
        unsafe { BinaryOpStmt::new(self.pin().create_min(l.cast(), r.cast())) }
    }

    pub fn create_atan2(&mut self, mut l: impl Stmt, mut r: impl Stmt) -> BinaryOpStmt {
        unsafe { BinaryOpStmt::new(self.pin().create_atan2(l.cast(), r.cast())) }
    }

    pub fn create_pow(&mut self, mut l: impl Stmt, mut r: impl Stmt) -> BinaryOpStmt {
        unsafe { BinaryOpStmt::new(self.pin().create_pow(l.cast(), r.cast())) }
    }

    pub fn create_and(&mut self, mut l: impl Stmt, mut r: impl Stmt) -> BinaryOpStmt {
        unsafe { BinaryOpStmt::new(self.pin().create_and(l.cast(), r.cast())) }
    }

    pub fn create_or(&mut self, mut l: impl Stmt, mut r: impl Stmt) -> BinaryOpStmt {
        unsafe { BinaryOpStmt::new(self.pin().create_or(l.cast(), r.cast())) }
    }

    pub fn create_xor(&mut self, mut l: impl Stmt, mut r: impl Stmt) -> BinaryOpStmt {
        unsafe { BinaryOpStmt::new(self.pin().create_xor(l.cast(), r.cast())) }
    }

    pub fn create_shl(&mut self, mut l: impl Stmt, mut r: impl Stmt) -> BinaryOpStmt {
        unsafe { BinaryOpStmt::new(self.pin().create_shl(l.cast(), r.cast())) }
    }

    pub fn create_shr(&mut self, mut l: impl Stmt, mut r: impl Stmt) -> BinaryOpStmt {
        unsafe { BinaryOpStmt::new(self.pin().create_shr(l.cast(), r.cast())) }
    }

    pub fn create_sar(&mut self, mut l: impl Stmt, mut r: impl Stmt) -> BinaryOpStmt {
        unsafe { BinaryOpStmt::new(self.pin().create_sar(l.cast(), r.cast())) }
    }

    pub fn create_cmp_lt(&mut self, mut l: impl Stmt, mut r: impl Stmt) -> BinaryOpStmt {
        unsafe { BinaryOpStmt::new(self.pin().create_cmp_lt(l.cast(), r.cast())) }
    }

    pub fn create_cmp_le(&mut self, mut l: impl Stmt, mut r: impl Stmt) -> BinaryOpStmt {
        unsafe { BinaryOpStmt::new(self.pin().create_cmp_le(l.cast(), r.cast())) }
    }

    pub fn create_cmp_gt(&mut self, mut l: impl Stmt, mut r: impl Stmt) -> BinaryOpStmt {
        unsafe { BinaryOpStmt::new(self.pin().create_cmp_gt(l.cast(), r.cast())) }
    }

    pub fn create_cmp_ge(&mut self, mut l: impl Stmt, mut r: impl Stmt) -> BinaryOpStmt {
        unsafe { BinaryOpStmt::new(self.pin().create_cmp_ge(l.cast(), r.cast())) }
    }

    pub fn create_cmp_eq(&mut self, mut l: impl Stmt, mut r: impl Stmt) -> BinaryOpStmt {
        unsafe { BinaryOpStmt::new(self.pin().create_cmp_eq(l.cast(), r.cast())) }
    }

    pub fn create_cmp_ne(&mut self, mut l: impl Stmt, mut r: impl Stmt) -> BinaryOpStmt {
        unsafe { BinaryOpStmt::new(self.pin().create_cmp_ne(l.cast(), r.cast())) }
    }

    pub fn create_logical_or(&mut self, mut l: impl Stmt, mut r: impl Stmt) -> BinaryOpStmt {
        unsafe { BinaryOpStmt::new(self.pin().create_logical_or(l.cast(), r.cast())) }
    }

    pub fn create_logical_and(&mut self, mut l: impl Stmt, mut r: impl Stmt) -> BinaryOpStmt {
        unsafe { BinaryOpStmt::new(self.pin().create_logical_and(l.cast(), r.cast())) }
    }

    pub fn create_atomic_add(&mut self, mut dest: impl Stmt, mut val: impl Stmt) -> AtomicOpStmt {
        unsafe { AtomicOpStmt::new(self.pin().create_atomic_add(dest.cast(), val.cast())) }
    }

    pub fn create_atomic_sub(&mut self, mut dest: impl Stmt, mut val: impl Stmt) -> AtomicOpStmt {
        unsafe { AtomicOpStmt::new(self.pin().create_atomic_sub(dest.cast(), val.cast())) }
    }

    pub fn create_atomic_max(&mut self, mut dest: impl Stmt, mut val: impl Stmt) -> AtomicOpStmt {
        unsafe { AtomicOpStmt::new(self.pin().create_atomic_max(dest.cast(), val.cast())) }
    }

    pub fn create_atomic_min(&mut self, mut dest: impl Stmt, mut val: impl Stmt) -> AtomicOpStmt {
        unsafe { AtomicOpStmt::new(self.pin().create_atomic_min(dest.cast(), val.cast())) }
    }

    pub fn create_atomic_and(&mut self, mut dest: impl Stmt, mut val: impl Stmt) -> AtomicOpStmt {
        unsafe { AtomicOpStmt::new(self.pin().create_atomic_and(dest.cast(), val.cast())) }
    }

    pub fn create_atomic_or(&mut self, mut dest: impl Stmt, mut val: impl Stmt) -> AtomicOpStmt {
        unsafe { AtomicOpStmt::new(self.pin().create_atomic_or(dest.cast(), val.cast())) }
    }

    pub fn create_atomic_xor(&mut self, mut dest: impl Stmt, mut val: impl Stmt) -> AtomicOpStmt {
        unsafe { AtomicOpStmt::new(self.pin().create_atomic_xor(dest.cast(), val.cast())) }
    }

    pub fn create_atomic_mul(&mut self, mut dest: impl Stmt, mut val: impl Stmt) -> AtomicOpStmt {
        unsafe { AtomicOpStmt::new(self.pin().create_atomic_mul(dest.cast(), val.cast())) }
    }

    pub fn create_select(
        &mut self,
        mut cond: impl Stmt,
        mut true_result: impl Stmt,
        mut false_result: impl Stmt,
    ) -> TernaryOpStmt {
        unsafe {
            TernaryOpStmt::new(self.pin().create_select(
                cond.cast(),
                true_result.cast(),
                false_result.cast(),
            ))
        }
    }

    pub fn create_global_ptr(
        &mut self,
        snode: &mut SNode
    ) -> GlobalPtrStmt {
        todo!()
    }

    pub fn create_ad_stack(
        &mut self,
        dt: types::DataType,
        max_size: usize
    ) -> AdStackAllocaStmt {
        
        AdStackAllocaStmt::new(
                self.pin().create_ad_stack(&dt.uni_ptr(), max_size)
            )
        
    }

    pub fn ad_stack_push(
        &mut self,
        stack: &AdStackAllocaStmt,
        mut val: impl Stmt
    ) {
        unsafe { self.pin().ad_stack_push(*stack.raw(), val.cast()) }
    }

    pub fn ad_stack_pop(&mut self, stack: &AdStackAllocaStmt) {
        unsafe { self.pin().ad_stack_pop(*stack.raw())}
    }

    pub fn ad_stack_load_top(&mut self, stack: &AdStackAllocaStmt) -> AdStackLoadTopStmt {
        unsafe {
            AdStackLoadTopStmt::new(self.pin().ad_stack_load_top(*stack.raw()))
        }
    }
    
    pub fn ad_stack_load_top_adjoint(&mut self, stack: &AdStackAllocaStmt) -> AdStackLoadTopAdjStmt {
        unsafe {
            AdStackLoadTopAdjStmt::new(
                self.pin().ad_stack_load_top_adjoint(*stack.raw())
            )
        }
    }

    pub fn ad_stack_accumulate_adjoint(&mut self, stack: &AdStackAllocaStmt, mut val: impl Stmt) {
        unsafe {
            self.pin().ad_stack_accumulate_adjoint(*stack.raw(), val.cast())
        }
    }

    // pub fn get_relation_size(&mut self, mesh: &Mesh, mut mesh_idx: impl Stmt, to_type: MeshElementType) -> MeshRelationAccessStmt {
    //     todo!()
    // }

    // pub fn get_relation_size(&mut self, mesh: &Mesh, mut mesh_idx: impl Stmt, to_type: MeshElementType, mut neighbor_idx: impl Stmt) -> MeshRelationAccessStmt {
    //     todo!()
    // }

    pub fn get_patch_index(&mut self) -> MeshPatchIndexStmt {
        
            MeshPatchIndexStmt::new(self.pin().get_patch_index())
        
    }

    pub fn pin(&mut self) -> Pin<&mut lang::IRBuilder> {
        self.inner.as_mut()
    }
}
