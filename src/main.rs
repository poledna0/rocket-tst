#[macro_use] extern crate rocket;
use rocket::{get, post, routes};
use rocket::serde::{json::Json, Serialize, Deserialize};
use rusqlite::{params, Connection as SqliteConnection};

#[derive(Serialize, Deserialize)]
struct Person {
    id: i32,
    name: String,
}

#[post("/add", format = "json", data = "<person>")]
async fn add_person(person: Json<Person>) -> Json<Person> {
    let conn = SqliteConnection::open("people.db").expect("n abriu a merda do bd");

    conn.execute(
        "INSERT INTO people (name) VALUES (?1)",
        params![person.name],
    ).expect("fraco, n consegiu inserir");

    Json(person.into_inner())
}

#[get("/people")]
async fn get_people() -> Json<Vec<Person>> {
    let conn = SqliteConnection::open("people.db").expect("NÃO ABREEEEEEEE");
    let mut stmt = conn.prepare("SELECT id, name FROM people").expect("fraco em buscar tb, catapimbas");

    let people = stmt.query_map([], |row| {
        Ok(Person {
            id: row.get(0)?,
            name: row.get(1)?,
        })
    }).expect("sem query, fraco ").map(|person| person.unwrap()).collect();

    Json(people)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![add_person, get_people])
}
