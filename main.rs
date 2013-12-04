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
		1 => {println("Red"); }
		2 => {println("Green"); }
		3 => {println("Blue"); }
		4 => {println("Yellow"); }
		5 => {println("Purple"); }
		6 => {println("Cyan"); }
		7 => {println("White"); }
		_ => {println("There is nothing there..."); }
	}
}



fn main() {
  
	static CMD_PROMPT: &'static str = "Enter your command:";

	let mut WorldMap: [[int, ..100], ..100] = [[0, ..100], ..100];

	let mut PosX: int = 1;
	let mut PosY: int = 1;
  
	let mut object_int: int = 0;
	let mut inventory: ~[int] = ~[];

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
					if (cmd_vec.len() == 0) {println("Please specify the direction you are looking!");}
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
								object_int = WorldMap[PosX + 1][PosY]
							}
							
							print_object(object_int);
						}
					}						
				}
							
				~"inventory" => {
					let mut i = 0;
					let mut counter: int = 0;
					while i < inventory.len() {
						print_object(counter);
						counter += 1;
						i += 1;
					}
				}
				

				~"take" => {
					if (cmd_vec.len() == 0) {println("Please the quadrant you wish to pick a block from!");}
					else {
						let dir = cmd_vec.remove(0);
						if !(allowed_direction(dir.clone())) {
							print("Look direction is invalid!");
						}
						else {
							if (dir == ~"north") {
								object_int = WorldMap[PosX][PosY + 1];
								if (object_int < 8 && object_int > 0) {
									inventory.push(object_int);
									WorldMap[PosX][PosY + 1] = 0;
								}
								else { println("No block at location"); }
							}
                            				if (dir == ~"south") {
								object_int = WorldMap[PosX][PosY - 1];
								if (object_int < 8 && object_int > 0) {
									inventory.push(object_int);
									WorldMap[PosX][PosY - 1] = 0;
								}
								else { println("No block at location"); }
							}
							if (dir == ~"west") {
								object_int = WorldMap[PosX + 1][PosY];
								if (object_int < 8 && object_int > 0) {
									inventory.push(object_int);
									WorldMap[PosX + 1][PosY] = 0;
								}
								else { println("No block at location"); }
							}
							if (dir == ~"east") {
								object_int = WorldMap[PosX - 1][PosY];
								if (object_int < 8 && object_int > 0) {
									inventory.push(object_int);
									WorldMap[PosX - 1][PosY] = 0;
								}
								else { println("No block at location"); }
							}
						}
					}
				}



				~"drop" => {
					
				}

				~"eat" => {
					
				}

				~"mix" => {
					
				}

				_        =>  {println("We don't recognize your command...");}
			}
		}
	}
}
						  
			  
  
  
  



