mod hello_macro;
mod greet_someone;
mod add;

fn main(){
    hello!();
    greet!("John");

    println!("2 + 5 = {}", add!(2, 5));
    
}
