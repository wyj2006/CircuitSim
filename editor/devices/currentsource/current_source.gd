class_name CurrentSource extends Device

@export var value_expr = "0"

@onready var pos_port = $PosPort
@onready var neg_port = $NegPort

func construct() -> Dictionary:
    return {
        "device_id": device_id,
        "type": "current_source",
        "value_expr": value_expr,
        "pos": pos_port.node_id,
        "neg": neg_port.node_id,
    }

func get_device_name() -> String:
    return "电流源"

func get_attributes() -> Array[Dictionary]:
    return [ {
        "display_name": "电流表达式",
        "attr_name": "value_expr",
        "type": "expr"
    }]