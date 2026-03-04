use std::io;
use std::process::Command;

enum FileOperation {
    List(String),            // Directory path
    Display(String),         // File path
    Create(String, String),  // File path and content
    Remove(String),          // File path
    Pwd,                     // Print working directory
}

fn perform




  else if choice == "4" {
            println!("Enter file path:");
            let path = read_input();
            let operation = FileOperation::Remove(path);
            perform_operation(operation);
        }
