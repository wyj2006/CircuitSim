class_name Voltage extends Device

@export var value := 0.0

@onready var pos_port = $PosPort
@onready var neg_port = $NegPort

func _ready() -> void:
    current_num = 1

func construct() -> Dictionary:
    return {
        "device_id": device_id,
        "type": "voltage",
        "value": value,
        "pos": pos_port.node_id,
        "neg": neg_port.node_id,
        "current": current_ids[0]
    }

func get_device_name() -> String:
    return "电压源"

func get_texture() -> Texture:
    return $SourceVoltage.texture

func get_attributes() -> Array[Dictionary]:
    return [ {
        "display_name": "电压",
        "attr_name": "value",
        "type": "float"
    }]