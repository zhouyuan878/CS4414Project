extern mod extra;


use std::{io, run, os, path, libc, str};


use std::rt::io::*;
use std::io::ReaderUtil;
use std::rand;
use std::rand::Rng;
use std::rt::io::net::ip::{SocketAddr, Ipv4Addr};
use std::io::println;
use std::cell::Cell;
use std::task;
use extra::arc;
use extra::arc::RWArc;

static PORT: int = 4414;
static IPV4_LOOPBACK: &'static str = "127.0.0.1";
static mut visitor_count: uint = 0;

//check if command is allowed
fn allowed_command(cmd: ~str) -> bool {
        let allowed_cmdlist: ~[~str] = ~[~"mix", ~"eat", ~"pick", ~"drop", ~"move", ~"look", ~"inventory",~"help"];
        if allowed_cmdlist.contains(&cmd) {return true;}
        else {return false;}
}

//check if a direction is valid
fn allowed_direction(dir: ~str) -> bool {
        let allowed_dirlist: ~[~str] = ~[~"north", ~"south", ~"west", ~"east"];
        if allowed_dirlist.contains(&dir) {return true;}
        else {return false;}
}

//print out the object at a specific coordinate
fn print_object(input: int) -> ~str{
        match input {
                1 => {return ~"Red block is spotted!"; }
                2 => {return ~"Green block is spotted!"; }
                3 => {return ~"Blue block is spotted!"; }
                4 => {return ~"Yellow block is spotted!";}
                5 => {return ~"Purple block is spotted!"; }
                6 => {return ~"Cyan block is spotted!"; }
                7 => {return ~"White block is spotted!"; }
		10 => {return ~"A player is spotted!";}
                _ => {return ~"There is nothing there...";}
        }
}

fn get_object(input: int) -> ~str{
        match input {
                1 => {return ~"Red"; }
                2 => {return ~"Green"; }
                3 => {return ~"Blue"; }
                4 => {return ~"Yellow";}
                5 => {return ~"Purple"; }
                6 => {return ~"Cyan"; }
                7 => {return ~"White"; }
                _ => {return ~"";}
        }
}


