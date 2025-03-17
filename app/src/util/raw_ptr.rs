/// RawPtr encapsulation that allows thread exchange
/// by implementing [Sync, Clone, Copy] traits
///
#[derive(Copy, Clone)]
pub struct RawPtr<T> {
    pub ptr: *mut T,
}
unsafe impl<T> Sync for RawPtr<T> {}
unsafe impl<T> Send for RawPtr<T> {}
