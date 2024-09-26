use chirp_sys::taichi::lang;

pub trait Stmt {
    fn cast(&mut self) -> *mut lang::Stmt;
}

pub struct AllocaStmt {
    inner: *mut lang::AllocaStmt,
}

impl AllocaStmt {
    pub fn new(inner: *mut lang::AllocaStmt) -> Self {
        AllocaStmt { inner }
    }
    pub fn raw(&self) -> &*mut lang::AllocaStmt {
        &self.inner
    }
}

impl Stmt for AllocaStmt {
    fn cast(&mut self) -> *mut lang::Stmt {
        self.inner.cast()
    }
}

pub struct WhileControlStmt {
    inner: *mut lang::WhileControlStmt,
}

impl WhileControlStmt {
    pub fn new(inner: *mut lang::WhileControlStmt) -> Self {
        WhileControlStmt { inner }
    }
    pub fn raw(&self) -> &*mut lang::WhileControlStmt {
        &self.inner
    }
}

impl Stmt for WhileControlStmt {
    fn cast(&mut self) -> *mut lang::Stmt {
        self.inner.cast()
    }
}

pub struct ContinueStmt {
    inner: *mut lang::ContinueStmt,
}

impl ContinueStmt {
    pub fn new(inner: *mut lang::ContinueStmt) -> Self {
        ContinueStmt { inner }
    }
    pub fn raw(&self) -> &*mut lang::ContinueStmt {
        &self.inner
    }
}

impl Stmt for ContinueStmt {
    fn cast(&mut self) -> *mut lang::Stmt {
        self.inner.cast()
    }
}

pub struct DecorationStmt {
    inner: *mut lang::DecorationStmt,
}

impl DecorationStmt {
    pub fn new(inner: *mut lang::DecorationStmt) -> Self {
        DecorationStmt { inner }
    }
    pub fn raw(&self) -> &*mut lang::DecorationStmt {
        &self.inner
    }
}

impl Stmt for DecorationStmt {
    fn cast(&mut self) -> *mut lang::Stmt {
        self.inner.cast()
    }
}

pub struct UnaryOpStmt {
    inner: *mut lang::UnaryOpStmt,
}

impl UnaryOpStmt {
    pub fn new(inner: *mut lang::UnaryOpStmt) -> Self {
        UnaryOpStmt { inner }
    }
    pub fn raw(&self) -> &*mut lang::UnaryOpStmt {
        &self.inner
    }
}

impl Stmt for UnaryOpStmt {
    fn cast(&mut self) -> *mut lang::Stmt {
        self.inner.cast()
    }
}

pub struct ArgLoadStmt {
    inner: *mut lang::ArgLoadStmt,
}

impl ArgLoadStmt {
    pub fn new(inner: *mut lang::ArgLoadStmt) -> Self {
        ArgLoadStmt { inner }
    }
    pub fn raw(&self) -> &*mut lang::ArgLoadStmt {
        &self.inner
    }
}

impl Stmt for ArgLoadStmt {
    fn cast(&mut self) -> *mut lang::Stmt {
        self.inner.cast()
    }
}

pub struct RandStmt {
    inner: *mut lang::RandStmt,
}

impl RandStmt {
    pub fn new(inner: *mut lang::RandStmt) -> Self {
        RandStmt { inner }
    }
    pub fn raw(&self) -> &*mut lang::RandStmt {
        &self.inner
    }
}

impl Stmt for RandStmt {
    fn cast(&mut self) -> *mut lang::Stmt {
        self.inner.cast()
    }
}

pub struct BinaryOpStmt {
    inner: *mut lang::BinaryOpStmt,
}

impl BinaryOpStmt {
    pub fn new(inner: *mut lang::BinaryOpStmt) -> Self {
        BinaryOpStmt { inner }
    }
    pub fn raw(&self) -> &*mut lang::BinaryOpStmt {
        &self.inner
    }
}

impl Stmt for BinaryOpStmt {
    fn cast(&mut self) -> *mut lang::Stmt {
        self.inner.cast()
    }
}

pub struct TernaryOpStmt {
    inner: *mut lang::TernaryOpStmt,
}

