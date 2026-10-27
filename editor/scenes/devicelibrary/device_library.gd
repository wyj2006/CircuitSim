extends Control

@onready var editor: Editor = $/root/Editor

var devices: Array[PackedScene] = []

func _ready() -> void:
    for dir in DirAccess.get_directories_at("res://devices"):
        for file_name in DirAccess.get_files_at("res://devices/%s"%dir):
            if file_name.ends_with(".tscn"):
                devices.append(load("res://devices/%s/%s" % [dir, file_name]))

    for i in devices:
        var device: Device = i.instantiate()
        $%ItemList.add_item(device.get_device_name(), device.get_texture())

func _on_item_list_item_activated(index: int) -> void:
    editor.hsm.dispatch(&"add_device", devices[index].instantiate())
