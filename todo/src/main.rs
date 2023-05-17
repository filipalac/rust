/*
Your task is to implement a to-do list application using Rust. The application should allow users to create new tasks, mark tasks as completed, and view a list of all tasks.

Here are the specific requirements for the application:

Users should be able to add new tasks to the to-do list.
Users should be able to mark tasks as completed.
Users should be able to view a list of all tasks, with completed tasks displayed separately from incomplete tasks.
Tasks should be persisted across application launches.
The application should have a CLI interface, with subcommands for adding tasks, marking tasks as completed, and viewing the task list.

*/


use std::env;
use std::process;



fn parse_complete_num(arg: &str) -> u32 {
    let complete: u32 = match arg.parse() {
        Ok(parsed) => parsed,
        Err(_) => panic!("failed to parse number"),
    };

    return complete;
}

fn main() {
    let args: Vec<String> = env::args().collect();


    if args.len() == 2 && &args[1] == "list" {
        println!("Listy DPC");
    } else if args.len() == 3 && &args[1] == "complete" {
        println!("complete {}", &args[2]);

        let complete = parse_complete_num(&args[2]);
        println!("complete num {}", complete);



    } else if  args.len() == 3 && &args[1] == "add" {
        println!("add {}", &args[2]);


    } else {
        println!("Bad input, exiting");
        process::exit(0);
    }





}
