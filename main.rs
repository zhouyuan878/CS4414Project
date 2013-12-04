use std::{io, run, os, path, libc};


//check if command is allowed
fn allowed_command(cmd: ~str) -> bool {
        let allowed_cmdlist: ~[~str] = ~[~"mix", ~"eat", ~"take", ~"drop", ~"move", ~"look", ~"create"];
        if allowed_cmdlist.contains(&cmd) {return true;}
        else {return false;}
}

//check if a direction is valid
fn allowed_direction(dir: ~str) -> bool {
        let allowed_dirlist: ~[~str] = ~[~"north", ~"south", ~"west", ~"east"];
        if allowed_dirlist.contains(&dir) {return true;}
        else {return false;}
}

//get and print the object at a specific coordinate
fn print_object(input: int) {
	match input {
		1 => {println("Gold spotted!");}
		2 => {println(" ");}
		_ => {println("There is nothing there...");}
	}
}



fn main() {
  
	static CMD_PROMPT: &'static str = "Enter your command:";

	let mut WorldMap: [[int, ..100], ..100] = [[0, ..100], ..100];

	let mut PosX: int = 1;
	let mut PosY: int = 1;
  
	let mut object_int: int = 0;

	println("Welcome to the world of mystery!");
        
        
	loop {        
		println(CMD_PROMPT);

		let mut cmd_line = io::stdin().read_line();
		let mut cmd_vec: ~[~str] = cmd_line.split_iter(' ').filter_map(|x| if x != "" { Some(x.to_owned()) } else { None }).to_owned_vec();
  
		let main_command = cmd_vec.remove(0);
  
		if !(allowed_command(main_command.clone())) {
			println("The command you entered is invalid! Use help to view the list of commands available!");
		}
 
		else {
			match main_command {
				~"exit" => {return;}
				~"move" => {
					if (cmd_vec.len() == 0) {println("Please specify the direction you are moving!");}
					
					else {
						let dir = cmd_vec.remove(0);
						
						if !(allowed_direction(dir.clone())) {
							println("Move direction is invalid!");
						}
						
						else {
							WorldMap[PosX][PosY] = 0;
							 
							if (dir == ~"north") {PosY += 1;}
                            				if (dir == ~"south") {PosY -= 1;}
							if (dir == ~"west")  {PosX -= 1;}
							if (dir == ~"east")  {PosX += 1;}
     
							if (WorldMap[PosX][PosY] == 0) {
								WorldMap[PosX][PosY] = 10;
								println(fmt!("Your current coordinate is: <%?,%?>", PosX, PosY));
							}
     
							else {
								println("the position is already occupied");
						     	}
						   
						}
					}
						  
				}
						  
				~"look" => {
					if (cmd_vec.len() == 0) {println("Please specify the direction you are moving!");}
					else {
						let dir = cmd_vec.remove(0);
						if !(allowed_direction(dir.clone())) {
							print("Look direction is invalid!");
						}
						else {
							if (dir == ~"north") {
								object_int = WorldMap[PosX][PosY + 1];
							}
                            				if (dir == ~"south") {
								object_int = WorldMap[PosX][PosY - 1]
							}
							if (dir == ~"west") {
								object_int = WorldMap[PosX - 1][PosY]
							}
							if (dir == ~"east") {
								object_int = WorldMap[PosX + 1][PosY + 1]
							}
							
							print_object(object_int);
						}
					}
				}
							
						  
				_        =>  {println("We don't recognize your command...");}
			}
		}
	}
}
						  
			  
  
  
  




