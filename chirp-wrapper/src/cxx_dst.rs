use cxx::UniquePtr;

pub struct CVec {
    inner: UniquePtr<cxx::CxxVector<cxx::CxxString>>,
}

impl CVec {
    pub fn new() -> Self {
        let inner = cxx::CxxVector::new();
        CVec { inner }
    }

    pub fn as_ref(&self) -> &cxx::CxxVector<cxx::CxxString> {
        &self.inner
    }
}
