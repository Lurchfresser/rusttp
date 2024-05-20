use rusttp::overlow_models::Overflow;
use diesel::prelude::*;


pub fn main() {
    use rusttp::establish_connection;
    use rusttp::schema::overflows::dsl::*;

    let connection = &mut establish_connection();

    println!("Please enter the String you want to search for in the overflow body");
    let mut search_str = String::new();
    std::io::stdin().read_line(&mut search_str).unwrap();
    let search_str = search_str.trim_end();

    let results = overflows
        .filter(body.like(format!("%{}%", search_str)))
        .filter(deleted.eq(false))
        .load::<Overflow>(connection)
        .expect("msg");

    println!();
    println!("Found {} overflows", results.len());
    println!("-----------------");

    for overflow in results {
        println!("ID: {}", overflow.id);
        println!("Title: {}", overflow.title);
        println!("Author: {}", overflow.author);
        println!("Body: {}", overflow.body);
        println!("Rating: {}", overflow.rating);
        println!("Overflow ID: {}", overflow.overflow_id);
        println!("Published: {}", overflow.published);
        println!("-----------------");
    }

}