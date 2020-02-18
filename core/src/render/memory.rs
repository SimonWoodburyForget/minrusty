/// A trait for plain-old-data types.
///
/// A POD type does not have invalid bit patterns and can be safely created from arbitrary bit pattern.
/// The `Pod` trait is implemented for standard integer and floating point numbers as well as common
/// arrays of them (for example `[f32; 2]`).
pub unsafe trait Pod {}

macro_rules! impl_pod {
    ( ty = $($ty:ty)* ) => { $( unsafe impl Pod for $ty {} )* };
    ( ar = $($tt:expr)* ) => { $( unsafe impl<T: Pod> Pod for [T; $tt] {} )* };
}

impl_pod! { ty = isize usize i8 u8 i16 u16 i32 u32 i64 u64 f32 f64 }
impl_pod! {
    ar = 0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16
        17 18 19 20 21 22 23 24 25 26 27 28 29 30 31 32
}

unsafe impl<T: Pod> Pod for &[T] {}
