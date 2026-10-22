extends Window

@onready var editor: Editor = $/root/Editor

func _on_close_requested() -> void:
    hide()

func _on_plotter_button_pressed() -> void:
    show()
