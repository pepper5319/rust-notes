pub fn example(){
    liveness::liveness_2();
    liveness::liveness_3();
    shared_borrowing::example_1();
    shared_borrowing::example_2();
    mutable_borrowing::example_1();
    mutable_borrowing::example_2();
    mutable_borrowing::example_3();
}

/*
    Borrowing
        Referencing the original binding; Borrowing data from it
        Shared Borrowing (&T) (basically read-only):
            Can be borrowed by single or multiple users, should NOT be altered.
        Mutable Borrowing (&mut T):
            Can be borrowed AND altered by a single user, should not be accessable
             by other users at that time.

        Rules for Borrowing:
            Data can be borrowed either as shared borrow or mutable borrow at any
             given time, but not both at the same time.
            Borrowing applies for both copy types and move types.
            Liveness: Borrow lives in scope (see liveness_x() examples)
*/
mod liveness{
    pub fn liveness_2(){
        let mut a = vec![1, 2, 3];
        let b = &mut a;         // &mut borrow of 'a' starts here

        b[0] = 4;               // ---Where the weird shit starts---
        let c = &a;             // <- can shared borrow an already mutable borrowed variable
                                //     IF mutable borrowed variable isn't used afterwards
        // b[1] = 5;            // <- Can't do this and also use 'c'
        println!("Live 2: {:?}", c);

    }                           // &mut borrow of 'a' ends here

    pub fn liveness_3(){
        let mut a = vec![1, 2, 3];
        {
            let b = &mut a;     // &mut borrow of 'a' starts here
                                // ...
        }                       // &mut borrow of 'a' ends here
        println!("Live 3: {:?}", a);
    }
}

mod shared_borrowing{
    pub fn example_1(){
        let a = [1, 2, 3];
        let b = &a; // shared borrow

        println!("{:?}, {}", a, b[0]);
    }

    pub fn example_2(){
        let a = vec![1, 2, 3];
        let b = get_first(&a);
        println!("{:?}, {}", a, b);
    }

    fn get_first(a: &Vec<i32>) -> i32 {
        return a[0];
    }
}

mod mutable_borrowing{
    pub fn example_1(){
        let mut a = [1, 2, 3];
        let b = &mut a;
        b[0] = 4;
        println!("{:?}", b);
    }
    pub fn example_2(){
        let mut a = [1, 2, 3];
        {
            let b = &mut a;
            b[0] = 4;
        }

        println!("{:?}", a); // Can't use 'b' because out of scope, but 'a' data has been modified
    }
    pub fn example_3(){
        let mut a = vec![1, 2, 3];
        let b = change_first(&mut a);

        println!("{:?}, {}", a, b);
    }

    fn change_first(a: &mut Vec<i32>) -> i32 {
        a[0] = 4;
        return a[0];
    }
}
