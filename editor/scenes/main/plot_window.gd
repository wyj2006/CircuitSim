extends Window

@onready var editor: Editor = $/root/Editor

func _process(_delta: float) -> void:
    if editor.state != Editor.State.Simulate:
        for child in get_children():
            remove_child(child)

func _on_close_requested() -> void:
    pass
