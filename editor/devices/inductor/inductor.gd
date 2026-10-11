class_name Inductor extends Device

@export var value := 0.0

func construct() -> Dictionary:
    return {
        "type": "inductor",
        "value": value,
        "pos": $PosPort.node_id,
        "neg": $NegPort.node_id
    }

func get_device_name() -> String:
    return "电感"

func get_texture() -> Texture:
    return $Inductor.texture

func get_attributes() -> Array[Dictionary]:
    return [ {
        "display_name": "感值",
        "attr_name": "value",
        "type": "float",
    }]