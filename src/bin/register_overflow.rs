use rusttp::create_overflow_now;
use rusttp::establish_connection;
use std::io::stdin;

pub fn main(){
    //use rusttp::schema::overflows::dsl::*;
    
    let mut connection = establish_connection();


    println!("Please enter the title of the overflow");
    let mut title = String::new();
    stdin().read_line(&mut title).unwrap();
    let title = title.trim_end();

    println!("Please enter the author of the overflow");
    let mut author = String::new();
    stdin().read_line(&mut author).unwrap();
    let author = author.trim_end();

    println!("Please enter the body of the overflow");
    let mut body = String::new();
    stdin().read_line(&mut body).unwrap();
    let body = body.trim_end();

    println!("Please enter the overflow_id of the overflow");
    let mut overflow_id_str = String::new();
    stdin().read_line(&mut overflow_id_str).unwrap();
    let overflow_id_str = overflow_id_str.trim_end();
    let overflow_id = overflow_id_str.parse::<i32>().unwrap();

    println!("Please enter the rating of the overflow");
    let mut rating_str = String::new();
    stdin().read_line(&mut rating_str).unwrap();
    let rating_str = rating_str.trim_end();
    let rating = rating_str.parse::<i32>().unwrap();

    create_overflow_now(&mut connection, title, body, overflow_id, author, rating);

}