impl TernaryOpStmt {
    pub fn new(inner: *mut lang::TernaryOpStmt) -> Self {
        TernaryOpStmt { inner }
    }
    pub fn raw(&self) -> &*mut lang::TernaryOpStmt {
        &self.inner
    }
}

impl Stmt for TernaryOpStmt {
    fn cast(&mut self) -> *mut lang::Stmt {
        self.inner.cast()
    }
}

pub struct AtomicOpStmt {
    inner: *mut lang::AtomicOpStmt,
}

impl AtomicOpStmt {
    pub fn new(inner: *mut lang::AtomicOpStmt) -> Self {
        AtomicOpStmt { inner }
    }
    pub fn raw(&self) -> &*mut lang::AtomicOpStmt {
        &self.inner
    }
}

impl Stmt for AtomicOpStmt {
    fn cast(&mut self) -> *mut lang::Stmt {
        self.inner.cast()
    }
}

pub struct ExternalPtrStmt {
    inner: *mut lang::ExternalPtrStmt,
}

impl ExternalPtrStmt {
    pub fn new(inner: *mut lang::ExternalPtrStmt) -> Self {
        ExternalPtrStmt { inner }
    }
    pub fn raw(&self) -> &*mut lang::ExternalPtrStmt {
        &self.inner
    }
}

impl Stmt for ExternalPtrStmt {
    fn cast(&mut self) -> *mut lang::Stmt {
        self.inner.cast()
    }
}

pub struct GlobalPtrStmt {
    inner: *mut lang::GlobalPtrStmt,
}

impl GlobalPtrStmt {
    pub fn new(inner: *mut lang::GlobalPtrStmt) -> Self {
        GlobalPtrStmt { inner }
    }
    pub fn raw(&self) -> &*mut lang::GlobalPtrStmt {
        &self.inner
    }
}

impl Stmt for GlobalPtrStmt {
    fn cast(&mut self) -> *mut lang::Stmt {
        self.inner.cast()
    }
}

pub struct MatrixOfGlobalPtrStmt {
    inner: *mut lang::MatrixOfGlobalPtrStmt,
}

impl MatrixOfGlobalPtrStmt {
    pub fn new(inner: *mut lang::MatrixOfGlobalPtrStmt) -> Self {
        MatrixOfGlobalPtrStmt { inner }
    }
    pub fn raw(&self) -> &*mut lang::MatrixOfGlobalPtrStmt {
        &self.inner
    }
}

impl Stmt for MatrixOfGlobalPtrStmt {
    fn cast(&mut self) -> *mut lang::Stmt {
        self.inner.cast()
    }
}

pub struct MatrixOfMatrixPtrStmt {
    inner: *mut lang::MatrixOfMatrixPtrStmt,
}

impl MatrixOfMatrixPtrStmt {
    pub fn new(inner: *mut lang::MatrixOfMatrixPtrStmt) -> Self {
        MatrixOfMatrixPtrStmt { inner }
    }
    pub fn raw(&self) -> &*mut lang::MatrixOfMatrixPtrStmt {
        &self.inner
    }
}

impl Stmt for MatrixOfMatrixPtrStmt {
    fn cast(&mut self) -> *mut lang::Stmt {
        self.inner.cast()
    }
}

pub struct MatrixPtrStmt {
    inner: *mut lang::MatrixPtrStmt,
}

impl MatrixPtrStmt {
    pub fn new(inner: *mut lang::MatrixPtrStmt) -> Self {
        MatrixPtrStmt { inner }
    }
    pub fn raw(&self) -> &*mut lang::MatrixPtrStmt {
        &self.inner
    }
}

impl Stmt for MatrixPtrStmt {
    fn cast(&mut self) -> *mut lang::Stmt {
        self.inner.cast()
    }
}

pub struct SNodeOpStmt {
    inner: *mut lang::SNodeOpStmt,
}

impl SNodeOpStmt {
    pub fn new(inner: *mut lang::SNodeOpStmt) -> Self {
        SNodeOpStmt { inner }
    }
    pub fn raw(&self) -> &*mut lang::SNodeOpStmt {
        &self.inner
    }
}

impl Stmt for SNodeOpStmt {
    fn cast(&mut self) -> *mut lang::Stmt {
        self.inner.cast()
    }
}

pub struct ExternalTensorShapeAlongAxisStmt {
    inner: *mut lang::ExternalTensorShapeAlongAxisStmt,
}

