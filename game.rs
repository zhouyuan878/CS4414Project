//mod database;

use std::*;

//check if command is allowed
fn allowed_command(cmd: ~str) -> bool {
	let allowed_cmdlist: ~[~str] = ~[~"mix", ~"eat", ~"take", ~"drop", ~"move", ~"look", ~"create"];
        if allowed_cmdlist.contains(&cmd) {println("ya"); return true;}
        else {return false;}
}

//check if command has the ocrrect number of parameters; to be modified
fn check_command(command: ~[~str]) {
    
	if (command[0] == ~"eat") ||  (command[0] == ~"take") || (command[0] == ~"drop") || (command[0] == ~"look") || (command[0] == ~"create") {
        	if (command.len() > 1) {
        		print("No parameter should be used for your command!");
        		return;
                }
        }

    	else if (command[0] == ~"mix") ||  (command[0] == ~"move") {
        	if (command.len() == 1) {
        		print("Use a parameter for your command!");
        		return;
        	}
        }
}
  
//need to be modified to handle commands with different parameters
fn handle_command(cmd: ~[~str]) {
	let command: ~str = cmd[0].clone(); 
        if !allowed_command(command){
		return;
	}
        
        else {
        	check_command(cmd);
		//database::cmd;
        }
}

fn main() {
        static CMD_PROMPT: &'static str = "Enter your command:";
        
        loop {        
                print(CMD_PROMPT);
                let command = io::stdin().read_line();
		let cmd: ~[~str] = command.split_iter(' ').filter_map(|x| if x != "" { Some(x.to_owned()) } else { None }).to_owned_vec();  
                handle_command(cmd);
        }
}
