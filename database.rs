//Need: world-tracking rgb, player db (connection info, rgb, attributes, position), map db (size, capacity, active positions), block info db (makeup, eat, destroy, take, drop), command db (function library for all commands)


use std::*;

//mut currentPosX: int;
//mut currentPosY: int;

struct object {
   c_block: block,
   c_player: player
}

struct WorldData {
   field: [[int]],
   rgb: [int],
   players: [player]
}

impl WorldData {
   fn new() -> WorldData {
      field = [[0]];//how do we declare all positions equal 0?
      rgb = [0, 0, 0];
      players = [0];//again, how do we fill this with 0's?
   }
   
  
   
   fn checkForBlock(self, xPos: int, yPos: int) -> block {
      if (self.field[xPos][yPos] != null) {
         let tmp: object = map.coordinates[xPos][yPos];//this won't be an object, it will be an int
         match tmp {
            block(tmp) => {return tmp;}
            
         }
      }
   }
   
   fn generate(self) {//call each time the counter increments- eventually refine to speed, narrow search?
      let mut xPos: int = 0;
      let mut yPos: int = 0;
      if (self.rgb[0] == 10) {
 	 while (self.field[xPos][yPos] != 0) {
	    xPos = Rand();
	    yPos = Rand();
	 }
	 self.field[xPos][yPos] = 1;
      } else if (self.rgb[1] == 10) {
         while (self.field[xPos][yPos] != 0) {
	    xPos = Rand();
	    yPos = Rand();
	 }
	 self.field[xPos][yPos] = 2;
      } else if (self.rgb[2] == 10) {
         while (self.field[xPos][yPos] != 0) {
	    xPos = Rand();
	    yPos = Rand();
	 }
	 WorldData.field[xPos][yPos] = 3;
      } else {break}
   }
}

struct block {
   source: [int],
   key: int, 
   name: ~str
}

impl block {
   fn new(new_color: str)-> block {
      if (new_color == "new") {
      }
   
      if (new_color == "red") {
         source[0] = 10;
         key = 1;
      }
      if (new_color == "green") {
         source[1] = 10;
         key = 2;
      }
      if (new_color == "blue") {
         source[2] = 10;
         key = 3;
      }
      if (new_color == "yellow") {
         source[0] = 10;
         source[1] = 10;
         key = 4;
      }
      if (new_color == "purple") {
         source[0] = 10;
         source[2] = 10;
         key = 5;
      }
      if (new_color == "cyan") {
         source[1] = 10;
         source[2] = 10;
         key = 6;
      }
      if (new_color == "white") {
         source[0] = 10;
         source[1] = 10;
         source[2] = 10;
         key = 7;
      }
   }

   fn mix_block(first: block, second: block) -> block {
     /*if (first.color == "red") && (second.color == "green") {return new("yellow");}
     else if (first.color == "red") && (second.color == "blue") {return new("magenta");}
     else if (first.color == "green") && (second.color == "blue") {return new("cyan");}
     else if (first.color == "yellow") && (second.color == "magenta") {return new("white");}
     else if (first.color == "yellow") && (second.color == "cyan") {return new("white");}
     else if (first.color == "magenta") && (second.color == "cyan") {return new("white");}
     else if (first.color == "white") {return new(second.color);}
     else if (second.color == "white") {return new(first.color);}
     else {return first}*/
     
     //tmpBlock = 
   }
   
   fn checkName(name: ~str) -> int {
      if (self.name == "red") { return 1 }
      else if (self.name == "blue") { return 2 }
      else if (self.name == "green") { return 3 }
      else if (self.name == "yellow") { return 4 }
      else if (self.name == "cyan") { return 5 }
      else if (self.name == "purple") { return 6 }
      else if (self.name == "white") { return 7 }
      else { return 0 }
   }
}

struct player {
   id: int,
   health: int,
   hunger: int,
   attack: int,
   defense: int,
   hand: block,
   xPos: int,
   yPos: int,
   xDir: int,
   yDir: int,
   effects: [int]

   //mut invSize: int,
   //mut inv: [int, ...10]//not sure if this is correct syntax for declaring a variable size array- or if that's possible without simply destroying the old one and replacing it.
}

impl player {
   
   fn new(World: WorldData) -> player {
      id = {while (World.players[id] != 0) {id += 1}}; //or something along those lines
      health = 1;
      attack = 1;
      defense = 1;
      hunger = 1;
      xPos = 0;
      yPos = 0;
      xDir = 0;
      yDir = 0;
      effects = [0];
   }
   
   fn eat(self) {
      let target = checkForBlock((self.xPos+self.xDir), (self.yPos+self.yDir));
      match  target.key {
         ~1 => {self.attack += 1}
         ~2 => {self.defense += 1}
         ~3 => {self.health += 1}
	 ~4 => {self.attack += 1
		}//need to implement timer to round this out
         _    => {println("Fingers aren't tasty")}
      }
      block.color = 'n';
   }

   fn mix(self, first: block, second: block) {
      let newblock: block = mix_block(first, second);
      let i = 0;
      while (self.inventory[i] != 0) {
         i += 1;
      }
      self.inventory[i] = block.key;
   }

   fn take(self) {
      let target = checkForBlock((self.xPos+self.xDir), (self.yPos+self.yDir);
      if (self.inventory[9] != 0) {println("Don't get greedy!");}
      else {
         while (self.inventory[i] != 0) {
         i += 1;
         }
      self.inventory[i] = block.key;
      }
   }
   
   fn throw(self, block: ~str) {
      let mut i = 0;
      let mut thisKey = checkName(block);
      let mut possible: bool = false;
      let mut invPos: int = 0;
      while (i < 10) {
         if (self.inventory[i] == thisKey) {
            possible = true;
    	    invPos = i;
         }
      }
      if (possible == false) { println("Throw what? Dumbass."); }
      else {
         self.inventory[invPos] = 0;
    	 //match for effects goes here
      }
   }
   
   fn move(self, direction: str) {
     let tempX = xPos;
     let tempY = yPos;
     
     if (direction == "north") {self.yPos += 1;}
     if (direction == "south") {self.yPos -= 1;}
     if (direction == "west") {self.yPos -= 1;}
     if (direction == "east") {self.yPos += 1;}
     
     if (World[self.xPos][self.yPos] == 0) {
        World[self.xPos][self.yPos] = self.id;
     }
     
     else {
        self.xPos = tempX;
        self.yPos = tempY;
        print("the position is already occupied");
     }
   }
   
   fn look(self, direction: str) {
     
   }
}
   
/*fn addInventory(~player: player, ~block: block) {

}

fn readCommand(args: string[], target) {

}*/
