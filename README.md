CS4414-Project 
=============
Goal: Develop a game engine and a small example game as a demo of the game engine. 



Deliverables: A text-based adventure game developed in Rust that supports multiplayers in real time. Multiple players can connect to the game server and play the game concurrently. The game world is a two-dimensional map where players could interact with different objects in the game world through a set of commands. The game world dynamically updates itself by generating more objects(blocks) as players' interaction with the game world increases. 
Blocks of different colors including Red, Green, Blue, Yellow, Purple, Cyan and White can be generated in the game 
world, which can be consumed by players and produce distinctive effects. Two blocks can also be mixed to produce a new block.



How to play the game: Compile and run the game.rs file and the game server will be started. Use command 
"nc localhost 4414" to establish a connection to the game server which starts the game. Multiple players can 
connect to the game server and play the game at the same time. 




Commands available:

help: Display the set of commands the player could use.

move: Move to a specified coordinate on the map. Takes a direction parameter of "north", "south", "west" or "east".

look: Check the object(block or player) at a specified coordinate on the map if there is any. Takes a direction parameter of "north", "south", "west" or "east".

pick: Pick the block at a specified coordinate on the map if there is any. Takes a direction parameter of "north", "south", "west" or "east".

inventory: Check the blocks available in the player's inventory. Takes no parameter.

drop: Remove a block from the player's inventory and drop it on one of the nearest coordinate on the map. Takes no parameter.

eat: Consume a block in the player's inventory. Takes no parameter. 

mix: Mix a nearby block with a block in the player's inventory. Takes a direction parameter of "north", "south", "west" or "east".



