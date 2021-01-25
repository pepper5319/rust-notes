pub fn example(){
    let a = [1, 2, 3];
    let b = a; // Data is copied to 'b'. Can access 'a' if needed
    println!("{:?} {:?}", a, b);

    let c = vec![1, 2, 3];
    let d = c; // Data is moved to 'd'. Cannot access 'c' anymore
    // println!("{:?} {:?}", c, d); // Error; use of moved value: 'c'
}
/*
    Ownership:
        Variable bindings have ownership of what they're bound to.
        Data can only have ONE owner at a time.
        If assigning one variable to another, or passing to function w/o reference,
         if data type is:
            Copy Type:
                Bound resources are made a copy and assigned or passed to function
                Ownership of original bindings are set to "copied" state
                Mostly PRIMATIVE data types
            Move Type:
                Bound resources MOVED to new variable binding. CANNOT ACCESS original
                 binding anymore
                Ownership of orignal binding set to "moved" state
                Mostly NON-PRIMATIVE data types
        By default, variable bindings have 'move semantics'. However this depends on the
         traits that have been implemented.
*/
