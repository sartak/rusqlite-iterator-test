use rusqlite::{Connection, Result};

struct Person {
    id: i32,
    name: String,
}

struct PersonIterator<'a> {
    results: Box<dyn Iterator<Item = Result<Person>> + 'a>,
}

impl<'a> Iterator for PersonIterator<'a> {
    type Item = Result<Person>;

    fn next(&mut self) -> Option<Self::Item> {
        self.results.next()
    }
}

fn get_people<'a>(conn: &'a Connection) -> Result<PersonIterator<'a>> {
    let mut stmt = conn.prepare("SELECT rowid, name FROM people")?;
    let results = stmt.query_map([], |row| {
        Ok(Person {
            id: row.get(0)?,
            name: row.get(1)?,
        })
    })?;

    Ok(PersonIterator {
        results: Box::new(results),
    })
}

fn main() {
    let conn = Connection::open("test.sqlite").unwrap();
    let people = get_people(&conn).unwrap();

    for person in people {
        let person = person.unwrap();
        println!("person {}: {}", person.id, person.name);
    }
}
