#[macro_export]
macro_rules! greet{
    ($x: expr) => {
        println!("Hello {}", $x);
    };
}
