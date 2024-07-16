


use traits_p::{Summary, Tweet , News};


fn main() {
    let data = News{
        headlines: String::from("Three Main Had An Accident At Main Road"),
        location : String::from("Main Road"),
        author : String::from("Akram"),
        context : String::from("Accident")
    };

    println!("{}",data.summarize());

    let tweet = Tweet{
        username : String::from("Akram"),
        content : String::from("No more hard work , smart work"),
        reply : String::from("Null"),
        retweet : String::from("Null")
    };
    println!("Tweet : {}",tweet.summarize());

     notify(&tweet);
     notify(&data);

}
// simple fn which will return the data of trait
pub fn notify1(item : &impl Summary){
    println!("News : {}" , item.summarize())
}

// now what if we want to bound this function to trait 

pub fn notify<T : Summary>( item : &T){
    println!("News : {}" , item.summarize())

}


// pub fn notify(item1: &impl Summary, item2: &impl Summary) { both of item need to impl Summary Traits
// pub fn notify<T: Summary>(item1: &T, item2: &T) { both of item have to be same type

