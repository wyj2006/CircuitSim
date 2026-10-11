class_name DeviceItem extends Control

@onready var editor: Editor = $/root/Editor

var device_scene: PackedScene

func _ready() -> void:
    var device = device_scene.instantiate()
    $%Button.icon = device.texture
    $%Button.text = device.device_name

func _on_button_pressed() -> void:
    if editor.state != Editor.State.Idle:
        return

    editor.state = Editor.State.AddDevice
    editor.device_to_add = device_scene.instantiate()
    editor.get_node("Devices").add_child(editor.device_to_add)
