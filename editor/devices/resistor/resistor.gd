class_name Resistor extends Device

@export var value := 0.0

func construct() -> Dictionary:
    return {
        "type": "resistor",
        "value": value,
        "pos": $PosPort.node_id,
        "neg": $NegPort.node_id
    }

func get_device_name() -> String:
    return "电阻"

func get_texture() -> Texture:
    return $Resistor.texture

func get_attributes() -> Array[Dictionary]:
    return [ {
        "display_name": "阻值",
        "attr_name": "value",
        "type": "float"
    }]