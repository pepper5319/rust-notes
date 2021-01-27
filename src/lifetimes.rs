pub fn example() {
    let player = Person::new("Woot", "Poot");
    let player_fullname = player.fullname();

    println!("Player: {}", player_fullname);
}

struct Person<'a> {
    fname: &'a str,
    lname: &'a str
}

    impl<'a> Person<'a> {
        fn new(fname: &'a str, lname: &'a str) -> Person<'a> {
            Person {
                fname: fname,
                lname: lname
            }
        }
        fn fullname(&self) -> String {
            format!("{} {}", self.fname, self.lname)
        }
    }

/*
    Lifetimes
        A resources can only have one owner at a time, when it goes out of scope it gets
         removed from memory.
        When dealing w/ References, specify lifetime annotations for the compiler to set
         how long those referenced resources should be alive.
        Annotations checked at compile-time (cause for slower compile times)
        Memory is managed in run time
        No deallocation needed, values aren't dropped
        Denoted with `'` followed by lowercase letter in alphabetical order
*/

 mod fun_declaration {
     /*
     Input and output parameters attach lifetimes after '&'
     Lifetimes are generic types
     */
     // Return reference
     fn function<'a>() -> &'a str {""}

     // Single input
     fn function_2<'a>(x: &'a str) {}

     // Input and returned reference
     //  Output should live as long as input
     //  exists
     fn function_3<'a>(x: &'a str) -> &'a str {""}

     // Output should live as long as y exists
     fn function_4<'a>(x: i32, y: &'a str) -> &'a str {""}

     // Inputs and output share same lifetime
     // Output should live as long as x and y exist
     fn function_5<'a>(x: &'a str, y: &'a str) -> &'a str {""}

     // Inputs have different lifetimes, output shares w/ one
     // Output should live as long as x exists
     fn function_6<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {""}
 }

 mod struct_and_enum {
     /*
        Elements w/ reference attach lifetimes after '&'
        Lifetimes are generic types
     */
     // Single Element
     // Data of x lives as long as Struct exists
     struct StructA<'a>{
         x: &'a str
     }

     // Multiple elements
     // Data should live as long as Struct exists
     struct StructB<'a>{
         x: &'a str,
         y: &'a str
     }

     // Variant w/ single element
     // Data of variant should live as long as Enum exists
     enum Enum<'a> {
         Variant(&'a str)
     }
 }

 mod impl_traits {
     struct Struct<'a> {
         x: &'a str
     }

        impl<'a> Struct<'a> {
            fn function(&self) -> &'a str{  // No need to specify `'a` after `function`; impl
                                            //  already has it
                self.x
            }
        }
 }

 mod elision {
     /*
        Lifetime Elision only in fn definitions
        Can be elided if either:
            only one input parameter passed by reference
            a parameter with either '&self' or '&mut self' reference
     */
     // only on input parameter passed by reference
     fn triple(x: &u64) -> u64{
         x * 3
     }

     // One input passed by reference
     fn filter(x: u8, y: &str) -> &str {
         if x > 5 { y } else { "invalid" }
     }

     struct Player<'a> {
         id: u8,
         name: &'a str
     }

     impl<'a> Player<'a> {
         // one input passed by reference
         fn new(id: u8, name: &str) -> Player{
             Player{
                id: id,
                name: name
             }
         }
         // &self is parameter
         fn heading_text(&self) -> String{
             format!("{}: {}", self.id, self.name)
         }
     }

 }

 mod static_annotation {
     /*
        'static is reserved
        valid for entire program
        will NEVER go out of scope
     */
     static N: i32 = 5; // constance w/ static lifetime

     // let a = "Hello, World"; // a: &'static str

     fn index() -> &'static str {
         "Hello, World"
     }

 }
