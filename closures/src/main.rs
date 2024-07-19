#[derive(Debug,PartialEq,Clone , Copy)]
enum ShirtColor{
    Red,
    Green
}

struct Inventory{
    shirts : Vec<ShirtColor>
}

impl Inventory {
     fn giveaway(&self , user_ref : Option<ShirtColor>) -> ShirtColor{

        // here closure is || which wrap it self to env
user_ref.unwrap_or_else(|| self.most_stocked())
     }

     fn most_stocked(&self) ->  ShirtColor{
        let mut num_red =0;
        let mut num_green=0;
        for color in &self.shirts{
            match color {
                ShirtColor::Red => num_red +=1,
                ShirtColor::Green => num_green +=1,
            }
        }
        if num_green > num_red  {
            ShirtColor::Red
        } else {
            ShirtColor::Green
        }
     }
}


#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}



fn main() {
let store = Inventory{
    shirts: vec![ShirtColor::Red , ShirtColor::Green , ShirtColor::Red]
};

let user_ref = Some(ShirtColor::Red);
let giveaway1 = store.giveaway(user_ref);
println!("User with ref {:?} gets shirt color {:?} ", user_ref , giveaway1);

let user_ref1 = None;
let giveaway2 = store.giveaway(user_ref1);

println!("User with ref {:?} gets shirt color {:?} ", user_ref1 , giveaway2);


// Capturing References or Moving Ownership

let mut line = vec![1,2,3,4];

println!("Before defining closure: {line:?}");

let mut only_borrow = || line.push(5);
// println!("Before calling closure : {line:?}");
only_borrow();
println!("After calling closure: {line:?}");



let mut list = [
    Rectangle{width: 10, height:12},
    Rectangle{width: 13, height:14},
    Rectangle{width: 4, height:15},
    ];
list.sort_by_key(|x| x.width);
println!("{list:#?}");

// now let find the number of iteration that this list takes to sort
let mut some_iteration = 0;
list.sort_by_key(|x| {
    some_iteration +=1;
    x.width
});
println!("number of iteration {some_iteration} its take to sort {list:#?} ");

// the Iterator
let v1 = vec![1, 2, 3];

let v1_iter = v1.iter();

for val in v1_iter {
    println!("Got: {val}");
}

}
