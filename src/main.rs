use std::io;

struct Todo {
    id: i16,
    title: String,
    completed: bool,
    deleted: bool,
}

fn main() {
    let mut todos: Vec<Todo> = Vec::new();

    loop {
        let mut commands = String::new();
        
        println!("command: ");
        io::stdin()
            .read_line(&mut commands)
            .expect("failed to read input");

        let command_parts: Vec<&str> = commands.split_whitespace().collect();
        
        match command_parts.len() {
            0 => invalid_command(&commands),
            1 => {
                match command_parts[0] {
                    "list" => print_todos(&todos),
                    _ => invalid_command(&commands)
                }
            },
            _ => {
                match command_parts[0] {
                    "add" => add_todo(&mut todos, &command_parts[1..].join(" ")),
                    "remove" => {
                        if let Ok(num) = command_parts[1].parse::<i16>() {
                            remove_todo(&mut todos, num);
                        }
                    },
                    "done" => {
                        if let Ok(num) = command_parts[1].parse::<i16>() {
                            mark_done(&mut todos, num);
                        }
                    },
                    _ => invalid_command(&commands),
                }
            }
        }

        print_todos(&todos);
    }
}

fn mark_done(todos: &mut Vec<Todo>, num: i16) {
    if let Some(todo) = todos.iter_mut().find(|todo| todo.id == num) {
        todo.completed = true;
    }
}

fn remove_todo(todos: &mut Vec<Todo>, num: i16) {
    if let Some(todo) = todos.iter_mut().find(|todo| todo.id == num) {
        todo.deleted = true;
    }
}

fn add_todo(todos: &mut Vec<Todo>, title: &str) {
    let new_id = todos.len() as i16 + 1;

    todos.push(Todo {
        id: new_id,
        title: title.to_string(),
        completed: false,
        deleted: false,
    });
}

fn print_todos(todos: &Vec<Todo>) {
    for todo in todos {
        if !todo.deleted {
            let done = if todo.completed { "✔" } else { " " };
            println!("[{}] {} {}", done, todo.id, todo.title);
        }
    }
}

fn invalid_command(command: &str) {
    println!("invalid command {}", command);
}