
#[derive(Debug)]
struct UserData {
    name : String,
    email : String,
    is_active : bool,
    sign_in_count : u64
}


fn build_user(name : String , email : String ) -> UserData{
    let user =  UserData{
        name,
        email,
        is_active: true,
        sign_in_count:1

    };
    user
}

fn update_structs(user: UserData) -> UserData{
    let user = UserData{
        email: "user2@gmail.com".to_string(),
        ..user
    };
    user

}

// The area calculation program with structs and methods
struct Rectangle {
    width : u32,
    height : u32
}


impl Rectangle{
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self , other : &Rectangle) -> bool{
        self.width > other.width && self.height > other.height
    }

    fn square(size : u32) -> Self{
        Self{
            width:size,
            height:size
        }
    }
}

fn area(rec : Rectangle) -> u32{
rec.width * rec.height
}

fn main() {

    let user = build_user("Testing".to_string() , "testing@gmail.com".to_string());
    println!( "{:#?}",user);
    let user1 = update_structs(user);
    println!("{:#?}" , user1);
    // println!( "{:?}",user); will throw ownership error

    let rect = Rectangle{width : 12 , height : 12};
    let area_cal = area(rect);
    println!("Area Calculated: {} " , area_cal);

    dbg!(&area_cal);

    // now call the method of Rectangle struct
    let rect1 = Rectangle{width : 13 , height : 20};
    let rect2 = Rectangle{width : 11 , height : 10};
    let rect3 = Rectangle{width : 14 , height : 19};


    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    println!(
        "Can hold the Rec ?: {}",
        rect1.can_hold(&rect2)
    );
    println!(
        "Can hold the Rec ? : {}",
        rect1.can_hold(&rect3)
    );

    // using methods with out taking Self as first argument means it will return new instance of struct
    let new_instance = Rectangle::square(20);
    println!("Square Width: {0} && Square Height:  {1}" , new_instance.width , new_instance.height);



}

