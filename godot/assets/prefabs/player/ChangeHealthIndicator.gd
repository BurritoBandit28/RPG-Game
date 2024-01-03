extends RichTextLabel




func _on_player_health_changed(health):
	self.clear()
	var fmt = "%d"
	self.add_text(fmt % [health])
