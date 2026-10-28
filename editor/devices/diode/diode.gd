class_name Diode extends Device

@export var isat_base: float = 1
@export var isat_exp: float = -9
@export var vt_base: float = 28.85
@export var vt_exp: float = -3

var saturation_current:
    get:
        return isat_base * pow(10, isat_exp)
var thermal_voltage:
    get:
        return vt_base * pow(10, vt_exp)

func construct():
    return {
        "device_id": device_id,
        "type": "diode",
        "saturation_current": saturation_current,
        "thermal_voltage": thermal_voltage,
        "pos": $PosPort.node_id,
        "neg": $NegPort.node_id
    }

func get_device_name() -> String:
    return "二极管"

func get_attributes() -> Array[Dictionary]:
    return [
        {
            "display_name": "饱和电流底数部分",
            "attr_name": "isat_base",
            "type": "float"
        },
        {
            "display_name": "饱和电流指数部分",
            "attr_name": "isat_exp",
            "type": "float"
        },
        {
            "display_name": "热电压底数部分",
            "attr_name": "vt_base",
            "type": "float"
        },
        {
            "display_name": "热电压指数部分",
            "attr_name": "vt_exp",
            "type": "float"
        },
    ]
