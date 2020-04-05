use rusqlite::{params, Connection, Result};
use std::io;
#[macro_use]
extern crate prettytable;
use prettytable::{Cell, Row, Table};

#[derive(Debug)]
struct Todo {
    id: i16,
    title: String,
    completed: bool,
    deleted: bool,
}

fn main() -> Result<()> {
    let mut todos: Vec<Todo> = Vec::new();
    let conn = Connection::open("db.db")?;
    load_data(&conn, &mut todos)?;

    loop {
        let mut commands = String::new();

        println!("command: ");
        io::stdin()
            .read_line(&mut commands)
            .expect("failed to read input");

        let command_parts: Vec<&str> = commands.split_whitespace().collect();

        match command_parts.len() {
            0 => invalid_command(&commands),
            1 => match command_parts[0] {
                "list" => print_todos(&todos),
                _ => invalid_command(&commands),
            },
            _ => match command_parts[0] {
                "add" => add_todo(&conn, &mut todos, &command_parts[1..].join(" "))?,
                "remove" => {
                    if let Ok(num) = command_parts[1].parse::<i16>() {
                        remove_todo(&conn, &mut todos, num)?;
                    }
                }
                "done" => {
                    if let Ok(num) = command_parts[1].parse::<i16>() {
                        mark_done(&conn, &mut todos, num)?;
                    }
                }
                _ => invalid_command(&commands),
            },
        }
    }
}

fn load_data(conn: &Connection, todos: &mut Vec<Todo>) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS todos (
        id INTEGER PRIMARY KEY,
        title TEXT NOT NULL,
        completed BOOLEAN,
        deleted BOOLEAN
    )",
        params![],
    )?;

    let mut data = conn.prepare("SELECT * from todos where deleted = false")?;

    let todos_iter = data.query_map(params![], |row| {
        Ok(Todo {
            id: row.get(0)?,
            title: row.get(1)?,
            completed: row.get(2)?,
            deleted: row.get(3)?,
        })
    })?;

    for todo in todos_iter {
        todos.push(todo.unwrap());
    }
    Ok(())
}

fn mark_done(conn: &Connection, todos: &mut Vec<Todo>, num: i16) -> Result<()> {
    if let Some(todo) = todos.iter_mut().find(|todo| todo.id == num) {
        todo.completed = true;

        conn.execute(
            "update todos set completed = true where id = ?1",
            params![num],
        )?;
    }

    print_todos(&todos);
    Ok(())
}

fn remove_todo(conn: &Connection, todos: &mut Vec<Todo>, num: i16) -> Result<()> {
    if let Some(todo) = todos.iter_mut().find(|todo| todo.id == num) {
        todo.deleted = true;

        conn.execute(
            "update todos set deleted = true where id = ?1",
            params![num],
        )?;
    }
    print_todos(&todos);
    Ok(())
}

fn add_todo(conn: &Connection, todos: &mut Vec<Todo>, title: &str) -> Result<()> {
    let new_id = todos.len() as i16 + 1;

    todos.push(Todo {
        id: new_id,
        title: title.to_string(),
        completed: false,
        deleted: false,
    });

    conn.execute(
        "INSERT INTO TODOS
            (id, title, completed, deleted)
            values (?1, ?2, ?3, ?4)",
        params![new_id, title.to_string(), false, false],
    )?;

    print_todos(&todos);
    Ok(())
}

fn print_todos(todos: &Vec<Todo>) {
    let mut table = Table::new();

    table.add_row(row!["[ ]", "ID", "TITLE"]);

    for todo in todos {
        if !todo.deleted {
            let done = if todo.completed { "✔" } else { " " };
            table.add_row(Row::new(vec![
                Cell::new(done),
                Cell::new(&todo.id.to_string()),
                Cell::new(&todo.title.to_string()),
            ]));
        }
    }

    table.printstd();
}

fn invalid_command(command: &str) {
    println!("invalid command {}", command);
}
