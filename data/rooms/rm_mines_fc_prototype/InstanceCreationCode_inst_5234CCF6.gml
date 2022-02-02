on_activate = function() {
	//Make a lil light!
	instance_create_depth(x, y - OBJECT_CELL_SIZE, depth, obj_light_small);	
	
	//I want to spawn a chest 2 cells below me (these are 8x8 coords, not 16x16)
	var xx = xstart div OBJECT_CELL_SIZE,		// This will get my x grid coord
	var yy = (ystart div OBJECT_CELL_SIZE) + 2	// And my y grid coord + 2 spaces BELOW me
	//GM's y axis starts at 0 up the top of the room and INCREASES as we go down!
	
	//NOTE: I use "xstart" and "ystart" above instead of "x" and "y" current position
	//as all objects correct for their origin/offset after spawning
	//but xstart and ystart will give me where I put this object in the room editor!
	
	//SpawnObject makes a object at a 8x8 grid location and returns the instance it makes
	SpawnObject(CURRENT_BLOB, ObjectId.ChestBiome1, xx, yy);
	
	//Then we spawn a little poof, but we need pixel coords; our current location will do!
	create_animation_effect(x, y, depth, spr_fx_poof1_neutral_once);
}											