impl ExternalTensorShapeAlongAxisStmt {
    pub fn new(inner: *mut lang::ExternalTensorShapeAlongAxisStmt) -> Self {
        ExternalTensorShapeAlongAxisStmt { inner }
    }
    pub fn raw(&self) -> &*mut lang::ExternalTensorShapeAlongAxisStmt {
        &self.inner
    }
}

impl Stmt for ExternalTensorShapeAlongAxisStmt {
    fn cast(&mut self) -> *mut lang::Stmt {
        self.inner.cast()
    }
}

pub struct ExternalTensorBasePtrStmt {
    inner: *mut lang::ExternalTensorBasePtrStmt,
}

impl ExternalTensorBasePtrStmt {
    pub fn new(inner: *mut lang::ExternalTensorBasePtrStmt) -> Self {
        ExternalTensorBasePtrStmt { inner }
    }
    pub fn raw(&self) -> &*mut lang::ExternalTensorBasePtrStmt {
        &self.inner
    }
}

impl Stmt for ExternalTensorBasePtrStmt {
    fn cast(&mut self) -> *mut lang::Stmt {
        self.inner.cast()
    }
}

pub struct AssertStmt {
    inner: *mut lang::AssertStmt,
}

impl AssertStmt {
    pub fn new(inner: *mut lang::AssertStmt) -> Self {
        AssertStmt { inner }
    }
    pub fn raw(&self) -> &*mut lang::AssertStmt {
        &self.inner
    }
}

impl Stmt for AssertStmt {
    fn cast(&mut self) -> *mut lang::Stmt {
        self.inner.cast()
    }
}

pub struct ExternalFuncCallStmt {
    inner: *mut lang::ExternalFuncCallStmt,
}

impl ExternalFuncCallStmt {
    pub fn new(inner: *mut lang::ExternalFuncCallStmt) -> Self {
        ExternalFuncCallStmt { inner }
    }
    pub fn raw(&self) -> &*mut lang::ExternalFuncCallStmt {
        &self.inner
    }
}

impl Stmt for ExternalFuncCallStmt {
    fn cast(&mut self) -> *mut lang::Stmt {
        self.inner.cast()
    }
}

pub struct RangeAssumptionStmt {
    inner: *mut lang::RangeAssumptionStmt,
}

impl RangeAssumptionStmt {
    pub fn new(inner: *mut lang::RangeAssumptionStmt) -> Self {
        RangeAssumptionStmt { inner }
    }
    pub fn raw(&self) -> &*mut lang::RangeAssumptionStmt {
        &self.inner
    }
}

impl Stmt for RangeAssumptionStmt {
    fn cast(&mut self) -> *mut lang::Stmt {
        self.inner.cast()
    }
}

pub struct LoopUniqueStmt {
    inner: *mut lang::LoopUniqueStmt,
}

impl LoopUniqueStmt {
    pub fn new(inner: *mut lang::LoopUniqueStmt) -> Self {
        LoopUniqueStmt { inner }
    }
    pub fn raw(&self) -> &*mut lang::LoopUniqueStmt {
        &self.inner
    }
}

impl Stmt for LoopUniqueStmt {
    fn cast(&mut self) -> *mut lang::Stmt {
        self.inner.cast()
    }
}

pub struct GlobalLoadStmt {
    inner: *mut lang::GlobalLoadStmt,
}

impl GlobalLoadStmt {
    pub fn new(inner: *mut lang::GlobalLoadStmt) -> Self {
        GlobalLoadStmt { inner }
    }
    pub fn raw(&self) -> &*mut lang::GlobalLoadStmt {
        &self.inner
    }
}

impl Stmt for GlobalLoadStmt {
    fn cast(&mut self) -> *mut lang::Stmt {
        self.inner.cast()
    }
}

pub struct GlobalStoreStmt {
    inner: *mut lang::GlobalStoreStmt,
}

impl GlobalStoreStmt {
    pub fn new(inner: *mut lang::GlobalStoreStmt) -> Self {
        GlobalStoreStmt { inner }
    }
    pub fn raw(&self) -> &*mut lang::GlobalStoreStmt {
        &self.inner
    }
}

