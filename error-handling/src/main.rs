use std::fs::File;
use std::io::ErrorKind;
use std::io::{self, Read};


fn main() {

    // let v = vec![1, 2, 3];

    // v[99];
// panic!("Error");

let greeting_file_result = File::open("hello1.txt");
// print!("data:{:?}" , greeting_file_result);

// let greeting_file = match greeting_file_result {
//     Ok(file) => file,
//     Err(error) => panic!("Problem opening the file: {error:?}"),
// };

// print!("data after:{:?}" , greeting_file);


let greeting_file = match greeting_file_result {
    Ok(file) => file,
    Err(error) => match error.kind(){
        ErrorKind::NotFound => match File::create("hello1.txt"){
            Ok(f) => f,
            Err(error) => panic!("problem creating file : {error:?}")
        },
        other_error => {
            panic!("Problem with the file , {other_error:?}")
        }

    }
};
print!("data after:{:?}" , greeting_file);

let data = read_from_file();
print!("data :{:?}" , data);


}


// propagating errors 


fn read_from_file() -> Result<String , io::Error> {
    let message = File::open("hello1.txt");
    let mut message_file = match message{
        Ok(file) => file,
        Err(err) => return Err(err)
    };

    let mut me = String::new();
// to read the data from file
    match message_file.read_to_string(&mut me) {
        Ok(_) => Ok(me),
        Err(e) => Err(e),
    }

}


fn read_file_with_Q() -> Result<String, io::Error>  {
    let mut message = File::open("hello1.txt")?;
    let mut me = String::new();
    message.read_to_string(&mut me)?;
    Ok(me)

}



