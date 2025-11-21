# Untitled Survival Shooter
by Team Blueberry

## Team Members
* Advanced Topic Subteam 1: Multiplayer Networking
	* gcw33: Gordon Wyrostek
	* ifo2: Ifemi Olojo-Kosoko
	* zij15: Peter Ju

* Advanced Topic Subteam 2: Lighting Systems
	* mrk161: Matthew Kindja
	* haw102: Hao Wang
	* dva8: Daniel Aleardi
	* vlb56: Vladyslav Bordia

## Game Description

Our untitled survival shooter is a cooperative survival game in which
players progress through levels, fending off enemies and gathering resources necessary to survival.
Players must locate batteries to keep their lights on, or face certain death in darkness.

## Advanced Topic Description

### Multiplayer Networking

Our multiplayer networking implementation will synchronize the actions of up to 4 players across the network.
One client player will also assume the role of the host. Clients will transmit actions to the server, which
will resolve conflicts, simulate the world, and send state updates.

### Lighting Systems

There will be two components to our lighting system: phong lighting and shadow masks.
* For each on-screen sprite, our phong lighting algorithm will sample orientation data (stored in a normal map) to determine the amount of light reflected for each pixel. As such, each sprite include a corresponding, pre-computed normal map.
* Crucial to our shadow mask system is the implementation of a signed-distance-field, from which information is pulled to compute screen-space shadow map information.

## Midterm Goals

* Player 4-direction movement from a top-down perspective
* Player orients toward cursor
* Player shooting
* Enemies die when attacked
* Enemy type that moves in straight line toward player
* Players die when colliding with enemies
* Phong lighting implementation
* Point and cone lights illuminate enviornment within radius
* Server and client connection

## Final Goals

* 10%: 100x100 tile level (Each tile is 64x64 pixels) with 3 distinct buildings that players will venture through toward their goal. Entire level will be composed of 20 handmade tile assets that make up walls, doors, and other props, where at least 10 of which are lit (include normal information).
* 02%: Batteries will be situated throughout the level, and are collected when a player collides with the item. Playing without a flashlight will cause a sanity value to slowly decrease, killing the player when depleted. Also, a player with a depleted flashlight won't be able to see enemies coming through the darkness.
* 01%: Revive kits will be situated throughout the level, and are collected during collision. Up to 1 can be held at a time, indicated by a UI element. Revive kits are used at the body of a dead player to restore their life.
* 01%: Ammunition can be collected, and the current amount is represented by a UI element. Available ammunition is reduced when attacking.
* 02%: Three enemy types will be present: Balanced, Fast and weak (Comes in groups of up to 5), slow and strong (Rare encounter). At each of the 3 buildings, there will be 2 - 5 instances.
* 01%: Win state: all players reach the end of the level
* 01%: Lose state: all players are simultaneously dead
* 08%: Signed distance field implemented
* 12%: Screen-space shadow maps implemented
* 08%: Point and cone lights illuminate enviornment within radius
* 02%: Normal maps computed for all lit sprites
* 08% Phong lighting computed per-pixel for all lit sprites on screen
* 10%: Player input data recording and server validation (lag compensation)
* 12%: Server-client interface: Players will communicate their position and rotation to the server, which will distribute this information to all clients other than the sender. Similarly, player interactions, like firing are sent. Enemy position and movement are calculated by the client, while the server designates which player the enemies are targetting. Bullets are deterministic after being created at the position of the player.

## Stretch Goals

* Add a G-buffer, which will enable a more efficient deferred rendering approach
* Add an enemy that pathfinds toward the players
