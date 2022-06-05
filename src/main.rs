use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::{env, io};
use std::io::Read;
use std::{
    cell::Cell,
    collections::LinkedList,
    fs::{self, DirBuilder},
    path,
};

const TODO_FILE: &str = "~/.rtodo2/todo_file";

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
enum Status {
    Open,
    Done,
    Overdue,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
enum Lifespan {
    Day,
    Week,
    Month,
    Year,
    Life,
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
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
    status: Cell<Status>,
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
            lifespan,
            lifecycle,
            status: Cell::new(Status::Open),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Todos {
    todos: Vec<Todo>,
}

impl Todos {
    fn new() -> Self {
        Self { todos: vec![] }
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
        if data.is_empty() {
            data = String::from("[]")
        }
        self.todos = serde_json::from_str(&data).unwrap();
    }

    fn get_all(&self) -> &Vec<Todo> {
        &self.todos
    }

    fn add(&mut self, todo: Todo) {
        self.todos.push(todo)
    }
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
    // let file_path = shellexpand::full(TODO_FILE).unwrap();
    // let path = path::Path::new(file_path.as_ref());
    // let prefix = path.parent().unwrap();

    // DirBuilder::new().recursive(true).create(prefix).unwrap();
    // let todo_file = fs::OpenOptions::new()
    //     .read(true)
    //     .write(true)
    //     .create(true)
    //     .open(file_path.as_ref());
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
            
            println!("Enter Lifespan: ");
            let mut lifespan = Some(String::new());
            while let Some(l) = &lifespan {
                io::stdin()
                    .read_line(l)
                    .ok()
                    .expect("Failed to read line");
            }
            
            
            
        }
        Commands::List => {
            println!("list todos ya nerd")
        }
        Commands::Edit { index } => {
            println!("edit {} todo", index)
        }
        Commands::Remove { index } => {
            println!("Remove {} todo", index)
        }
    }

}
