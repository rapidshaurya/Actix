use std::process;
use std::fs;
use std::io;
use std::collections::HashMap;
pub struct Todo {
    // use rust built in HashMap to store key - val pairs
    pub map: HashMap<String, HashMap<String, String>>,   //change
}

impl Todo {
    
    pub fn new() -> Result<Todo, std::io::Error> {
        // open db.json
    let f = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .read(true)
        .open("user1.json")?;
    // serialize json as HashMap
    
    match serde_json::from_reader(f) {
        Ok(map) => Ok(Todo { map }),
        Err(e) if e.is_eof() => Ok(Todo {
            map: HashMap::new(),
        }),
        Err(e) => panic!("An error occurred: {}", e),
    }
    }

    pub fn insert(&mut self,username:String, key: String) {
        // insert a new item into our map.
        // we pass true as value.
        let mut map = HashMap::new();
        for (key1, val1) in self.map.iter() {
            if key1.to_string() == username.trim() {
                for (key2, val2) in val1.iter() {
                    map.insert(key2.to_string(), val2.to_string());
                }
                map.insert(key.trim().to_string(), "☐".to_string());
            }
        }
        map.insert(key.trim().to_string(), "☐".to_string());
        self.map.insert(username.trim().to_string(), map );  
    }

    pub fn save(self) -> Result<(), std::io::Error> {
        // open file
    let f = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open("user1.json")?;
    // write to file with serde
    serde_json::to_writer_pretty(f, &self.map)?;
    Ok(())
    }

    pub fn complete(&mut self, username:String, key: &String) -> Option<()> {
        let mut map = HashMap::new();
        let mut f = false;
        let s = Some(());
        for (key1, val1) in self.map.iter() {
            if key1.to_string() == username.trim() {
                for (key2, val2) in val1.iter() {
                    map.insert(key2.to_string(), val2.to_string());
                    if key.trim().to_string() == key2.to_string() {
                        map.insert(key.trim().to_string(), "✅".to_string());
                        f = true;
                    }
                }   
            }
        }
        if f == true{
        self.map.insert(username.trim().to_string(), map);
        }
        return match f {
            true => s,
            false => None,
        }
    }
    pub fn display(self,username: String) ->String{
        let mut ans:String = "".to_string();
        for (key1, val1) in self.map.iter() {
            if key1.to_string() == username.trim() {
                println!("{:#?}", val1);
                ans.push_str(&format!("{:#?}", val1));
            }
        }
        ans
    }
    pub fn delete(&mut self, username: String) {
        self.map.remove_entry(username.trim());
        let _x = fs::remove_file("user1.json");
        println!("{:?}",self.map);
    }
}

pub fn reset() {
    let _x= fs::remove_file("data.txt");
    let _x = fs::remove_file("user1.json");
    println!("Rest Completed");
}
pub fn welcome() -> String{

    println!("Welcome To To-do list");
    println!("(Press 1)\tNew User");
    println!("(Press 2)\tExisting User");
    
    let mut option = String::new();
         io::stdin().read_line(&mut option).expect("Invalid Option");
    let trimmed = option.trim();
    if !(trimmed =="1" || trimmed == "2") {
        println!("Invalid Option");
        process::exit(1);
    }
    return trimmed.to_string();
}