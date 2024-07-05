fn main() {
    let mut s = String::from("hello");
    s.push_str(", world");
    println!("{s}");

    // reassign the string to check for ownership
    let s1 = String::from("hello");
    let mut s2 = s1.clone();

    println!("s1={s1} , s2={s2}"); // print s1 here will throw error :   ^^^^ value borrowed here after move

    // to pass a value to function without moving the ownership we need to pass the reference of it

    let len = find_length(&s1);
    println!("S : {s1} && len : {len}");
    s2.push_str(" world!");
    let strr = first_word(&s2);
    println!("str {}" , strr);
    let snd_word = second_word(&s2);
    println!("2nd word: {snd_word}"); 
}

fn find_length(s: &String) -> usize {
    s.len()
}

fn first_word(s: &String) -> &str {
    let str_bytes = s.as_bytes();
    for (i, &item) in str_bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
fn second_word(s: &String) -> &str {
    let str_bytes = s.as_bytes();
    for (i, &item) in str_bytes.iter().enumerate() {
        if item == b' ' {
            return &s[i..s.len()];
        }
    }
    &s[..]
}
//
// rules for Ownership
// 1 . Each value has Owner
// 2. Each value has only one owner
//3. if owner goes out of scope the value will be dropped

// The Rules of References
// Let’s recap what we’ve discussed about references:

// At any given time, you can have either one mutable reference or any number of immutable references.
// References must always be valid.