impl Stmt for GlobalStoreStmt {
    fn cast(&mut self) -> *mut lang::Stmt {
        self.inner.cast()
    }
}

pub struct LocalLoadStmt {
    inner: *mut lang::LocalLoadStmt,
}

impl LocalLoadStmt {
    pub fn new(inner: *mut lang::LocalLoadStmt) -> Self {
        LocalLoadStmt { inner }
    }
    pub fn raw(&self) -> &*mut lang::LocalLoadStmt {
        &self.inner
    }
}

impl Stmt for LocalLoadStmt {
    fn cast(&mut self) -> *mut lang::Stmt {
        self.inner.cast()
    }
}

pub struct LocalStoreStmt {
    inner: *mut lang::LocalStoreStmt,
}

impl LocalStoreStmt {
    pub fn new(inner: *mut lang::LocalStoreStmt) -> Self {
        LocalStoreStmt { inner }
    }
    pub fn raw(&self) -> &*mut lang::LocalStoreStmt {
        &self.inner
    }
}

impl Stmt for LocalStoreStmt {
    fn cast(&mut self) -> *mut lang::Stmt {
        self.inner.cast()
    }
}

pub struct IfStmt {
    inner: *mut lang::IfStmt,
}

impl IfStmt {
    pub fn new(inner: *mut lang::IfStmt) -> Self {
        IfStmt { inner }
    }
    pub fn raw(&self) -> &*mut lang::IfStmt {
        &self.inner
    }
}

impl Stmt for IfStmt {
    fn cast(&mut self) -> *mut lang::Stmt {
        self.inner.cast()
    }
}

pub struct PrintStmt {
    inner: *mut lang::PrintStmt,
}

impl PrintStmt {
    pub fn new(inner: *mut lang::PrintStmt) -> Self {
        PrintStmt { inner }
    }
    pub fn raw(&self) -> &*mut lang::PrintStmt {
        &self.inner
    }
}

impl Stmt for PrintStmt {
    fn cast(&mut self) -> *mut lang::Stmt {
        self.inner.cast()
    }
}

pub struct ConstStmt {
    inner: *mut lang::ConstStmt,
}

impl ConstStmt {
    pub fn new(inner: *mut lang::ConstStmt) -> Self {
        ConstStmt { inner }
    }
    pub fn raw(&self) -> &*mut lang::ConstStmt {
        &self.inner
    }
}

impl Stmt for ConstStmt {
    fn cast(&mut self) -> *mut lang::Stmt {
        self.inner.cast()
    }
}

pub struct RangeForStmt {
    inner: *mut lang::RangeForStmt,
}

impl RangeForStmt {
    pub fn new(inner: *mut lang::RangeForStmt) -> Self {
        RangeForStmt { inner }
    }
    pub fn raw(&self) -> &*mut lang::RangeForStmt {
        &self.inner
    }
}

impl Stmt for RangeForStmt {
    fn cast(&mut self) -> *mut lang::Stmt {
        self.inner.cast()
    }
}

pub struct StructForStmt {
    inner: *mut lang::StructForStmt,
}

impl StructForStmt {
    pub fn new(inner: *mut lang::StructForStmt) -> Self {
        StructForStmt { inner }
    }
    pub fn raw(&self) -> &*mut lang::StructForStmt {
        &self.inner
    }
}

impl Stmt for StructForStmt {
    fn cast(&mut self) -> *mut lang::Stmt {
        self.inner.cast()
    }
}

pub struct MeshForStmt {
    inner: *mut lang::MeshForStmt,
}

impl MeshForStmt {
    pub fn new(inner: *mut lang::MeshForStmt) -> Self {
        MeshForStmt { inner }
    }
    pub fn raw(&self) -> &*mut lang::MeshForStmt {
        &self.inner
    }
}

impl Stmt for MeshForStmt {
    fn cast(&mut self) -> *mut lang::Stmt {
        self.inner.cast()
    }
}

pub struct FuncCallStmt {
    inner: *mut lang::FuncCallStmt,
}

impl FuncCallStmt {
    pub fn new(inner: *mut lang::FuncCallStmt) -> Self {
        FuncCallStmt { inner }
    }
    pub fn raw(&self) -> &*mut lang::FuncCallStmt {
        &self.inner
    }
}

