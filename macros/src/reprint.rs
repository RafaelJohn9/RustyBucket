#[macro_export]
macro_rules! reprint {
    ($text: expr, $n: expr) => {
        for _ in 0..$n{
            println!("{}", $text);
        }
    };
}
