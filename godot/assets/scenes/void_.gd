extends Area2D


var createVoid = false;
@export var colorect : ColorRect;

func _on_body_entered(body):
	createVoid = true
	
func _process(delta):
	if createVoid && colorect.color.b > 0:
		var prev = colorect.color
		var new = Color(prev.r - 0.01, prev.g - 0.01, prev.b - 0.01)
		colorect.color = new
		print(new)
