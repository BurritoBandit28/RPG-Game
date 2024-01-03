extends RichTextLabel

var count : int;
@export
var ind : int;



func _ready():
	var player : Player = get_parent().get_parent().get_parent().get_parent().get_parent().get_parent()
	count = player.get_item_count(ind)
	self.clear()
	var fmt = "x%d"
	self.add_text(fmt % [count])

func _process(_delta):
	_ready()
