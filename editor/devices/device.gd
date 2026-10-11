class_name Device extends Node2D

var device_name:
    get:
        return get_device_name()

var texture:
    get:
        return get_texture()

var attributes:
    get:
        return get_attributes()

var region:
    get:
        return get_region()

var current_ids = []
var current_num = 0

func construct() -> Dictionary:
    return {}

func get_device_name() -> String:
    return "Unknown"

func get_texture() -> Texture:
    return null

func get_attributes() -> Array[Dictionary]:
    return []

func get_region() -> Rect2:
    return Rect2($Editable/Area.position, $Editable/Area.size)