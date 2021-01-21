pub fn example(){
    generic_structs::example();
    options();
    results();
}

/*
    Generics
        If a function should work with multiple types
        Used by specifying <T> first
        Must be uppercase(PascalCase) letter
*/

mod generic_functions {
    pub fn takes_anything<T>(x: T){
        // 'T' means any data type
    }
    pub fn takes_two<T>(x: T, y: T){
        // x and y must be SAME data type
    }
    pub fn takes_multiple<T, U>(x: T, y: U){
        // x and y can be different data types
    }
}

mod generic_structs {
    struct Point<T>{
        x: T,
        y: T,
    }
    pub fn example(){
        let point_a = Point{x: 0, y: 0};
        let point_b = Point{x: 0.1, y: 0.2};
    }
}

mod generic_enums {
    enum MyOption<T>{
        Some(T),
        None
    }
    enum MyResult<T, E>{
        Ok(T),
        Err(E)
    }
}

/*
    Option and Result
        Optional: Will either have Some value or None
        Result: Represents success/Ok or failure/Err
*/
fn get_id_by_username(username: &str) -> Option<usize>{
    if username == "woot"{
        return Some(12345);
    }else{
        return None;
    }

}
fn options(){
    let username = "woot";
    match get_id_by_username(username) {
        None => println!("User not found"),
        Some(i) => println!("User ID: {}", i)
    }
}

fn get_word_count(file_name: &str) -> Result<u32, &str>{
    if false{
        return Ok(10);
    }else{
        return Err("File not found");
    }
}
fn results(){
    let file_name="file.txt";
    match get_word_count(file_name) {
        Ok(i) => println!("Word Count: {}", i),
        Err(e) => println!("ERROR: {}", e)
    }
}
