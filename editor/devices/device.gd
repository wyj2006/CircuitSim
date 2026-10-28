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

var device_id = 0
var current_ids = []
var current_num = 0

func construct():
    return null

func get_device_name() -> String:
    return "Unknown"

func get_texture() -> Texture:
    var sprite = get_node_or_null("Sprite2D")
    if sprite == null:
        return
    return sprite.texture

func get_attributes() -> Array[Dictionary]:
    return []

func get_region() -> Rect2:
    return Rect2($Editable/Area.position, $Editable/Area.size)