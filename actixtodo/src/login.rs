use std::fs;
use std::process;
pub struct Login {
    pub username: String,
}

impl Login {

    pub fn new() -> Result<Login, std::io::Error> {
        let _f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open("data.txt")?;
        let content = String::new();
       
        Ok(Login { username: content})
    }

    pub fn insert(&mut self, username: String, password: String) {
        // insert a new item into our map.
        // we pass true as value.
        let mut contents = fs::read_to_string("data.txt").expect("file to open user");
         let mut key=1; 
         for _line in contents.lines() {
                key=key+1;
                if _line.contains(&username)  {
                    print!("User already exit\n please user other username\n");
                    process::exit(1);
                }
         }
         let insert = format!("{}\t{}", username.trim(), password.trim());
         if key>1{
            contents.push_str("\n");
         }
         contents.push_str(&insert);
         std::fs::write("data.txt", contents).expect("Unable to write file");
         println!("Data Saved successfully")
    }

    


}