impl Stmt for FuncCallStmt {
    fn cast(&mut self) -> *mut lang::Stmt {
        self.inner.cast()
    }
}

pub struct ReferenceStmt {
    inner: *mut lang::ReferenceStmt,
}

impl ReferenceStmt {
    pub fn new(inner: *mut lang::ReferenceStmt) -> Self {
        ReferenceStmt { inner }
    }
    pub fn raw(&self) -> &*mut lang::ReferenceStmt {
        &self.inner
    }
}

impl Stmt for ReferenceStmt {
    fn cast(&mut self) -> *mut lang::Stmt {
        self.inner.cast()
    }
}

pub struct GetElementStmt {
    inner: *mut lang::GetElementStmt,
}

impl GetElementStmt {
    pub fn new(inner: *mut lang::GetElementStmt) -> Self {
        GetElementStmt { inner }
    }
    pub fn raw(&self) -> &*mut lang::GetElementStmt {
        &self.inner
    }
}

impl Stmt for GetElementStmt {
    fn cast(&mut self) -> *mut lang::Stmt {
        self.inner.cast()
    }
}

pub struct ReturnStmt {
    inner: *mut lang::ReturnStmt,
}

impl ReturnStmt {
    pub fn new(inner: *mut lang::ReturnStmt) -> Self {
        ReturnStmt { inner }
    }
    pub fn raw(&self) -> &*mut lang::ReturnStmt {
        &self.inner
    }
}

impl Stmt for ReturnStmt {
    fn cast(&mut self) -> *mut lang::Stmt {
        self.inner.cast()
    }
}

pub struct WhileStmt {
    inner: *mut lang::WhileStmt,
}

impl WhileStmt {
    pub fn new(inner: *mut lang::WhileStmt) -> Self {
        WhileStmt { inner }
    }
    pub fn raw(&self) -> &*mut lang::WhileStmt {
        &self.inner
    }
}

impl Stmt for WhileStmt {
    fn cast(&mut self) -> *mut lang::Stmt {
        self.inner.cast()
    }
}

pub struct IntegerOffsetStmt {
    inner: *mut lang::IntegerOffsetStmt,
}

impl IntegerOffsetStmt {
    pub fn new(inner: *mut lang::IntegerOffsetStmt) -> Self {
        IntegerOffsetStmt { inner }
    }
    pub fn raw(&self) -> &*mut lang::IntegerOffsetStmt {
        &self.inner
    }
}

impl Stmt for IntegerOffsetStmt {
    fn cast(&mut self) -> *mut lang::Stmt {
        self.inner.cast()
    }
}

pub struct LinearizeStmt {
    inner: *mut lang::LinearizeStmt,
}

impl LinearizeStmt {
    pub fn new(inner: *mut lang::LinearizeStmt) -> Self {
        LinearizeStmt { inner }
    }
    pub fn raw(&self) -> &*mut lang::LinearizeStmt {
        &self.inner
    }
}

impl Stmt for LinearizeStmt {
    fn cast(&mut self) -> *mut lang::Stmt {
        self.inner.cast()
    }
}

pub struct GetRootStmt {
    inner: *mut lang::GetRootStmt,
}

impl GetRootStmt {
    pub fn new(inner: *mut lang::GetRootStmt) -> Self {
        GetRootStmt { inner }
    }
    pub fn raw(&self) -> &*mut lang::GetRootStmt {
        &self.inner
    }
}

impl Stmt for GetRootStmt {
    fn cast(&mut self) -> *mut lang::Stmt {
        self.inner.cast()
    }
}

pub struct SNodeLookupStmt {
    inner: *mut lang::SNodeLookupStmt,
}

impl SNodeLookupStmt {
    pub fn new(inner: *mut lang::SNodeLookupStmt) -> Self {
        SNodeLookupStmt { inner }
    }
    pub fn raw(&self) -> &*mut lang::SNodeLookupStmt {
        &self.inner
    }
}

impl Stmt for SNodeLookupStmt {
    fn cast(&mut self) -> *mut lang::Stmt {
        self.inner.cast()
    }
}

pub struct GetChStmt {
    inner: *mut lang::GetChStmt,
}

