extends Control



func _on_play_button_pressed() -> void:
	get_tree().change_scene_to_file("res://scenes/lobby.tscn")

func _on_exit_button_pressed() -> void:
	get_tree().quit()
