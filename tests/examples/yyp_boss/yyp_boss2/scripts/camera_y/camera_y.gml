// Script assets have changed for v2.3.0 see
// https://help.yoyogames.com/hc/en-us/articles/360005277377 for more information
function camera_y(){
	var _cam = argument[0];
	if (argument_count == 2) {
		camera_set_view_pos(_cam,camera_x(_cam),argument[1]);
	}
	return camera_get_view_y(_cam);
}