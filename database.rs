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

struct block {
   color: char,
   mut xPos: int,
   mut yPos: int
   //need to represent eat, destroy, etc. as functions, blockInfo needs to be able to get them
}

struct player {
   mut red: int,
   mut green: int,
   mut blue: int,
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
      red: 1,
      green: 1,
      blue: 1,
      health: 1,
      attack: 1,
      defense: 1,
      xPos: 0,
      yPos: 0,
   }
   
   fn eat(~player: player) {
      match player.block.color {
         ~'r' => {player.attack += 1}
         ~'b' => {player.defense += 1}
         ~'g' => {player.health += 1}
         _    => {println("Fingers aren't tasty")}
      }
      block.color = 'n';
   }

   fn take(~player: player) {
      if (player.block.color != 'n') { println("Don't get greedy"); }
      else {
         player.block = checkForBlock(xPos+xDir,yPos+yDir);
      }
   }
}

/*fn addInventory(~player: player, ~block: block) {

}

fn readCommand(args: string[], target) {

}*/
