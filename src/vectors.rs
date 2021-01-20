pub fn example(){
    println!("Vectors-----");
    // Vectors
    //   Resizable array
    //   All elements are same data type
    //   Can be any data type (generic)
    //   Data allocated to heap

    // Create empty (MUST specify type)
    let mut a: Vec<i32> = Vec::new(); // With new()
    let mut b: Vec<i32> = vec![];     // With macro

    // Create with data
    let mut a1: Vec<i32> = vec![1, 2, 3];
    let mut a2: Vec<i32> = vec![1i32, 2, 3]; // Sufix first value with type
    let mut a3 = vec![1, 2, 3];
    let mut a4 = vec![0; 10]; // 10 zeros

    // Access & changing data
    let mut b: Vec<i32> = vec![1, 2, 3, 4, 5];
    print_vector(&mut b, "Initial");
    b[0] = 5;
    b[1] = 4;
    print_vector(&mut b, "Changed Data");

    // Push & Pop
    let mut c: Vec<i32> = vec![];
    c.push(1);
    c.push(2);
    print_vector(&mut c, "Push");
    c.pop();
    print_vector(&mut c, "Pop");

    // Capacity & reallocation
    let mut d: Vec<i32> = Vec::with_capacity(10);
    println!("Length: {}, Capacity: {}", d.len(), d.capacity());
    //  Done without reallocating additional memory
    for i in 0..10{
        d.push(i);
    }
    println!("Length: {}, Capacity: {}", d.len(), d.capacity());
    // May cause vector to reallocate since capacity exceeded
    d.push(11);
    println!("Length: {}, Capacity: {}", d.len(), d.capacity());
}

fn print_vector(v: &mut Vec<i32>, msg: &str){
    println!("----{}----", msg);
    for value in v.iter(){
        println!("{}", value);
    }
    println!("\n", );
}
