mod login;
use crate::login::Login;
use actixtodo::{reset, Todo};
use std::process;
use std::fs;

use actix_web::{get, App, HttpRequest, HttpServer, Result};

#[get("/newuser/{username}/{password}/")]
async fn newuser(req: HttpRequest) -> Result<String> {
    let username: String = req.match_info().get("username").unwrap().parse().unwrap();
    let password: String = req.match_info().query("password").parse().unwrap();
   
    let mut logres = Login::new().expect("Failed to create new user");
        if password.trim() == "" {
            println!("Invalid Password");
        }
         logres.insert(username.trim().to_string(), password.trim().to_string());
    Ok(format!("Values {} {}", username, password))
}

#[get("/user/add/{username}/{password}/{item}/")]
async fn add(req: HttpRequest) -> Result<String> {
    let username: String = req.match_info().get("username").unwrap().parse().unwrap();
    let password: String = req.match_info().query("password").parse().unwrap();
    let item: String = req.match_info().query("item").parse().unwrap();
    check(&username,&password);
    let mut todo = Todo::new().expect("Initialisation of db failed");
    todo.insert(username,item);
        match todo.save() {
            Ok(_) => Ok(format!("File Saved")),
            Err(why) => Ok(format!("An error occurd {}", why)),
        }
}

#[get("/user/complete/{username}/{password}/{item}/")]
async fn complete(req: HttpRequest) -> Result<String> {
    let username: String = req.match_info().get("username").unwrap().parse().unwrap();
    let password: String = req.match_info().query("password").parse().unwrap();
    let item: String = req.match_info().query("item").parse().unwrap();
    check(&username,&password);
    let mut todo = Todo::new().expect("Initialisation of db failed");

    match todo.complete(username,&item) {
        None => Ok(format!("'{}' is not present in the list", item)),
        Some(_) => match todo.save() {
            Ok(_) => Ok(format!("File Saved....")),
            Err(why) => Ok(format!("An error occurred: {}", why)),
        },
    }
}
#[get("/user/display/{username}/{password}/")]
async fn display(req: HttpRequest) -> Result<String> {
    let username: String = req.match_info().get("username").unwrap().parse().unwrap();
    let password: String = req.match_info().query("password").parse().unwrap();
    check(&username,&password);
    let  todo = Todo::new().expect("Initialisation of db failed");
    
    Ok(format!("{:#?}", todo.display(username)))
}
#[get("/user/resetall/{username}/{password}/")]
async fn resetall(req: HttpRequest) -> Result<String> {
    let username: String = req.match_info().get("username").unwrap().parse().unwrap();
    let password: String = req.match_info().query("password").parse().unwrap();
    check(&username,&password);
    reset();
    Ok(format!("RESET ALL...."))
}

pub fn check(username: &str, password: &str) {
    let check = format!("{}\t{}", username.trim(), password.trim());

        let contents = fs::read_to_string("data.txt").expect("file to open user");
         let mut f=0;
         let mut count=1; 
         for line in contents.lines() {
                if line.contains(&check.trim()) {
                    f=1;
                    break;
                }
                count=count+1;
         }
        if f ==0 {
        println!("Invalid Password");
        process::exit(1);
                 }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new()
    .service(newuser)
    .service(add)
    .service(complete)
    .service(display)
    .service(resetall)
     )
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}


