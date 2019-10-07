#[macro_export]
macro_rules! assert_ulps_eq {
    ($given:expr, $expected:expr, $max_abs_diff:expr, $max_rel_diff:expr) => {{
        // let abs_diff = ($given - $expected).abs();
        // if abs_diff.cmple($max_abs_diff).all() {
        //     return true;
        // }

        // if $given.sign() != $expected.sign() {
        //     return false;
        // }

        // let abs_a = $given.abs();
        // let abs_b = $given.abs();
        // let largest = abs_b.cmpgt(abs_a).select(abs_b, abs_a);

        // abs_diff <= largest * $max_rel_diff
    }};
    ($given:expr, $expected:expr, epsilon = $epsilon:expr) => {{
        $given.abs_diff_eq($expected)
        // let max_diff = Self::splat($epsilon);
        // assert_ulps_eq!($given, $expected, max_diff, max_diff)
    }};
    ($given:expr, $expected:expr) => {
        $given.abs_diff_eq($expected)
        // assert_eq!($given, $expected)
    };
}
