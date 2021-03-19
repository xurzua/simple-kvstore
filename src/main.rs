mod db;
use db::Database;
fn main() {
    // Skip the 1st console argument which is the path of running binary program itself.
    let mut arguments = std::env::args().skip(1);
    let key = arguments.next().expect("No key provided!"); // .unwrap() will crash the program if there's no value.
    let val = arguments.next().expect("No value provided!");

    println!("The key is '{}', and the value is '{}'", key, val);

    let mut instance = Database::new("kv.db").expect("Database just crashed"); // .expect() will also crash the program, but you can customize it's output message.

    // The '&' prefix in a value means that the value is being 'borrowed' not giving away it's original ownership.
    // '.clone()' 'borrows' the String value and creates a completely new String based on data of the previous one. AKA clones the String and give its ownership to 'database.insert()'.
    instance.insert(key.clone(), val.clone());
}
