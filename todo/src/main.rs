use std::env;
use std::process;
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::Write;
use std::io::Read;
//use std::io::BufReader;
//use std::error::Error;



#[derive(Serialize, Deserialize, Debug)]
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


fn print_list(tasks: &Vec<Task>) {
    for task in tasks {
        println!("[{}] {}",if task.completed {"x"} else {""},  task.task);
    }
}


fn write_list_to_file(tasks: &Vec<Task>) {
        let serialized = serde_json::to_string(&tasks).unwrap();
        let mut file = File::create("tasks.json").unwrap();
        file.write_all(serialized.as_bytes()).unwrap();
}


fn main() {
    let args: Vec<String> = env::args().collect();

    let mut tasks: Vec<Task> = Vec::new();


    /*
    push_task(&mut tasks, "ASDASDASDASD");
    push_task(&mut tasks, "ASDASDASDASD");
    mark_complete(&mut tasks, 1);
    print_list(&tasks);
    write_list_to_file(&tasks);
    */


    if args.len() == 2 && &args[1] == "list" {
        print_list(&tasks);
    } else if args.len() == 3 && &args[1] == "complete" {
        let complete = parse_complete_num(&args[2]);
        mark_complete(&mut tasks, complete);
    } else if  args.len() == 3 && &args[1] == "add" {
        push_task(&mut tasks, &args[2]);
    } else {
        process::exit(0);
    }

    //write_list_to_file(&tasks);
}