impl GetChStmt {
    pub fn new(inner: *mut lang::GetChStmt) -> Self {
        GetChStmt { inner }
    }
    pub fn raw(&self) -> &*mut lang::GetChStmt {
        &self.inner
    }
}

impl Stmt for GetChStmt {
    fn cast(&mut self) -> *mut lang::Stmt {
        self.inner.cast()
    }
}

pub struct OffloadedStmt {
    inner: *mut lang::OffloadedStmt,
}

impl OffloadedStmt {
    pub fn new(inner: *mut lang::OffloadedStmt) -> Self {
        OffloadedStmt { inner }
    }
    pub fn raw(&self) -> &*mut lang::OffloadedStmt {
        &self.inner
    }
}

impl Stmt for OffloadedStmt {
    fn cast(&mut self) -> *mut lang::Stmt {
        self.inner.cast()
    }
}

pub struct LoopIndexStmt {
    inner: *mut lang::LoopIndexStmt,
}

impl LoopIndexStmt {
    pub fn new(inner: *mut lang::LoopIndexStmt) -> Self {
        LoopIndexStmt { inner }
    }
    pub fn raw(&self) -> &*mut lang::LoopIndexStmt {
        &self.inner
    }
}

impl Stmt for LoopIndexStmt {
    fn cast(&mut self) -> *mut lang::Stmt {
        self.inner.cast()
    }
}

pub struct LoopLinearIndexStmt {
    inner: *mut lang::LoopLinearIndexStmt,
}

impl LoopLinearIndexStmt {
    pub fn new(inner: *mut lang::LoopLinearIndexStmt) -> Self {
        LoopLinearIndexStmt { inner }
    }
    pub fn raw(&self) -> &*mut lang::LoopLinearIndexStmt {
        &self.inner
    }
}

impl Stmt for LoopLinearIndexStmt {
    fn cast(&mut self) -> *mut lang::Stmt {
        self.inner.cast()
    }
}

pub struct GlobalThreadIndexStmt {
    inner: *mut lang::GlobalThreadIndexStmt,
}

impl GlobalThreadIndexStmt {
    pub fn new(inner: *mut lang::GlobalThreadIndexStmt) -> Self {
        GlobalThreadIndexStmt { inner }
    }
    pub fn raw(&self) -> &*mut lang::GlobalThreadIndexStmt {
        &self.inner
    }
}

impl Stmt for GlobalThreadIndexStmt {
    fn cast(&mut self) -> *mut lang::Stmt {
        self.inner.cast()
    }
}

pub struct BlockCornerIndexStmt {
    inner: *mut lang::BlockCornerIndexStmt,
}

impl BlockCornerIndexStmt {
    pub fn new(inner: *mut lang::BlockCornerIndexStmt) -> Self {
        BlockCornerIndexStmt { inner }
    }
    pub fn raw(&self) -> &*mut lang::BlockCornerIndexStmt {
        &self.inner
    }
}

impl Stmt for BlockCornerIndexStmt {
    fn cast(&mut self) -> *mut lang::Stmt {
        self.inner.cast()
    }
}

pub struct GlobalTemporaryStmt {
    inner: *mut lang::GlobalTemporaryStmt,
}

impl GlobalTemporaryStmt {
    pub fn new(inner: *mut lang::GlobalTemporaryStmt) -> Self {
        GlobalTemporaryStmt { inner }
    }
    pub fn raw(&self) -> &*mut lang::GlobalTemporaryStmt {
        &self.inner
    }
}

impl Stmt for GlobalTemporaryStmt {
    fn cast(&mut self) -> *mut lang::Stmt {
        self.inner.cast()
    }
}

pub struct ThreadLocalPtrStmt {
    inner: *mut lang::ThreadLocalPtrStmt,
}

impl ThreadLocalPtrStmt {
    pub fn new(inner: *mut lang::ThreadLocalPtrStmt) -> Self {
        ThreadLocalPtrStmt { inner }
    }
    pub fn raw(&self) -> &*mut lang::ThreadLocalPtrStmt {
        &self.inner
    }
}

impl Stmt for ThreadLocalPtrStmt {
    fn cast(&mut self) -> *mut lang::Stmt {
        self.inner.cast()
    }
}

pub struct BlockLocalPtrStmt {
    inner: *mut lang::BlockLocalPtrStmt,
}

