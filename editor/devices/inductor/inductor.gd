class_name Inductor extends Device

@export var value := 0.0

func _ready() -> void:
    current_num = 1

func construct():
    return {
        "device_id": device_id,
        "type": "inductor",
        "value": value,
        "pos": $PosPort.node_id,
        "neg": $NegPort.node_id,
        "current": current_ids[0]
    }

func get_device_name() -> String:
    return "电感"

func get_attributes() -> Array[Dictionary]:
    return [ {
        "display_name": "感值",
        "attr_name": "value",
        "type": "float",
    }]