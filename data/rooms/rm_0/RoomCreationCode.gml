if(!instance_exists(obj_chaos_loop)) {
	//madness!!
	instance_create_depth(0, 0, 0, obj_chaos_loop);
} else {
	trace("Finished chaos loop; all rooms Tiles_Collision and Tiles_Rules are saved. Starting game.");
	instance_create_depth(0, 0, 0, Setup);
	var _bundle = obj_chaos_loop.chaos_bundle;
	var _names = variable_struct_get_names(_bundle);
	var _len = array_length(_names);
	for(var i = 0; i < _len; i++) {
		variable_struct_set(Setup, _names[i], _bundle[$ _names[i]]);
		array_push(global.janus_checklist, _names[i]);
	}
	instance_destroy(obj_chaos_loop);
	if MIST_BOOT {
		if (!variable_struct_exists(Setup.meta_serde.get_manifest(), "last_played")) {
			CRASH("Cannot mist boot if you don't have a save file!");	
		}
		Setup.meta_serde.set_game_save_to_continue();
		Setup.handoff_to_game(false);
		Game.taxi.eject();
		Game.cutscene_manager.hot_load(global.mist_boot_path);
		
	// Unit test?
	// (not using cli params because its not possible to pass them to the runner)
	} else if environment_get_variable("FOM_UNIT_TEST") == "1" {
		run_unit_test_boot();	
	} else {
		room_goto(rm_menu);
	}
}