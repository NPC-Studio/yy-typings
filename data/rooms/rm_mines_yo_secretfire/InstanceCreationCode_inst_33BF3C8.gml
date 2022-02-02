on_activate = function() {
	instance_create_depth(x, y - OBJECT_CELL_SIZE, depth, obj_light_big);
	
	// Check if Everyone else is activated to unlock the ladder!
	var _success = true;
	with(obj_dungeon_interactable) {
	    if(id != other.id && state == MinesInteractState.Deactivated) {
	        _success = false;
	        break;
	    }
	}

	if(_success) {
	  with(obj_dungeon_ladder_down) {
	    if(state != MinesLadderState.Spawned) {
	      self.try_activate();
	    }
	  }
	}
}

