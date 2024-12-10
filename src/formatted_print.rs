
fn main(){

    // In general, the `{}` will be automatically replaced with any 
    // arguments. These will be stringfied.
    println!("{} days", 31);


    // positional arguments can be used.
    println!("{0}, this is {1}. {1}, this is {0}.", "Alice", "Bob");

    // Named arguments
    println!("{subject} {verb} {object}",
    object = "the lazy dog",
    subject="the quick brown fox",
    verb="jump over");

    //print integer
    let x = 8;
    println!("Base 10: {}", x);
    println!("Base  2: {:0>16b}", x);
    println!("Base  8: {:0>2o}", x);
    println!("Base  16: {:0>1x}", x);
    

    // add zeros after the value
    println!("Left zeros : {number:0>5}", number = x);
    println!("Right zeros : {number:0<5}", number = x);



}