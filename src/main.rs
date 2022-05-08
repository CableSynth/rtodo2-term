use serde::{Deserialize, Serialize};

use clap::{Parser, Subcommand};
use std::env;
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
        /// The text for the TODO
        #[clap(required = true)]
        todo_str: String,
    },

    /// List todos
    #[clap(arg_required_else_help = true)]
    List,

    /// Edit todo
    #[clap(arg_required_else_help = true)]
    Edit {
        /// Index of the
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
    let path_name = shellexpand::full(TODO_FILE).unwrap();
    let path = path::Path::new(path_name.as_ref());
    let prefix = path.parent().unwrap();

    DirBuilder::new().recursive(true).create(prefix).unwrap();

    // let args = Cli::parse();

    print!("{:?}", prefix);
}