impl BlockLocalPtrStmt {
    pub fn new(inner: *mut lang::BlockLocalPtrStmt) -> Self {
        BlockLocalPtrStmt { inner }
    }
    pub fn raw(&self) -> &*mut lang::BlockLocalPtrStmt {
        &self.inner
    }
}

impl Stmt for BlockLocalPtrStmt {
    fn cast(&mut self) -> *mut lang::Stmt {
        self.inner.cast()
    }
}

pub struct ClearListStmt {
    inner: *mut lang::ClearListStmt,
}

impl ClearListStmt {
    pub fn new(inner: *mut lang::ClearListStmt) -> Self {
        ClearListStmt { inner }
    }
    pub fn raw(&self) -> &*mut lang::ClearListStmt {
        &self.inner
    }
}

impl Stmt for ClearListStmt {
    fn cast(&mut self) -> *mut lang::Stmt {
        self.inner.cast()
    }
}

pub struct InternalFuncStmt {
    inner: *mut lang::InternalFuncStmt,
}

impl InternalFuncStmt {
    pub fn new(inner: *mut lang::InternalFuncStmt) -> Self {
        InternalFuncStmt { inner }
    }
    pub fn raw(&self) -> &*mut lang::InternalFuncStmt {
        &self.inner
    }
}

impl Stmt for InternalFuncStmt {
    fn cast(&mut self) -> *mut lang::Stmt {
        self.inner.cast()
    }
}

pub struct TexturePtrStmt {
    inner: *mut lang::TexturePtrStmt,
}

impl TexturePtrStmt {
    pub fn new(inner: *mut lang::TexturePtrStmt) -> Self {
        TexturePtrStmt { inner }
    }
    pub fn raw(&self) -> &*mut lang::TexturePtrStmt {
        &self.inner
    }
}

impl Stmt for TexturePtrStmt {
    fn cast(&mut self) -> *mut lang::Stmt {
        self.inner.cast()
    }
}

pub struct TextureOpStmt {
    inner: *mut lang::TextureOpStmt,
}

impl TextureOpStmt {
    pub fn new(inner: *mut lang::TextureOpStmt) -> Self {
        TextureOpStmt { inner }
    }
    pub fn raw(&self) -> &*mut lang::TextureOpStmt {
        &self.inner
    }
}

impl Stmt for TextureOpStmt {
    fn cast(&mut self) -> *mut lang::Stmt {
        self.inner.cast()
    }
}

pub struct AdStackAllocaStmt {
    inner: *mut lang::AdStackAllocaStmt,
}

impl AdStackAllocaStmt {
    pub fn new(inner: *mut lang::AdStackAllocaStmt) -> Self {
        AdStackAllocaStmt { inner }
    }
    pub fn raw(&self) -> &*mut lang::AdStackAllocaStmt {
        &self.inner
    }
}

impl Stmt for AdStackAllocaStmt {
    fn cast(&mut self) -> *mut lang::Stmt {
        self.inner.cast()
    }
}

pub struct AdStackLoadTopStmt {
    inner: *mut lang::AdStackLoadTopStmt,
}

impl AdStackLoadTopStmt {
    pub fn new(inner: *mut lang::AdStackLoadTopStmt) -> Self {
        AdStackLoadTopStmt { inner }
    }
    pub fn raw(&self) -> &*mut lang::AdStackLoadTopStmt {
        &self.inner
    }
}

impl Stmt for AdStackLoadTopStmt {
    fn cast(&mut self) -> *mut lang::Stmt {
        self.inner.cast()
    }
}

pub struct AdStackLoadTopAdjStmt {
    inner: *mut lang::AdStackLoadTopAdjStmt,
}

impl AdStackLoadTopAdjStmt {
    pub fn new(inner: *mut lang::AdStackLoadTopAdjStmt) -> Self {
        AdStackLoadTopAdjStmt { inner }
    }
    pub fn raw(&self) -> &*mut lang::AdStackLoadTopAdjStmt {
        &self.inner
    }
}

impl Stmt for AdStackLoadTopAdjStmt {
    fn cast(&mut self) -> *mut lang::Stmt {
        self.inner.cast()
    }
}

pub struct AdStackPopStmt {
    inner: *mut lang::AdStackPopStmt,
}

