use rusqlite::{Connection, Result};

struct Person {
    id: i32,
    name: String,
}

fn get_people(conn: Connection) -> Result<Box<dyn Iterator<Item = Result<Person>>>> {
    let mut stmt = conn.prepare("SELECT rowid, name FROM people")?;
    let results = stmt.query_map([], |row| {
        Ok(Person {
            id: row.get(0)?,
            name: row.get(1)?,
        })
    })?;

    Ok(Box::new(results))
}

fn main() {
    let conn = Connection::open("test.sqlite").unwrap();
    let people = get_people(conn).unwrap();

    for person in people {
        let person = person.unwrap();
        println!("person {}: {}", person.id, person.name);
    }
}
