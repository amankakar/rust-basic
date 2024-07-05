fn main() {
    println!("Hello, world!");
    // first example
    another_function(5, 'h');

    // 2nd example
    let x = five();
    println!("value of : {x}");

    let y=plus_one(5);
    println!("plus one : {y}")
}

fn another_function(x:u32 , label : char) {
    println!("another function : {x}{label}");
}
// function is expression which will always consider to return values
fn five() -> i32 {
    5
}
// if we make it statement then we need to put return keyword
fn plus_one(x:i32) -> i32 {
    return x+1;
}