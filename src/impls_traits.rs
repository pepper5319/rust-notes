pub fn example(){
    no_traits::main();
    traits_no_default::main();
    associated_functions::main();
    trait_objects::main();
}

/*
    Impls and Traits
        Impls: Used to define methods for structs and enums
        Traits: Like interfaces. Define functionality a type
            must provide
                Can include default implementation of methods
*/

mod no_traits {
    struct Player{
        first_name: String,
        last_name: String
    }

    impl Player {
        fn full_name(&self) -> String {
            format!("{} {}", self.first_name, self.last_name)
        }
    }

    pub fn main(){
        let player_1 = Player {
            first_name: "Woot".to_string(),
            last_name: "Poot".to_string()
        };
        println!("Player 01: {}", player_1.full_name());
    }
}

mod traits_no_default{
    struct Player{
        first_name: String,
        last_name: String
    }

    trait FullName {
        fn full_name(&self) -> String;
    }

    impl FullName for Player {
        fn full_name(&self) -> String {
            format!("{} {}", self.first_name, self.last_name)
        }
    }
    pub fn main(){
        let player_2 = Player {
            first_name: "Poot".to_string(),
            last_name: "Woot".to_string()
        };
        println!("Player 02: {}", player_2.full_name());
    }
}

mod traits_default{
    trait Foo {
        fn bar(&self);
        fn baz(&self) { println!("BAZ!"); }
    }
}

mod associated_functions {
    // Static functions can be called without creating a new object. These are called
    //  'Associated Functions' in Rust. Use '::' notation
    struct Player{
        first_name: String,
        last_name: String
    }

    impl Player {
        fn new (first_name: String, last_name: String) -> Player {
            return Player{
                first_name: first_name,
                last_name: last_name
            }
        }
        fn full_name(&self) -> String {
            format!("{} {}", self.first_name, self.last_name)
        }
    }
    pub fn main(){
        let player = Player::new("Wooter".to_string(), "Pooter".to_string());
        println!("Player: {}", player.full_name());
    }
}

mod trait_inheritance{
    trait Person {
        fn full_name(&self) -> String;
    }
    trait Employee : Person { //  inherits from Person trait
        fn job_title(&self) -> String;
    }
}

mod trait_objects{
    // Rust favors static dispatch, supports dynamic dispatch with 'trait objects'
    //  Dynamic Dispatch: Selecting which impl of a polymorphic operation to call at runtime
    trait GetSound {
        fn get_sound(&self) -> String;
    }
    struct Cat {
        sound: String
    }
        impl GetSound for Cat {
            fn get_sound(&self) -> String {
                self.sound.clone()
            }
        }
    struct Bell {
        sound: String
    }
        impl GetSound for Bell {
            fn get_sound(&self) -> String {
                self.sound.clone()
            }
        }
    fn make_sound<T: GetSound>(t: &T){
        println!("{}!", t.get_sound())
    }
    pub fn main(){
        let kitty = Cat {sound: "Meow".to_string()};
        let bell = Bell {sound: "Ding Dong".to_string()};
        make_sound(&kitty);
        make_sound(&bell);
    }
}
