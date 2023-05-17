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

struct Task {
    task: String,
    completed: bool,
}



fn parse_complete_num(arg: &str) -> usize {
    let complete: usize = match arg.parse() {
        Ok(parsed) => parsed,
        Err(_) => panic!("failed to parse number"),
    };

    return complete;
}

fn push_task(tasks: &mut Vec<Task>, task: &str) {
    let new_task = Task {
        task: String::from(task),
        completed: false,
    };

    tasks.push(new_task);
}

fn mark_complete(tasks: &mut Vec<Task>, position: usize) {
    if let Some(task) = tasks.get_mut(position) {
        task.completed = true;
    } else {
        println!("Task at position {} does not exist.", position);
    }
}

fn print_list(tasks: &mut Vec<Task>) {
    for task in tasks {
        println!("[{}] {}",if task.completed {"x"} else {""},  task.task);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut tasks: Vec<Task> = Vec::new();

   //push_task(&mut tasks, "ASDASDASDASD");
   //push_task(&mut tasks, "ASDASDASDASD");
    //mark_complete(&mut tasks, 1);
   //print_list(&mut tasks);

    if args.len() == 2 && &args[1] == "list" {
        println!("Listy DPC");
    } else if args.len() == 3 && &args[1] == "complete" {

        let complete = parse_complete_num(&args[2]);

        mark_complete(&mut tasks, complete);



    } else if  args.len() == 3 && &args[1] == "add" {
        let add = &args[2];
        println!("add {}", add);
        push_task(&mut tasks, &args[2]);


    } else {
        println!("Bad input, exiting");
        process::exit(0);
    }





}
