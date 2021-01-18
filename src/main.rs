fn main() {
    //functions::example();
    primatives::example();
}

mod primatives{
    fn prims(){
        // Booleans
        //   no TRUE, FALSE, 1, 0
        let x = true;
        let y: bool = false;

        // Chars (Single unicode scalar value)
        //   Single quotes only
        //   Because of unicode char is 4 bytes, not single byte
        let char_x = 'x';
        let char_y: char = '😮';
        println!("{:?}", char_y);

        // Signed Integers
        //   8, 16, 32, 64, 128 bits
        //   Fixed sized, signed(+/-) integers
        //   Default is 'i32'
        let _i_x = 10;
        let _i_y: i8 = -128;

        // Unsigned Integers
        //   8, 16, 32, 64, 128 bits
        //   Fixed sized, unsigned(0/+) integers
        let _u_x: u8 = 10;

        // Pointer sized signed/unsigned integers
        //   Depends on computer architecture
        //   Default 32 bits on 32-bit platform
        //      64 bits on 64-bit platform
        let _i_x: isize = 10;
        let _u_x: usize = 20;

        // Floating Point Numbers
        //   'f32' similar to float(single precision)
        //   'f64' similar to float(double precision)
        //   default is 'f64'
        let _f_x = 1.5;
        let _f_y: f64 = 2.0;
    }
    fn prim_collections(){
        // Arrays
        //   Immutable by default
        //   Element count CANNOT be changed
        let a = [1, 2, 3];
        let a: [i32; 3] = [1, 2, 3]; // [type; # of elements]

        let b: [i32; 0]; // An empty array

        let mut c: [i32; 3] = [1, 2, 3]; // Mutable array
        c[0] = 2;
        c[1] = 4;
        c[2] = 6;
        println!("{:?}", c);
        println!("{:#?}", c);

        let d = [0; 5]; // [0, 0, 0, 0, 0]
        let e = ["x"; 3]; // ["x", "x", "x"]
    }
    pub fn example(){
        prim_collections();
    }
}

mod functions {
    fn sum(a: i32, b: i32) -> i32 {
        return a + b;
    }

    pub fn example(){
        // Normal function call
        let mut a : i32 = 5;
        let mut b : i32 = 10;
        let mut c : i32 = sum(a, b);
        println!("{:?}", c);

        // Function pointers
        let f1: fn(i32, i32) -> i32 = sum;
        a = 2;
        b = 2;
        c = f1(a, b);
        println!("{:?}", c);

        // Closures (aka lamda functions)
        a = 3;
        let square = |i: i32| -> i32 {
            i * i
        };
        println!("{:?}", square(a));

        // Closures w/o type declarations
        let square2 = |i| i * i; // '{}' optional for single line closures
        println!("{:?}", square2(a));

        // Closures created and called together
        let square3 = |i| -> i32 {i * i}(a); // Must include return type
        println!("{:?}", square3);
    }
}
