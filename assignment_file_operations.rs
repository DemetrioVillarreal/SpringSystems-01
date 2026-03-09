use std::io;


use std::process::Command;



enum FileOperation

 {

    List(String),     // Directory path

    Display(String),     // File path

    Create(String, String),  // File path and content



    Remove(String),  // File path 

    Pwd,  // Print working directory
}

fn perform_operation(operation: FileOperation) {

    match operation {


        FileOperation::List(path) => {


            let status = Command::new("ls")

                .arg(path)

                .status()

                .expect("Failed to execute ls");



            if !status.success() {

                println!("Error listing directory.");


            }
        }

        FileOperation::Display(path) => {

            let status = Command::new("cat")


                .arg(path)

                .status()

                .expect("Failed to execute cat");



            if !status.success() {
                println!("Error displaying file.");
            }
        }

        FileOperation::Create(path, content) => {


            let command = format!("echo '{}' > {}", content, path);

            let status = Command::new("sh")
                .arg("-c")

                .arg(command)


                .status()
                .expect("Failed to create file");


            if status.success() {

                println!("File created successfully.");


            } else {

                println!("Error creating file.");
            }



        }



        FileOperation::Remove(path) => {

            let status = Command::new("rm")

                .arg(path)
                
                .status()
                .expect("Failed to remove file");



            if status.success() {


                println!("File removed successfully.");


            } else {

                println!("Error removing file.");
            }


        }

        FileOperation::Pwd => {


            let status = Command::new("pwd")
                .status()


                .expect("Failed to execute pwd");

            if !status.success() {


                println!("Error printing working directory.");
            }
        }
    }
}




fn read_input() -> String {
    let mut input = String::new();



    io::stdin().read_line(&mut input).expect("Failed to read input");


    input.trim().to_string()
}

fn main() {


    println!("Welcome to the File Operations Program!");

    loop {
        println!("\nFile Operations Menu:");



        println!("1. List files in a directory");



        println!("2. Display file contents");



        println!("3. Create a new file");

        println!("4. Remove a file");


        println!("5. Print working directory");


        println!("0. Exit");



        println!("\nEnter your choice (0-5):");



        let choice = read_input();

        if choice == "0" {


            println!("Goodbye!");



            break;


        }

        else if choice == "1" {


            println!("Enter directory path:");


            let path = read_input();


            let operation = FileOperation::List(path);


            perform_operation(operation);
        }

        else if choice == "2" {


            println!("Enter file path:");


            let path = read_input();

            let operation = FileOperation::Display(path);


            perform_operation(operation);
        }

        else if choice == "3" {
            println!("Enter file path:");


            let path = read_input();


            println!("Enter content:");
            let content = read_input();


            let operation = FileOperation::Create(path, content);


            perform_operation(operation);
        }

        else if choice == "4" {

            println!("Enter file path:");

            let path = read_input();
            let operation = FileOperation::Remove(path);


            perform_operation(operation);
        }

        else if choice == "5" {

            let operation = FileOperation::Pwd;


            perform_operation(operation);
        }

        else {

            println!("Invalid option. Please try again.");
        }


        
    }
}