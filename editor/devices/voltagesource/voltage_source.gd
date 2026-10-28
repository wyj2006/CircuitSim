class_name VoltageSource extends Device

@export var value_expr = "0"

@onready var pos_port = $PosPort
@onready var neg_port = $NegPort

func _ready() -> void:
    current_num = 1

func construct():
    return {
        "device_id": device_id,
        "type": "voltage_source",
        "value_expr": value_expr,
        "pos": pos_port.node_id,
        "neg": neg_port.node_id,
        "current": current_ids[0]
    }

func get_device_name() -> String:
    return "电压源"

func get_attributes() -> Array[Dictionary]:
    return [ {
        "display_name": "电压表达式",
        "attr_name": "value_expr",
        "type": "expr"
    }]