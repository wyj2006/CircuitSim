class_name DeviceItem extends Control

@onready var editor: Editor = $/root/Editor

var device_scene: PackedScene

func _ready() -> void:
    var device = device_scene.instantiate()
    $%Button.icon = device.texture
    $%Button.text = device.device_name

func _on_button_pressed() -> void:
    editor.hsm.dispatch(&"add_device", device_scene.instantiate())