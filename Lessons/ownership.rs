fn greet(name: &String){
    // This functions spects a REFERENCE to a variable
    // When this happends we say that this functions BORROWS the 
    // variable, so it has no ownership of it.
    println!("{}", name);
}

fn takes_ownership(some_string: String){
    // some_string enters this scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed

fn makes_copy(some_integer: i32){
    // some_integer enters this scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens

fn gives_ownership() -> String{
    // gives_ownership will move its
    // return value into the function
    // that calls it
    let go: String = String::from("Given_ownership");  // some_string comes into scope
    go  // some_string is returned and
        // moves out to the calling
        // function
}

fn takes_and_gives_back(a_string: String) -> String{
    // gives_ownership will move its
    // return value into the function
    // that calls it
    a_string
}

fn calculate_length(s: String) -> (String, usize){
    let length = s.len();  // len() returns the length of a String
    (s, length)
}

fn main(){
    // Create a new String that will be inside the heap, the pointer
    // is in the stack
    let name: String = "John".to_string();
    println!("{}", name);
    let a = name; // now is a which is pointing to the heap
    println!("{}", a);

    // let b = name; // will fail because name is no pointing to anything
    let b = a.clone();  // this will work
    println!("{}", b);

    greet(&a);

    // examples with strings
    let mut s = String::from("Hello World");  // takes a &str, a string literal
    println!("{}", s);
    s.push_str(" people !");  // apends a string literal to the end of the string
    println!("{}", s);

    //
    takes_ownership(s);
    //println!("{}", s); // will fail cause "move occurs because `s` has type `String`, which does
    //not implement the `Copy` trait"
    let i: i32 = 3;
    makes_copy(i);
    println!("{}", i);

    let s1: String = gives_ownership();  // gives_ownership moves its return
                                         // value into s1
    println!("{}", s1);
    let s2 = String::from("hello");  // s2 comes into scope
    let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3
    //println!("{:?}", s3);
    println!("{}", s3);

    let s_1 = String::from("string_to_be_measure");
    let (s_2, length) = calculate_length(s_1);
    println!("{}: {}", s_2, length);
}

