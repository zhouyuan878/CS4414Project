

mod database;



//check if command is allowed
fn allowed_command(cmd: ~str) -> bool {
        let allowed_cmdlist: ~[~str] = ~[~"mix", ~"eat", ~"take", ~"drop", ~"move", ~"look", ~"create"];
        if allowed_cmdlist.contains(&cmd) {return true;}
        else {return false;}
}

//check if command has the ocrrect number of parameters; to be modified
fn check_command(command: ~[~str]) {
    let cmd: ~str = command[0];
    
    if (cmd == ~"eat") ||  (cmd == ~"take") || (cmd == ~"drop") || (cmd == ~"look") || (cmd == ~"create") {
        if (command[1] != "") 
        {print("No parameter should be used for your command!");
        return;
                      }
        }
    
    else if (cmd == ~"mix") ||  (cmd == ~"move") {
        if (command[1] == "") 
        {print("Use a parameter for your command!");
        return;
        }
      }
    }
  
//need to be modified to handle commands with different parameters
fn handle_command(cmd: ~[~str]) {
        let command: ~str = cmd[0]; 
        if !allowed_command(command){
           //(redundant) print("command not allowed!");
           return;
}
       else {
         check_command(cmd);
         database::cmd;
         }
  }
  
           



fn main() {

        static CMD_PROMPT: &'static str = "Enter your command:";
        
        
        loop {        
                print(CMD_PROMPT);
                let command = io::stdin().read_line();
  
                handle_command(command);
        }
}
  
  
 
 
  
