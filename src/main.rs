use uuid::Uuid;
use clipboard_win::set_clipboard_string;

fn main() {
    let my_uuid = Uuid::new_v4();
    println!("Generated UUID: {}", my_uuid);

    match set_clipboard_string(my_uuid.to_string().as_str()) {
        Ok(v) => { println!("Copied to clipboard") }
        Err(e) => {  println!("Could no copy to clipboard. An error occurred: {}", e)  }
    }
}
