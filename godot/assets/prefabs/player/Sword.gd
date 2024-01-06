extends UIButton

@export var ind : int; 


func _ready():
	var player : LimboPlayerStats = get_node("/root/GlobalLimboPlayerStats")
	var select : Sprite2D = get_node("Selected")
	
	print(player.get_selected_sword())
	
	if ind == player.get_selected_sword():
		select.visible = true

func on_button_press():
	var player : LimboPlayerStats = get_node("/root/GlobalLimboPlayerStats")
	player.set_selected_sword(ind)	
	_ready()
	
func do_shit_idc_anymore():
	var select : Sprite2D = get_node("Selected")
	select.visible = false
	