impl AdStackPopStmt {
    pub fn new(inner: *mut lang::AdStackPopStmt) -> Self {
        AdStackPopStmt { inner }
    }
    pub fn raw(&self) -> &*mut lang::AdStackPopStmt {
        &self.inner
    }
}

impl Stmt for AdStackPopStmt {
    fn cast(&mut self) -> *mut lang::Stmt {
        self.inner.cast()
    }
}

pub struct AdStackPushStmt {
    inner: *mut lang::AdStackPushStmt,
}

impl AdStackPushStmt {
    pub fn new(inner: *mut lang::AdStackPushStmt) -> Self {
        AdStackPushStmt { inner }
    }
    pub fn raw(&self) -> &*mut lang::AdStackPushStmt {
        &self.inner
    }
}

impl Stmt for AdStackPushStmt {
    fn cast(&mut self) -> *mut lang::Stmt {
        self.inner.cast()
    }
}

pub struct AdStackAccAdjointStmt {
    inner: *mut lang::AdStackAccAdjointStmt,
}

impl AdStackAccAdjointStmt {
    pub fn new(inner: *mut lang::AdStackAccAdjointStmt) -> Self {
        AdStackAccAdjointStmt { inner }
    }
    pub fn raw(&self) -> &*mut lang::AdStackAccAdjointStmt {
        &self.inner
    }
}

impl Stmt for AdStackAccAdjointStmt {
    fn cast(&mut self) -> *mut lang::Stmt {
        self.inner.cast()
    }
}

pub struct BitStructStoreStmt {
    inner: *mut lang::BitStructStoreStmt,
}

impl BitStructStoreStmt {
    pub fn new(inner: *mut lang::BitStructStoreStmt) -> Self {
        BitStructStoreStmt { inner }
    }
    pub fn raw(&self) -> &*mut lang::BitStructStoreStmt {
        &self.inner
    }
}

impl Stmt for BitStructStoreStmt {
    fn cast(&mut self) -> *mut lang::Stmt {
        self.inner.cast()
    }
}

pub struct MeshRelationAccessStmt {
    inner: *mut lang::MeshRelationAccessStmt,
}

impl MeshRelationAccessStmt {
    pub fn new(inner: *mut lang::MeshRelationAccessStmt) -> Self {
        MeshRelationAccessStmt { inner }
    }
    pub fn raw(&self) -> &*mut lang::MeshRelationAccessStmt {
        &self.inner
    }
}

impl Stmt for MeshRelationAccessStmt {
    fn cast(&mut self) -> *mut lang::Stmt {
        self.inner.cast()
    }
}

pub struct MeshIndexConversionStmt {
    inner: *mut lang::MeshIndexConversionStmt,
}

impl MeshIndexConversionStmt {
    pub fn new(inner: *mut lang::MeshIndexConversionStmt) -> Self {
        MeshIndexConversionStmt { inner }
    }
    pub fn raw(&self) -> &*mut lang::MeshIndexConversionStmt {
        &self.inner
    }
}

impl Stmt for MeshIndexConversionStmt {
    fn cast(&mut self) -> *mut lang::Stmt {
        self.inner.cast()
    }
}

pub struct MeshPatchIndexStmt {
    inner: *mut lang::MeshPatchIndexStmt,
}

impl MeshPatchIndexStmt {
    pub fn new(inner: *mut lang::MeshPatchIndexStmt) -> Self {
        MeshPatchIndexStmt { inner }
    }
    pub fn raw(&self) -> &*mut lang::MeshPatchIndexStmt {
        &self.inner
    }
}

impl Stmt for MeshPatchIndexStmt {
    fn cast(&mut self) -> *mut lang::Stmt {
        self.inner.cast()
    }
}

pub struct MatrixInitStmt {
    inner: *mut lang::MatrixInitStmt,
}

impl MatrixInitStmt {
    pub fn new(inner: *mut lang::MatrixInitStmt) -> Self {
        MatrixInitStmt { inner }
    }
    pub fn raw(&self) -> &*mut lang::MatrixInitStmt {
        &self.inner
    }
}

impl Stmt for MatrixInitStmt {
    fn cast(&mut self) -> *mut lang::Stmt {
        self.inner.cast()
    }
}
