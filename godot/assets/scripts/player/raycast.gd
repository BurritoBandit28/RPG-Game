extends RayCast2D

func _on_player_raycast_signal(facing : StringName):
	self.rotation_degrees = facing_to_rot(facing)
	var hit = get_collider()
	if hit != null && hit.get_class() == "NPC":
		var npc : NPC = hit as NPC
		print(npc.name + " the " + npc.character)
	
	pass

func facing_to_rot(facing : StringName) -> float:
	match facing:
		"LEFT":
			return 90.0
		"UP":
			return 180.0
		"DOWN":
			return 0.0
		_:
			return -90.0
