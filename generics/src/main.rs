struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &U {
        &self.y
    }
}

fn main() {
    // simple way of finding largest number
    let number_list = vec![20, 30, 40, 50, 60, 70];
    let mut largest_value = &number_list[0];
    for number in &number_list {
        if number > largest_value {
            largest_value = number;
        }
    }

    println!("Largest Number :{}", largest_value);

    let lar = largest(&number_list);
    println!("Largest Number :{lar}");

    // test both different function of two data types
    let lar = largest_i32(&number_list);
    println!("Largest Number :{lar}");

    let number_char = vec!['a', 'b', 'c', 'd', 'z'];

    let lar_char = largest_char(&number_char);
    println!("Largest Number :{lar_char}");

    let la_char = largest_new(&number_char);
    let la_32 = largest_new(&number_list);
    println!("Using Generic largest char :{lar_char} and largest number {la_32}");

    // points system

    let p = Point { x: 5, y: 10.8 };
    println!("{:?}", p.x);
    println!("{:?}", p.y);
    println!("p.x = {}", p.x());
    println!("p.y = {}", p.y());
}

// enc the largest code in function

fn largest(list: &[i32]) -> &i32 {
    let mut largest = &list[0];
    for number in list {
        if number > largest {
            largest = number;
        }
    }
    largest
}

// now if in case we have to find the largest value in char and i32 two different datatypes

fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];
    for number in list {
        if number > largest {
            largest = number;
        }
    }
    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];
    for number in list {
        if number > largest {
            largest = number;
        }
    }
    largest
}

// now we will try to make this function largest_type as a generic

fn largest_new<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for number in list {
        if number > largest {
            largest = number;
        }
    }
    largest
}
