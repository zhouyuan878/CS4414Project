extern mod extra;


use std::{io, run, os, path, libc, str};


use std::rt::io::*;
use std::io::ReaderUtil;
use std::rt::io::net::ip::{SocketAddr, Ipv4Addr};
use std::io::println;
use std::cell::Cell;
use std::task;

static PORT: int = 4414;
static IPV4_LOOPBACK: &'static str = "127.0.0.1";
static mut visitor_count: uint = 0;

//check if command is allowed
fn allowed_command(cmd: ~str) -> bool {
        let allowed_cmdlist: ~[~str] = ~[~"mix", ~"eat", ~"take", ~"drop", ~"move", ~"look", ~"create", ~"inventory"];
        if allowed_cmdlist.contains(&cmd) {return true;}
        else {return false;}
}

//check if a direction is valid
fn allowed_direction(dir: ~str) -> bool {
        let allowed_dirlist: ~[~str] = ~[~"north", ~"south", ~"west", ~"east"];
        if allowed_dirlist.contains(&dir) {return true;}
        else {return false;}
}

fn checkForBlock(dir: ~str, inVec: [[int, ..100], ..100], x: int, y: int) -> int {
        
        let mut object_int: int = 0;

        if(dir == ~"north") {
                object_int = inVec[x][y + 1];                      
        }
        if(dir == ~"south") {
                object_int = inVec[x][y - 1];
        }
        if(dir == ~"west") {
                object_int = inVec[x - 1][y];
        }
        if(dir == ~"east") {
                object_int = inVec[x + 1][y];
        }
		
		return object_int;
		
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
        let socket = net::tcp::TcpListener::bind(SocketAddr {ip: Ipv4Addr(127,0,0,1), port: PORT as u16});
		
		 println(fmt!("Listening on tcp port %d ...", PORT));
        let mut acceptor = socket.listen().unwrap();
  
        for stream in acceptor.incoming() {
        println!("Saw connection!");
        let stream = Cell::new(stream);
        // Start a task to handle the connection
        do task::spawn {
            let mut stream = stream.take();
            let mut buf = [0, ..500];
            stream.write("welcome\n".as_bytes());
            loop {
                stream.read(buf);
                let request_str = str::from_utf8(buf);
                stream.write("Thanks for telling me about ::".as_bytes());
                stream.write(buf);
            }
        }
       }
       
	   
	   
        static CMD_PROMPT: &'static str = "Enter your command:";

        let mut WorldMap: [[int, ..100], ..100] = [[0, ..100], ..100];
		
		WorldMap[1][1] = 4;
		WorldMap[2][2] = 5;

        let mut PosX: int = 1;
        let mut PosY: int = 1;

        let mut objX: int = 0;  
        let mut objY: int = 0;

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
												
                                                else { print_object(checkForBlock(dir, WorldMap, PosX, PosY)); }
                                        }                                                
                                }
                                                        
                                ~"inventory" => {
                                        let mut i = 0;
                                      
                                        while i < inventory.len() {
                                                print_object(inventory[i]);
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
												      if(dir == ~"north") {
														object_int = WorldMap[PosX][PosY + 1];  
														WorldMap[PosX][PosY + 1] = 0;
														}
													  if(dir == ~"south") {
														object_int = WorldMap[PosX][PosY - 1];
														WorldMap[PosX][PosY - 1] = 0;
													    }
												      if(dir == ~"west") {
														object_int = WorldMap[PosX - 1][PosY];
														WorldMap[PosX - 1][PosY] = 0;
														}
												      if(dir == ~"east") {
														object_int = WorldMap[PosX + 1][PosY];
														WorldMap[PosX + 1][PosY] = 0;
														}
                                                        
                                                        if (object_int < 8 && object_int > 0) {														           
                                                                        inventory.push(object_int);
                                                        }
                                                }
                                        }
										}
                                



                                ~"drop" => {
								   if (inventory.len() == 0) {
								     println("Your inventory is currently empty!");
									}
								   else {
								     let obj_drop: int = inventory.remove(0);
									 
									 if (WorldMap[PosX+1][PosY] == 0) {WorldMap[PosX+1][PosY] = obj_drop;}
									 else if (WorldMap[PosX-1][PosY] == 0) {WorldMap[PosX-1][PosY] = obj_drop;}
									 else if (WorldMap[PosX][PosY+1] == 0) {WorldMap[PosX][PosY+1] = obj_drop;}
									 else if (WorldMap[PosX][PosY-1] == 0) {WorldMap[PosX][PosY-1] = obj_drop;}
								     else {print("There is no place for you to drop your item!");}
                                        
                                }
								}

                                ~"eat" => {
                                        
                                }

                                ~"mix" => {
                                        if (cmd_vec.len() == 0) {println("Please the quadrant you wish to pick a block from!");}
                                        else {
                                                let dir = cmd_vec.remove(0);
                                                if !(allowed_direction(dir.clone())) {
                                                        print("Look direction is invalid!");
                                                }
                                                else {
                                                        let pblock: int = inventory[0];
                                                        let wblock: int = checkForBlock(dir, WorldMap, PosX, PosY);
                                                }
                                        }
                                }

                                _        =>  {println("We don't recognize your command...");}
                        }
                }
        }
}
                                                  
                          
  
  
  

