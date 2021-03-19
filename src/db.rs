// Remove the unused code warning, I know what I'm doing ... I guess LOL!.
#![allow(dead_code)]

use std::fs::OpenOptions;

pub struct Database {
    // A HashMap mirrors best the data structure we need for our key-value pair database.
    map: std::collections::HashMap<String, String>,
    // Keep track if the function '.flush_to_disk()' is called.
    flushed: bool,
}

impl Database {
    pub fn new(name: &str) -> Result<Database, std::io::Error> {
        let mut map = std::collections::HashMap::new();

        // Checks if the database file exists in the project root

        let db_path = std::path::Path::new(name);
        if !db_path.is_file() {
            OpenOptions::new()
                .create(true)
                .read(true)
                .append(true)
                .open(db_path)?;
        }

        // Reads the kv.db file and populates the HashMap with the current database information

        // let contents = match std::fs::read_to_string("kv.db") {
        //     Ok(c) => c,
        //     Err(error) => {
        //         return Err(error);
        //     }
        // };
        //
        // The below declararion is the same one as above, just shortened.
        // The '?' keyword is a shorthand to retrieve the Ok Result if succesfull, if not will popup the Error Result to the 'contents' variable.

        let contents = std::fs::read_to_string("kv.db")?;
        // '&str' keyword means is a 'String Slice' so it doesn't 'own' the contents of that String in memory, it's just a pointer (a ref/view) to the contents of
        // that particular String data in memory.

        for line in contents.lines() {
            let mut chunks = line.splitn(2, '\t');
            let key = chunks.next().expect("No key provided");
            let value = chunks.next().expect("No value provided");

            // Since the 'key' and 'value' are both String slices (&str), the content of both have to he 'owned', since the HashMap now will hold the data that
            // lives in the 'contents' variable. It's like creating a copy of the data in memory, since the 'contents' variable and the data it holds will be
            // 'dropped' after it gets out of scope on line 47. Rust doesn't rely on a GC to free that memory allocation, it relies on 'memory ownership'.

            // Other methods to 'borrow' and 'own' data from another String are:
            //
            // '.to_string()' or '.clone()'. '.to_owned()' it's more idiomatic to me though.
            map.insert(key.to_owned(), value.to_owned());
        }
        // Return the data to Database type!
        Ok(Database {
            map,
            flushed: false,
        })
    }

    // This is not a function but rather a 'method'.
    // Methods have to reference the data type itself using the keyword 'self'.
    //
    // Since we want to write into the 'map' that this data type implementation contains (self), we have to declare it as mutable.

    // &self ==> inmutable borrow (default as everything on Rust).
    // &mut self ==> mutable borrow.

    pub fn insert(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }

    pub fn delete(&mut self, key: String) {
        self.map.remove(&key);
    }

    pub fn flush(mut self) -> std::io::Result<()> {
        self.flushed = true;
        flush_to_disk(&self)
    }
}

fn flush_to_disk(database: &Database) -> std::io::Result<()> {
    println!("Flushing data to disk");
    let mut contents = String::new();

    // Destructure the HashMap tuple inside the for loop.
    for (key, value) in &database.map {
        contents.push_str(key);
        contents.push('\t');
        contents.push_str(value);
        contents.push('\n');
    }

    std::fs::write("kv.db", contents)
}

// When memory values are droped you can create an override a Prelude for a specific data type.
impl Drop for Database {
    fn drop(&mut self) {
        if !self.flushed {
            let _ = flush_to_disk(self);
        }
    }
}
