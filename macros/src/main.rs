mod hello_macro;
mod greet_someone;
mod add;
mod reprint;
mod dbg_if;

fn main(){
    hello!();
    greet!("John");

    println!("2 + 5 = {}", add!(2, 5));

    // reprint
    reprint!("Hello World!", 5);

    // dbg if
    dbg_if!(true, "This is a debug line. It should be printed.");
    dbg_if!(false, "This is a debug line. It should not be printed :) .");
    
}
