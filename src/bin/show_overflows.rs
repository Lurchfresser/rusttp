use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use rusttp::overlow_models::Overflow;

pub fn main() {
    use rusttp::establish_connection;
    use rusttp::schema::overflows::dsl::*;

    let connection = &mut establish_connection();

    println!("Please enter the amount of overflows you want to see");
    let mut amount_str = String::new();
    std::io::stdin().read_line(&mut amount_str).unwrap();
    let amount_str = amount_str.trim_end();
    let amount = amount_str.parse::<i64>().unwrap();    

    let results: Vec<Overflow> = overflows
        .order_by(published.desc())
        .limit(amount)
        .load(connection)
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