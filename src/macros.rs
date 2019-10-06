#[cfg(any(debug_assertions, feature = "glam-assert"))]
macro_rules! glam_assert {
    ($($arg:tt)*) => ( assert!($($arg)*); )
}
#[cfg(not(any(debug_assertions, feature = "glam-assert")))]
macro_rules! glam_assert {
    ($($arg:tt)*) => {};
}

macro_rules! is_normalized {
    ($self:expr, $max_diff:expr) => {
        ($self.length_squared() - 1.0).abs() <= $max_diff
    };
    ($self:expr) => {
        is_normalized!($self, core::f32::EPSILON)
    };
}

macro_rules! abs_diff {
    ($self:expr, $rhs:expr) => {
        ($self - $rhs).abs()
    };
}

macro_rules! abs_diff_eq {
    ($self:expr, $rhs:expr, $max_diff:expr) => {
        abs_diff!($self, $rhs).cmple($max_diff).all()
    };
    ($self:expr, $rhs:expr) => {
        abs_diff_eq!($self, $rhs, Self::splat(core::f32::EPSILON))
    };
}

macro_rules! abs_diff_zero {
    ($self:expr, $max_diff:expr) => {
        $self.abs().cmple($max_diff).all()
    };
    ($self:expr) => {
        abs_diff_zero!($self, Self::splat(core::f32::EPSILON))
    };
}
