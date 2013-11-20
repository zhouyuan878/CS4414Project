//Need: world-tracking rgb, player db (connection info, rgb, attributes, position), map db (size, capacity, active positions), block info db (makeup, eat, destroy, take, drop), command db (function library for all commands)


use std::*

static mut worldRGB: int[] = [0, 0, 0];

struct map {
   mut field: bool[bool[]]
   size: int,//it may be simpler to just fold this whole thing into Rust's native vector type, let it figure out when to get bigger
   capacity: float
}

struct blockInfo {
   makeup: int[red, green, blue],
   //need to represent eat, destroy, etc. as functions, blockInfo needs to be able to get them
}

struct playerRGB {
   mut red: int,
   mut green: int,
   mut blue: int,
   mut health: int,
   mut attack: int,
   mut defense: int,
   mut invSize: int,
   mut inv: blockInfo[]//not sure if this is correct syntax for declaring a variable size array- or if that's possible without simply destroying the old one and replacing it.
}

fn addInventory(~player: playerRGB, ~block: blockInfo) {

}

fn readCommand(args: string[], target) {
   
}
