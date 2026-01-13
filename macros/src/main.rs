mod hello_macro;
mod greet_someone;
mod add;
mod reprint;

fn main(){
    hello!();
    greet!("John");

    println!("2 + 5 = {}", add!(2, 5));

    // reprint
    reprint!("Hello World!", 5);
    
}