fn main() {
        let socket = net::tcp::TcpListener::bind(SocketAddr {ip: Ipv4Addr(127,0,0,1), port: PORT as u16});
		
		println(fmt!("Game server is up and running. "));
		println(fmt!("To start the game, establish a connnection to port 4414 on localhost!"));
        let mut acceptor = socket.listen().unwrap();
  
        let mut sharedMap: [[int, ..100], ..100] = [[0, ..100], ..100];
		let sMap = arc::RWArc::new(sharedMap);
		
  
        for stream in acceptor.incoming() {
        println!("Welcome to the world of mystery!");
        let stream = Cell::new(stream);
        // Start a task to handle the connection
		
		let WorldMap = sMap.clone();
		
        do task::spawn {

			
        let mut PosX: int = 5;
        let mut PosY: int = 5;

        let mut object_int: int = 0;
        let mut inventory: ~[int] = ~[];
		
		let mut counter: int = 0;

        let mut stream = stream.take();
        let mut buf = [0, ..500];
		stream.write("Hello! Welcome to the World of Mystery!\n".as_bytes());
		           
            loop {		
				stream.write("\n".as_bytes());
                stream.write("Enter your command with a space at the end:\n".as_bytes());			
                stream.read(buf);
                let mut cmd_line = str::from_utf8(buf);
				let mut cmd_vec: ~[~str] = cmd_line.split_iter(' ').filter_map(|x| if x != "" { Some(x.to_owned()) } else { None }).to_owned_vec();
				let main_command = cmd_vec.remove(0);				
                buf = [0, ..500];				
				
				if (counter == 1) {
					let mut rng = rand::task_rng();
                			let mut randomX: int = rng.gen_integer_range(0, 100);
					let mut randomY: int = rng.gen_integer_range(0, 100);
					let mut randomColor: int = rng.gen_integer_range(1, 8);	
    
                    do WorldMap.write |arg| {	
					
					while (!(arg[randomX][randomY] == 0)) {
	                	randomX = rng.gen_integer_range(0, 100);
						randomY = rng.gen_integer_range(0, 100);
						randomColor = rng.gen_integer_range(1, 8);	
                     }		
					
                    arg[randomX][randomY] = randomColor; 
				    
					}
					counter = 0;
					
					
					stream.write("A ".as_bytes());
					let colorstring: ~str = get_object(randomColor);
					stream.write(colorstring.as_bytes());
					let string: ~str = fmt!(" block has been generated at: <%?,%?>\n", randomX, randomY);
					stream.write(string.as_bytes());	
			//	    stream.write(" \n".as_byte());
					}
				
				if !(allowed_command(main_command.clone())) {
                        stream.write("The command you entered is invalid! Use help to view the list of commands available!\n".as_bytes());
                }		
				
				else {
					match main_command {
								~"help" => {
								stream.write("The commands available are: move, look, inventory, pick, drop, eat and mix.\n".as_bytes());
								}
                                ~"move" => {
										counter += 1;			
                                        if (cmd_vec.len() == 0) {stream.write("Please specify the direction you are moving!\n".as_bytes());}                                       
                                        else {
                                                let dir = cmd_vec.remove(0);                                                
                                                if !(allowed_direction(dir.clone())) {
                                                        stream.write("Move direction is invalid!\n".as_bytes());
                                                }                                               
                                                else {												
														do WorldMap.write |arg| {
														arg[PosX][PosY] = 0;
														}
														
                                                        if (dir == ~"north") {PosY += 1;}
                                                        if (dir == ~"south") {PosY -= 1;}
                                                        if (dir == ~"west")  {PosX -= 1;}
                                                        if (dir == ~"east")  {PosX += 1;}
     
	                                                    do WorldMap.write |arg| {
                                                        if (arg[PosX][PosY] == 0) {
                                                                arg[PosX][PosY] = 10;
																let cstring: ~str = fmt!("Your current coordinate is: <%?,%?>\n", PosX, PosY);
                                                                stream.write(cstring.as_bytes());
                                                        }     
                                                        else {
                                                                stream.write("The position is already occupied\n".as_bytes());
																if (dir == ~"north") {PosY -= 1;}
																if (dir == ~"south") {PosY += 1;}
																if (dir == ~"west")  {PosX += 1;}
																if (dir == ~"east")  {PosX -= 1;}
																
																arg[PosX][PosY] = 10;
                                                             }
															 }                                                
                                                }
                                        }                                                 
                                }
                                                  
                                ~"look" => {			
                                        if (cmd_vec.len() == 0) {stream.write("Please specify the direction you are looking!\n".as_bytes());}
                                        else {
                                                let dir = cmd_vec.remove(0);
                                                if !(allowed_direction(dir.clone())) {
                                                        stream.write("Look direction is invalid!\n".as_bytes());
                                                }																								
                                                else { 
												    let mut pstring: ~str = ~"";
													do WorldMap.write |arg| {
													match dir {													
													  ~"north" => {pstring = print_object(arg[PosX][PosY+1]);}
													  ~"south" => {pstring = print_object(arg[PosX][PosY-1]);}
													  ~"west" => {pstring = print_object(arg[PosX-1][PosY]);}
													  ~"east" => {pstring = print_object(arg[PosX+1][PosY]);}
													  _ => {}
													}			
											     stream.write(pstring.as_bytes());
												 stream.write("\n".as_bytes());															
												}
                                          } 
									}										  
                                }
                                                        
                                ~"inventory" => {			
                                        let mut i = 0;     
                                        if (inventory.len() == 0) {
 										   stream.write("Your inventory is empty!\n".as_bytes());
										   }
										else {
										stream.write("The blocks in your inventory are:\n".as_bytes());
                                        while i < inventory.len() {
										        let tstring: ~str = get_object(inventory[i]);
                                                stream.write(tstring.as_bytes());
												stream.write(" ".as_bytes());
                                                i += 1;
                                        }
										stream.write("\n".as_bytes());
										}
                                }
                                
                                ~"pick" => {
										counter += 1;			
                                        if (cmd_vec.len() == 0) {stream.write("Please specify the direction of the block to be picked!\n".as_bytes());}
                                        else {
                                                let dir = cmd_vec.remove(0);
                                                if !(allowed_direction(dir.clone())) {
                                                        stream.write("Take direction is invalid!\n".as_bytes());
                                                }
                                                else {
												      if(dir == ~"north") {
													    do WorldMap.write |arg| {
														object_int = arg[PosX][PosY + 1];  
														arg[PosX][PosY + 1] = 0;
														}
														}
													  if(dir == ~"south") {
													    do WorldMap.write |arg| {
														object_int = arg[PosX][PosY - 1];
														arg[PosX][PosY - 1] = 0;
														}
													    }
												      if(dir == ~"west") {
														do WorldMap.write |arg| {
														object_int = arg[PosX - 1][PosY];
														arg[PosX - 1][PosY] = 0;
														}
														}
												      if(dir == ~"east") {
														do WorldMap.write |arg| {
														object_int = arg[PosX + 1][PosY];
														arg[PosX + 1][PosY] = 0;
														}
														}
                                                        
                                                        if (object_int < 8 && object_int > 0) {														           
                                                                        inventory.push(object_int);			
																					
																		stream.write("A ".as_bytes());
																		let blockstring: ~str = get_object(object_int);
																		stream.write(blockstring.as_bytes());													
																		let bstring: ~str = fmt!(" block has been added to your inventory.\n");
																		stream.write(bstring.as_bytes());
                                                        }
                                                }
                                        }
									}
                                
                                ~"drop" => {			
								   if (inventory.len() == 0) {
								     stream.write("Your inventory is currently empty!\n".as_bytes());
									}
								   else {
								     let obj_drop: int = inventory.remove(0);
									 stream.write("You've dropped a \n".as_bytes());
									 let dropstring: ~str = get_object(obj_drop);
									 stream.write(dropstring.as_bytes());
									 stream.write(" block.\n".as_bytes());
									 
									 do WorldMap.write |arg| {
									 if (arg[PosX+1][PosY] == 0) {arg[PosX+1][PosY] = obj_drop;}
									 else if (arg[PosX-1][PosY] == 0) {arg[PosX-1][PosY] = obj_drop;}
									 else if (arg[PosX][PosY+1] == 0) {arg[PosX][PosY+1] = obj_drop;}
									 else if (arg[PosX][PosY-1] == 0) {arg[PosX][PosY-1] = obj_drop;}
								     else {stream.write("There is no place for you to drop your item!\n".as_bytes());}
									 }                                      
                                }
								}

                                ~"eat" => {												
									let pblock: int = inventory[0];									
									if (pblock == 0) {
										stream.write("No block available in your inventory!\n".as_bytes());
									}
				
									else {
										if (pblock == 1) {
											inventory[0] = 0;
											stream.write("Red block consumed.\n".as_bytes());
										}		
										if (pblock == 2) {
											inventory[0] = 0;	
											stream.write("Green block consumed.\n".as_bytes());										
										}	
										if (pblock == 3) {
											inventory[0] = 0;	
											stream.write("Blue block consumed.\n".as_bytes());										
										}	
										if (pblock == 4) {
											inventory[0] = 0;
											stream.write("Yellow block consumed.\n".as_bytes());											
										}	
										if (pblock == 5) {
											inventory[0] = 0;
											stream.write("Purple block consumed.\n".as_bytes());											
										}	
										if (pblock == 6) {
											inventory[0] = 0;	
											stream.write("Cyan block consumed.\n".as_bytes());										
										}	
										if (pblock == 7) {
											inventory[0] = 0;
											stream.write("White block consumed.\n".as_bytes());											
										}						
									}
                                }

                                ~"mix" => {
										counter += 1;			
                                        if (cmd_vec.len() == 0) {stream.write("Please specify the direction of the block you wish to mix with!\n".as_bytes());}
                                        else {
                                                let dir = cmd_vec.remove(0);
                                                if !(allowed_direction(dir.clone())) {
                                                        stream.write("No block in specified direction!\n".as_bytes());
                                                }
                                                else {                                                       														
														do WorldMap.write |arg| {
														
														let mut pblock: int = inventory[0];
                                                        let mut wblock: int = 0;
														
														if (dir == ~"north") {
															wblock = arg[PosX][PosY+1];
														}
														if (dir == ~"south") {
															wblock = arg[PosX][PosY-1];
														}
														if (dir == ~"east") {
															wblock = arg[PosX+1][PosY];
														}
														if (dir == ~"west") {
															wblock = arg[PosX-1][PosY];
														}														
														
														if (pblock == 1 && wblock == 1) {
															pblock = 1;
														}
														else if (pblock == 2 && wblock == 2) {
															pblock = 2;
														}
														else if (pblock == 3 && wblock == 3) {
															pblock = 3;
														}
														else if ((pblock == 1 && wblock == 2) || (pblock == 2 && wblock == 1)) {
															pblock = 4;
														}
														else if ((pblock == 1 && wblock == 3) || (pblock == 3 && wblock == 1)) {
															pblock = 5;
														}
														else if ((pblock == 3 && wblock == 2) || (pblock == 2 && wblock == 3)) {
															pblock = 6;
														}
														else if ((pblock == 4 && wblock == 5) || (pblock == 5 && wblock == 4)) {
															pblock = 7;
														}
														else if ((pblock == 4 && wblock == 6) || (pblock == 6 && wblock == 4)) {
															pblock = 7;
														}
														else if ((pblock == 6 && wblock == 5) || (pblock == 5 && wblock == 6)) {
															pblock = 7;
														}
														else if (pblock == 7 || wblock == 7) {
															pblock = 7;
														}
														else if (pblock == 6 || wblock == 6) {
															pblock = 6;
														}
														else if (pblock == 5 || wblock == 5) {
															pblock = 5;
														}
														else if (pblock == 4 || wblock == 4) {
															pblock = 4;
														}

														inventory[0] = pblock;
														
														if (dir == ~"north") {
															arg[PosX][PosY + 1] = 0;
														}
														if (dir == ~"south") {
															arg[PosX][PosY - 1] = 0;
														}
														if (dir == ~"west") {
															arg[PosX - 1][PosY] = 0;
														}
														if (dir == ~"east") {
															arg[PosX + 1][PosY] = 0;
														}
                                                }
												}
                                        }
                                }

                                _        =>  {
								      stream.write("We don't recognize your command...\n".as_bytes());
									  }
                        }
						}
							
            }
        }
       }
	   }
       
	   

                
  
                
  
               

                                                  
                          
  
  
  
