pub fn example(){
    // c_struct();
    t_struct();
}

/*
    Structs: 3 Variations
        C-Like Struct:
            Comma seperated name: value pairs
            Brace enclosed
            Similar to classes (WITHOUT methods) in OOP languages
            Access through dot notation (ex MyStruct.first_name)
        Tuple Struct:
            Comma seperated values
            Parenthesized list like tuples
            Looks like named tuples
        Unit Struct:
            No memebers
            Defines new type, but looks like empty tuple
            Rarely used, useful w/ generics
    OOP in Rust (since no classes)
        Structs = attributes
        Traits = methods
        Impls = connect Structs and Traits
*/

// C Struct
struct Color {
    red: u8,
    green: u8,
    blue: u8
}

fn c_struct(){
    let black = Color {red: 0, green: 0, blue: 0};
    println!("Black = rgb({}, {}, {})", black.red, black.green, black.blue); // Dot notation

    // Immutable by default, use 'mut' to make mutable
    let mut link_color = Color {red: 0, green: 0, blue: 255};
    link_color.blue = 238;
    println!("Link Color = rgb({}, {}, {})", link_color.red, link_color.green, link_color.blue);

    // Copy attributes from another instance
    let blue = Color {blue: 255, .. link_color};
    println!("Blue = rgb({}, {}, {})", blue.red, blue.green, blue.blue);

    // Destructure instance using 'let', will not destucutre 'blue'
    let Color {red: r, green: g, blue: b} = blue;
    println!("Destructured Blue = rgb({}, {}, {})", r, g, b);

}

// Tuple Struct
struct T_Color(u8, u8, u8);
struct Kilometer(i32);

fn t_struct(){
    let black = T_Color(0, 0, 0);

    // Destructure instance using 'let', will not destruure 'black'
    let T_Color(r, g, b) = black;
    println!("Destructured Black = rgb({}, {}, {})", r, g, b);

    // Newtype pattern (when tuple struct has only 1 element)
    let distance = Kilometer(20);
    // Destructure
    let Kilometer(distance_in_km) = distance;
    println!("Distance: {} km", distance_in_km);
}

// Unit Struct
//   Rarely used on its own
struct Electron;
fn u_struct(){
    let x = Electron;
}
