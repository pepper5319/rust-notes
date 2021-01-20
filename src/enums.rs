pub fn example(){
    status_enum();
}

/*
    Enums:
        Single type
        Contains variants access through :: notation (ex Day::Monday)
        Variant can have:
            No data (unit variant)
            Unnamed ordered data (tuple variant)
            Named data (struct variant)
*/

enum StatusMessage {
    Success, // Unit
    Warning{ category: i32, message: String }, // Struct
    Error(String) // Tuple
}
fn status_enum(){
    let mut form_status = StatusMessage::Success;
    print_status(form_status);

    form_status = StatusMessage::Warning{category: 2, message: String::from("Field is empty")};
    print_status(form_status);

    form_status = StatusMessage::Error(String::from("Connection Error"));
    print_status(form_status);
}

fn print_status(m: StatusMessage){
    match m {
        StatusMessage::Success =>
            println!("Form Submitted"),
        StatusMessage::Warning{category, message} =>
            println!("Warning: {} - {}", category, message),
        StatusMessage::Error(msg) =>
            println!("Error: {}", msg)
    }
}
