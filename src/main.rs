use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};
use std::{
    fs::{self, DirBuilder},
    path,
};
use serde_with::chrono::{prelude::*, Duration};
use serde_with::serde_as;
use strum::{Display, EnumIter, EnumString, IntoEnumIterator};
use std::io;

const TODO_FILE: &str = "~/.rtodo2/todo_file";

#[derive(Debug, Serialize, Deserialize, Copy, Clone, Display, EnumIter, EnumString)]
enum Status {
    Open,
    Done,
    Overdue,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
struct Lifespan {
    #[serde_as(as = "serde_with::DurationSeconds<i64>")]
    amount: Duration,
    unit: LifespanUnit,
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Display, EnumIter, EnumString)]
enum LifespanUnit {
    Day,
    Week,
    Year,
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone, Display, EnumIter, EnumString)]
enum LifeCycle {
    Once,
    Daily,
    Weekly,
    Monthly,
    Yearly,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Todo {
    title: String,
    description: String,
    date: chrono::DateTime<Local>,
    status: Status,
    lifespan: Lifespan,
    lifecycle: LifeCycle,
}

impl Todo {
    pub fn new(
        title: String,
        description: String,
        lifespan: Lifespan,
        lifecycle: LifeCycle,
    ) -> Self {
        Self {
            title,
            description,
            date: Local::now(),
            lifespan,
            lifecycle,
            status: Status::Open,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Todos {
    todos: Vec<Todo>,
    completed_todos: Vec<Todo>,
}

impl Todos {
    fn new() -> Self {
        Self { todos: vec![], completed_todos: vec![]}
    }

    fn load(&mut self) {
        let file_path = shellexpand::full(TODO_FILE).unwrap();
        let path = path::Path::new(file_path.as_ref());
        let prefix = path.parent().unwrap();

        DirBuilder::new().recursive(true).create(prefix).unwrap();
        let mut todo_file = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(file_path.as_ref())
            .expect("Unable to open file");
        let mut data = String::new();
        todo_file
            .read_to_string(&mut data)
            .expect("Unable to read file");
        data = data.trim().to_string();
        if !data.is_empty() {
            let t: Todos = serde_json::from_str(&data).expect(&format!("Unable to parse json {data}"));
            self.todos = t.todos;
            self.completed_todos = t.completed_todos;
        }
    }

    fn write_todo(&self) {

        let file_path = shellexpand::full(TODO_FILE).unwrap();
        let mut todo_file = fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(file_path.as_ref())
            .expect("Unable to open file");
        serde_json::to_writer_pretty(&mut todo_file, &self).expect("Could not write to file");
        todo_file.write_all(b"\n").expect("Could not write to file");
        todo_file.flush().expect("Could not flush file");
    }

    fn get_all(&self) -> &Vec<Todo> {
        &self.todos
    }

    fn add(&mut self, todo: Todo) {
        self.todos.push(todo)
    }

    fn remove(&mut self, index: usize) -> String{
        if index >= self.todos.len() {
            return "Index was the ".to_string();
        }
        _ = self.todos.remove(index);
        return "Removed".to_string();
    }

    fn set_lifespan() {}
}
///Simple todo command line tool
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about=None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// New todo
    #[clap(arg_required_else_help = true)]
    New {
        /// The title for the TODO
        #[clap(required = true)]
        todo_str: String,
    },

    /// List todos
    List,

    /// Edit todo
    #[clap(arg_required_else_help = true)]
    Edit {
        /// Index of the todo for edit
        index: i16,
    },

    /// Remove todo
    #[clap(arg_required_else_help = true)]
    Remove {
        /// Index of the todo
        index: i16,
    },
}


fn main() {
    
    let mut todos: Todos = Todos::new();
    todos.load();
    let args = Cli::parse();

    match args.command {
        Commands::New { todo_str } => {
            println!("Making a new todo: {}", todo_str);
            println!("Enter Discription: ");
            let mut desciption = String::new();
            io::stdin()
                .read_line(&mut desciption)
                .ok()
                .expect("Failed to read line");
            let desciption = desciption.trim();
            println!("Enter Lifespan Unit: ");
            for (i, unit) in LifespanUnit::iter().enumerate() {
                println!("{}: {}", i, unit);
            }
            let mut lifespan = String::new();

            io::stdin()
                .read_line(&mut lifespan)
                .ok()
                .expect("Failed to read line");

            println!("Enter number of {}s: ", lifespan);
            let mut amount = String::new();
            io::stdin()
                .read_line(&mut amount)
                .ok()
                .expect("Failed to read line");

            //Parse and read into the 
            let amount = amount.trim().parse().unwrap();
            let lifespan = match lifespan.to_lowercase().trim() {
                "day" | "0" => Lifespan {
                    amount: Duration::days(amount),
                    unit: LifespanUnit::Day
                },
                "week" | "1" => Lifespan{
                    amount: Duration::weeks(amount),
                    unit: LifespanUnit::Week
                },
                "year" | "3" => Lifespan {
                    amount: Duration::weeks(amount * 52),
                    unit: LifespanUnit::Year
                },
                _ => unreachable!("Bad value"),
            };

            println!("Enter Lifecycle: ");
            for (i, lifecycle) in LifeCycle::iter().enumerate() {
                println!("{}: {}", i, lifecycle);
            }
            let mut lifecycle = String::new();
            io::stdin()
                .read_line(&mut lifecycle)
                .ok()
                .expect("Failed to read line");
            let lifecycle = match lifecycle.to_lowercase().trim() {
                "once" | "0" => LifeCycle::Once,
                "daily" | "1" => LifeCycle::Daily,
                "weekly" | "2" => LifeCycle::Weekly,
                "monthly" | "3" => LifeCycle::Monthly,
                "yearly" | "4" => LifeCycle::Yearly,
                _ => LifeCycle::Once,
            };
            let todo = Todo::new(todo_str, desciption.to_string(), lifespan, lifecycle);
            todos.add(todo);
            println!("Added todo");
            todos.write_todo();
        }
        Commands::List => {
            println!("list todos ya nerd");
            for (i, todo) in todos.todos.iter().enumerate() {
                println!("{}: {}", i, todo.title);
                println!("\tDescription: {}", todo.description);
                println!("\tLifespan: {} {}", todo.lifespan.amount, todo.lifespan.unit);
                println!("\tLifecycle: {}", todo.lifecycle);
                println!("\tTime left: {}", todo.date);
            }
        }
        Commands::Edit { index } => {
            println!("edit {} todo", index)
        }
        Commands::Remove { index } => {
            println!("Remove {} todo", index);
            let msg = todos.remove(index as usize);
            println!("{}", msg);
            todos.write_todo();
        }
    }
}
