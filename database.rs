//Need: world-tracking rgb, player db (connection info, rgb, attributes, position), map db (size, capacity, active positions), block info db (makeup, eat, destroy, take, drop), command db (function library for all commands)


use std::*

mut worldRGB: [int] = [0, 0, 0];

mut currentPosX: int;
mut currentPosY: int;

struct object {
   c_block: block,
   c_player: player
}

mut WorldMap: [[int, ..100], ..100];

struct map {
   field: [[int, ..200], ..200]
  // size: int,//it may be simpler to just fold this whole thing into Rust's native vector type, let it figure out when to get bigger
   //capacity: float
}

impl map {
   fn new -> map {
      
   }
   
  
   
   fn checkForBlock(xPos,yPos) -> block {
      if (map.field[xPos,yPos] != null) {
         Object tmp = map.coordinates[xPos,yPos];
         match tmp {
            Block(tmp) => return tmp;
            
         }
      }
   }
}

struct block {
   mut source: [int] = [0,0,0],
   mut key: int
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
     
     tmpBlock = 
     }
    }
}

struct player {
   mut health: int,
   mut hunger: int,
   mut attack: int,
   mut defense: int,
   mut hand: block,
   mut xPos: int,
   mut yPos: int,
   mut xDir: int,
   mut yDir: int

   //mut invSize: int,
   //mut inv: blockInfo[]//not sure if this is correct syntax for declaring a variable size array- or if that's possible without simply destroying the old one and replacing it.
}

impl player {
   
   fn new -> player {
      health: 1,
      attack: 1,
      defense: 1,
      hunger: 1,
      xPos: 0,
      yPos: 0,
      xDir: 0,
      yDir: 0
   }
   
   fn eat(self) {
      match self.block.color {
         ~'r' => {self.attack += 1}
         ~'b' => {self.defense += 1}
         ~'g' => {self.health += 1}
         _    => {println("Fingers aren't tasty")}
      }
      block.color = 'n';
   }

   fn mix(self, second: block) {
      newblock: block = mix_block(self.block, second);
      self.block.color = newblock.color;
      self.block.xPos = newblock.xPos;
      self.block.yPos = newblock.yPos;
      }

   fn take(self) {
      if (self.block.color != 'n') { println("Don't get greedy"); }
      else {
         self.block = checkForBlock(xPos+xDir,yPos+yDir);
      }
   }
   
   fn drop(self) {
      if (self.block.color == 'n') { println("Don't get greedy"); }
      else {
         emptyBlock = checkForBlock(xPos+xDir,yPos+yDir);
         if (emptyBlock.color == 'n') {
            //clear block in hand, add block to world
         }
      }
   }
   
   fn move(self, direction: str) {
     WorldMap[currentPosX][currentPosY] = 0;
     
     if (direction == north) {currentPosY += 1;}
     if (direction == south) {currentPosY -= 1;}
     if (direction == west) {currentPosX -= 1;}
     if (direction == east) {currentPosX += 1;}
     
     if (WorldMap[currentPosX][currentPosY] == 0) {
     WorldMap[currentPosX][currentPosY] = 10;
     }
     
}

}
   
/*fn addInventory(~player: player, ~block: block) {

}

fn readCommand(args: string[], target) {

}*/
