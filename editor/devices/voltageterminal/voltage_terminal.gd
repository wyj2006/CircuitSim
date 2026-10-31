class_name VoltageTermial extends Device

@export var value_expr = "0"

@onready var port = $Port
@onready var net_list: NetList = $/root/Editor/NetList
@onready var expr_label: Label = $Label

func _ready() -> void:
    current_num = 1

func _process(_delta: float) -> void:
    expr_label.text = value_expr
    expr_label.rotation = - rotation

    var degree = expr_label.rotation_degrees
    while degree < 0:
        degree += 360
    while degree > 360:
        degree -= 360

    var d0 = absf(degree)
    var d90 = absf(degree - 90)
    var d180 = absf(degree - 180)
    var d270 = absf(degree - 270)

    var m = min(min(d0, d90), min(d180, d270))
    if m == d0:
        expr_label.horizontal_alignment = HORIZONTAL_ALIGNMENT_LEFT
        expr_label.vertical_alignment = VERTICAL_ALIGNMENT_CENTER
    elif m == d90:
        expr_label.horizontal_alignment = HORIZONTAL_ALIGNMENT_CENTER
        expr_label.vertical_alignment = VERTICAL_ALIGNMENT_BOTTOM
    elif m == d180:
        expr_label.horizontal_alignment = HORIZONTAL_ALIGNMENT_RIGHT
        expr_label.vertical_alignment = VERTICAL_ALIGNMENT_CENTER
    elif m == d270:
        expr_label.horizontal_alignment = HORIZONTAL_ALIGNMENT_CENTER
        expr_label.vertical_alignment = VERTICAL_ALIGNMENT_TOP

func construct():
    return {
        "device_id": device_id,
        "type": "voltage_source",
        "value_expr": value_expr,
        "pos": port.node_id,
        "neg": net_list.REF_NODE_ID,
        "current": current_ids[0]
    }

func get_device_name() -> String:
    return "电压端子"

func get_attributes() -> Array[Dictionary]:
    return [ {
        "display_name": "电压表达式",
        "attr_name": "value_expr",
        "type": "expr"
    }]