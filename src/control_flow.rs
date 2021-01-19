pub fn example(){
    // traditional();
    // inline();
    // _match();
    // _loop();
    // _while();
    // _for();
    _foreach();
}

fn traditional(){
    let i = 7;
    if i % 2 == 0{
        println!("Even");
    } else {
        println!("Odd");
    }

    // Return data type should be the same for each block
    let size = 7;
    let avg_size = if size < 5 {
        "Small"
    } else if size < 10 {
        "Medium"
    } else {
        "Large"
    };
    println!("Size: {}", avg_size);
}
fn inline(){
    let age: u8 = 18;
    let is_adult = if age < 18 { false } else { true }; // true
}
fn _match(){
    let width = 20;
    let size = match width {
        16 => "S", // check 16
        17 | 18 => "M", // check 17 and 18
        19 ..= 21 => "L", // check 19 to 21
        22 => "XL",
        _ => "Not available", // _ is default
    };
    println!("{}", size);
}
fn _loop(){
    // Traditional Break
    let mut a = 0;
    loop {
        if a == 0 {
            println!("Skipping {}", a);
            a += 1;
            continue;
        } else if a == 2 {
            println!("Break at {}", a);
            break;
        }
        println!("Current Value: {}", a);
        a += 1;
    }

    // Outer Break
    let mut b1 = 0;
    'outer_loop: loop {
        let mut b2 = 0;
        'inner_loop: loop {
            println!("Current Value: [{}][{}]", b1, b2);
            if b1 == 2 && b2 == 2 {
                break 'outer_loop; // kill outer loop
            } else if b2 == 5 {
                break; // kill inner loop
            }
            b2 += 1;
        }
        b1 += 1;
    }
}
fn _while(){
    // Traditional Break
    let mut a = 0;
    while a < 5 {
        if a == 0 {
            println!("Skipping {}", a);
            a += 1;
            continue;
        } else if a == 2 {
            println!("Break at {}", a);
            break;
        }
        println!("Current Value: {}", a);
        a += 1;
    }

    // Outer Break
    let mut b1 = 0;
    'outer_loop: while b1 < 6 {
        let mut b2 = 0;
        'inner_loop: while b2 < 6 {
            println!("Current Value: [{}][{}]", b1, b2);
            if b1 == 2 && b2 == 2 { break 'outer_loop; }
            b2 += 1;
        }
        b1 += 1;
    }
}
fn _for(){
    // for 0 to 10 exclusive
    for i in 0..10 {
        println!("Current Value: {}", i);
    }
    // for 1 to 10 inclusive
    for i in 1..=10 {
        println!("Current Value: {}", i);
    }
}
fn _foreach(){
    let group: [&str; 4] = ["Mark", "Lucas", "Steiner", "Ash"];

    // This works, but is bad practice. Checks group.len() on each iteration
    for n in 0..group.len() {
        println!("Current Person: {}", group[n]);
    }

    // Turn array into a simple iterator
    for person in group.iter() {
        println!("Current Person: {}", person);
    }

}
