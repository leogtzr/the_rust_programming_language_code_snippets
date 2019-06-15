use std::collections::HashMap;
use std::io;

enum CommandKind {
    Add,
    Show,
    Unknown
}

fn check_command(cmd: &str) -> bool {
    match cmd {
        "Add" | "Show" => true,
        _ => false
    }
}

fn string_to_command_kind(cmd: &str) -> CommandKind {
    match cmd {
        "Add" => CommandKind::Add,
        "Show" => CommandKind::Show,
        _ => CommandKind::Unknown
    }
}

fn validate_add_cmd(cmd_tokens: &Vec<&str>) -> bool {
    if cmd_tokens.len() != 4 {
        return false;
    }
    if cmd_tokens[2] != "to" {
        return false;
    }
    true
}

fn validate_show_cmd(cmd_tokens: &Vec<&str>) -> bool {
    if cmd_tokens.len() != 2 {
        return false;
    }
    true
}

fn add_to_department(departments: &mut HashMap<String, Vec<String>>, depto: &str, people: &str) {
    let d = departments.entry(depto.to_owned()).or_insert(Vec::new());
    d.push(people.to_owned());
}

fn show_department(departments: &HashMap<String, Vec<String>>, depto: &str) {
    match departments.get(depto) {
        Some(d) => {
            println!("{:?}", d);
        },
        None => {
            println!("Department '{}' not found.", depto);
        }
    }
}

fn main() {
    
    let mut department: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        print!("$ ");
        let mut cmd = String::new();

        io::stdin()
            .read_line(&mut cmd)
            .expect("Failed to read line");
        
        if cmd.is_empty() {
            continue;
        }

        let cmd_tokens = cmd.split_whitespace().collect::<Vec<&str>>();
        println!("{:?}", cmd_tokens);
        let cmd = cmd_tokens[0];
        if !check_command(cmd) {
            println!("error: wrong command '{}'", cmd);
            continue;
        }

        let command_kind = string_to_command_kind(cmd);
        match command_kind {
            CommandKind::Add => {
                if !validate_add_cmd(&cmd_tokens) {
                    println!("wrong add command ... ");
                    continue;
                }
                add_to_department(&mut department, cmd_tokens[3], cmd_tokens[1]);
            },
            CommandKind::Show => {
                if !validate_show_cmd(&cmd_tokens) {
                    println!("wrong show command ... ");
                    continue;
                }
                show_department(&department, cmd_tokens[1]);
            },
            _ => {
                println!("error: wrong command '{}'", cmd);
                continue;
            }
        }
    }

}
