pub fn example() {

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
