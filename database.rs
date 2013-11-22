//Need: world-tracking rgb, player db (connection info, rgb, attributes, position), map db (size, capacity, active positions), block info db (makeup, eat, destroy, take, drop), command db (function library for all commands)


use std::*

static mut worldRGB: int[] = [0, 0, 0];

struct object {
   c_block: block,
   c_player: player
}

struct map {
   mut field: bool[bool[]]
   mut ~coordinates: object[object[]]
   size: int,//it may be simpler to just fold this whole thing into Rust's native vector type, let it figure out when to get bigger
   capacity: float
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
   color: char,
   mut xPos: int,
   mut yPos: int
   //need to represent eat, destroy, etc. as functions, blockInfo needs to be able to get them
}

impl block {
   fn new(new_color: str)-> block {
     block {
       color: new_color,
       xPos: 0,
       yPos: 0,
      }
    }

   fn mix_block(first: block, second: block) -> block {
     if (first.color == "red") && (second.color == "green") {return new("yellow");}
     else if (first.color == "red") && (second.color == "blue") {return new("magenta");}
     else if (first.color == "green") && (second.color == "blue") {return new("cyan");}
     else if (first.color == "yellow") && (second.color == "magenta") {return new("white");}
     else if (first.color == "yellow") && (second.color == "cyan") {return new("white");}
     else if (first.color == "magenta") && (second.color == "cyan") {return new("white");}
     else if (first.color == "white") {return new(second.color);}
     else if (second.color == "white") {return new(first.color);}
     else {return first}
     }
    }
}

struct player {
   mut health: int,
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
}
   
/*fn addInventory(~player: player, ~block: block) {

}

fn readCommand(args: string[], target) {

}*/
