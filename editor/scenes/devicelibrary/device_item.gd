class_name DeviceItem extends Control

@onready var editor: Editor = $/root/Editor

var device_scene: PackedScene
var placing = false

func _ready() -> void:
    var device = device_scene.instantiate()
    $%Button.icon = device.texture
    $%Button.text = device.device_name

func _on_button_pressed() -> void:
    if editor.state != Editor.State.Idle:
        return
    if placing:
        return
    placing = true

    editor.state = Editor.State.AddDevice
    editor.device_to_add = device_scene.instantiate()
    editor.get_node("Devices").add_child(editor.device_to_add)

func _input(event: InputEvent) -> void:
    if editor.state != Editor.State.AddDevice:
        return
    if Input.is_action_just_pressed("cancel") and placing:
        editor.state = Editor.State.Idle
        editor.device_to_add.get_parent().remove_child(editor.device_to_add)
        editor.device_to_add = null
        placing = false
    if event is InputEventMouseButton and event.is_pressed() and event.button_index == MOUSE_BUTTON_LEFT:
        editor.state = Editor.State.Idle
        placing = false