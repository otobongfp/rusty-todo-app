use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]

pub enum Action{
    ///Add a new task to the todo
    Add {
        ///Add a text here, just letters please
        #[structopt()]
        task : String,
    },
    ///Remove completed task
    Done {
        ///Pick a number to mark a task as done
        #[structopt()]
        position : usize,
    },
    ///list out tasks in a file
    List,
}

#[derive(Debug, StructOpt)]
// #[structopt(
//     name: "ToDo App",
//     about: "Create new tasks and stay motivated"
// )]

pub struct CLArgs{
    #[structopt(subcommand)]
    pub action : Action,
    #[structopt(parse(from_os_str),short,long)]
    pub journal_file : Option<PathBuf>,
}
