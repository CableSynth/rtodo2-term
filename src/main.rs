
use serde::{Deserialize, Serialize};

use std::{cell::Cell, collections::LinkedList};
use clap::Parser;

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

#[derive(Debug, Serialize, Deserialize, Clone, )]
struct Todo {
    title: String,
    description: String,
    status: Cell<Status>,
    lifespan: Lifespan,
    lifecycle: LifeCycle, 
}

impl Todo {
    pub fn new(title: String, description: String, lifespan: Lifespan, lifecycle: LifeCycle) -> Self {
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
    ///Name of a new todo
    name: Option<String>,

    ///List the todos
    list: Option<bool>,
}

fn main() {
    let args = Cli::parse();

    print!("{:?}", args);

}
