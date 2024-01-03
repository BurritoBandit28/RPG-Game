extends UIButton

@export var ind : int; 


func _ready():
	var player : Player = get_parent().get_parent().get_parent().get_parent()
	var select : Sprite2D = get_node("Selected")

	if ind == player.get_selected_sword():
		select.visible = true

func on_button_press():
	var player : Player = get_parent().get_parent().get_parent().get_parent()
	player.set_selected_sword(ind)	
	_ready()
	
func do_shit_idc_anymore():
	var select : Sprite2D = get_node("Selected")
	select.visible = false
	
