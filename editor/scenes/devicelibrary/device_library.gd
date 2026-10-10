extends Control

var DeviceItemScene = preload("res://scenes/devicelibrary/device_item.tscn")
var devices: Array[PackedScene] = []

func _ready() -> void:
    for dir in DirAccess.get_directories_at("res://devices"):
        for file_name in DirAccess.get_files_at("res://devices/%s"%dir):
            if file_name.ends_with(".tscn"):
                devices.append(load("res://devices/%s/%s" % [dir, file_name]))

    for i in devices:
        var item: DeviceItem = DeviceItemScene.instantiate()
        item.device_scene = i
        $%GridContainer.add_child(item)
