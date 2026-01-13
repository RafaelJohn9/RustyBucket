#[macro_export]
macro_rules! sum {
    ($($x: expr), *) =>
    {
        0 $(+ $x)*
    };
}
