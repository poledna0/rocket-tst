#[macro_use] extern crate rocket;
use rocket::{get, post, routes};
use rocket::serde::{json::Json, Serialize, Deserialize};
use rusqlite::{params, Connection as SqliteConnection};

#[derive(Serialize, Deserialize)]
struct Person {
    id: i32,
    name: String,
}

// Rota para Criar um Registro
#[post("/add", format = "json", data = "<person>")]
async fn add_person(person: Json<Person>) -> Json<Person> {
    let conn = SqliteConnection::open("people.db").expect("Failed to open database");

    conn.execute(
        "INSERT INTO people (name) VALUES (?1)",
        params![person.name],
    ).expect("Failed to insert person");

    Json(person.into_inner())
}

// Rota para Buscar Todos os Registros
#[get("/people")]
async fn get_people() -> Json<Vec<Person>> {
    let conn = SqliteConnection::open("people.db").expect("Failed to open database");
    let mut stmt = conn.prepare("SELECT id, name FROM people").expect("Failed to prepare statement");

    let people = stmt.query_map([], |row| {
        Ok(Person {
            id: row.get(0)?,
            name: row.get(1)?,
        })
    }).expect("Failed to execute query").map(|person| person.unwrap()).collect();

    Json(people)
}

// Função principal que inicia o servidor
#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![add_person, get_people])
}

