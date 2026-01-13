#[macro_export]
macro_rules! dbg_if {
    ($cond: expr, $msg: expr) => {
        if ($cond){
            println!("{}", $msg);
        }
    };
}
