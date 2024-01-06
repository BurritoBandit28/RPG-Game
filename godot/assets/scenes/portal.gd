extends Area2D



func _on_body_entered(body):
	body.save_stats()
	var scene : PackedScene = load("res://assets/scenes/boss.tscn")
	self.get_tree().change_scene_to_packed(scene)
