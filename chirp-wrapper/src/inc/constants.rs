use chirp_sys;
pub enum AutodiffMode {
    KForward,
    KReverse,
    KNone,
    KCheckAutodiffValid,
}

impl AutodiffMode {
    pub(crate) fn to_sys(&self) -> chirp_sys::AutodiffMode {
        match self {
            AutodiffMode::KForward => chirp_sys::AutodiffMode::kForward,
            AutodiffMode::KReverse => chirp_sys::AutodiffMode::kReverse,
            AutodiffMode::KNone => chirp_sys::AutodiffMode::kNone,
            AutodiffMode::KCheckAutodiffValid => chirp_sys::AutodiffMode::kCheckAutodiffValid,
        }
    }
}
