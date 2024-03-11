
/// Static memoization
///
/// # Example
/// ```ignore
/// // The memoized function we want to expose
/// pub const fn test_memo(a: i32) -> i32 {
///     static_memo!(test_memo_impl(a) -> <i32> for [0, 1, 2])
/// }
/// 
/// // The actual implementation
/// const fn test_memo_impl(a: i32) -> i32 {
///     a + 1
/// }
/// ```
/// This produces the following code:
/// ```ignore
/// match a {
///     0 => {
///         const VALUE: i32 = test_memo_impl(0);
///         VALUE
///     }
///     1 => {
///         const VALUE: i32 = test_memo_impl(1);
///         VALUE
///     }
///     2 => {
///         const VALUE: i32 = test_memo_impl(2);
///         VALUE
///     }
///     3 => {
///         const VALUE: i32 = test_memo_impl(3);
///         VALUE
///     }
///     a => test_memo_impl(a),
/// }
/// ```
macro_rules! static_memo {
    ($name:ident($arg_name:ident) -> <$ty:ty> for [$($arg:tt),*$(,)?]) => { // see https://github.com/rust-lang/rust/issues/99380 for the reason why $arg:expr would be wrong
        match $arg_name {
            $($arg  => {
                const VALUE: $ty = $name($arg);
                VALUE
            },)*
            $arg_name => {
                $name($arg_name)
            }
        }
    };
}
pub(crate) use static_memo;