fn main() {
    // let r ;

    // {
    //     let x = 5;
    //     r = &x
    // }
    // println!("Hello, world! , {r}"); 
//     --> src/main.rs:6:13
//     |
//   5 |         let x = 5;
//     |             - binding `x` declared here
//   6 |         r = &x
//     |             ^^ borrowed value does not live long enough
//   7 |     }
//     |     - `x` dropped here while still borrowed
//   8 |     println!("Hello, world! , {r}");
//     |                               --- borrow later used here



// 2nd Issue 

let string1 = String::from("abcd");
let result;
{
let string2 = String::from("xyz");
result= longest(string1.as_str() , string2.as_str());
println!("longest is {result}");

}

}

// fn longest(x: &str, y: &str) -> &str {
//     |               ----     ----     ^ expected named lifetime parameter
// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }


// &i32        // a reference
// &'a i32     // a reference with an explicit lifetime
// &'a mut i32 // a mutable reference with an explicit lifetime

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}