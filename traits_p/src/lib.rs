pub trait Summary{
    fn summarize(&self) -> String;
}


pub struct News {
   pub headlines : String,
   pub location : String,
   pub author : String,
   pub context : String
}

impl Summary for News {
    fn summarize(&self) -> String {
        format!("{} by  {} , ({})" , self.headlines , self.author , self.location)
    }

 

}






pub struct Tweet {
    pub username : String,
    pub content : String,
    pub reply : String,
    pub retweet : String
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{} : {} " , self.username , self.content)
    }
}
