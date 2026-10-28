class_name SPDTSwitch extends Device

@export var is_touch1 = true;

@onready var common_port: Port = $CommonPort
@onready var throw1_port: Port = $Throw1Port
@onready var throw2_port: Port = $Throw2Port
@onready var editor: Editor = $/root/Editor
@onready var client: Client = $/root/Editor/Client

var just_moved = false
var selected:
    get:
        return $Editable.selected

func _ready() -> void:
    current_num = 2

func _process(_delta: float) -> void:
    $Sprite2D.flip_v = not is_touch1

func construct():
    return {
        "device_id": device_id,
        "type": "spdt_switch",
        "common": common_port.node_id,
        "throw1": throw1_port.node_id,
        "throw2": throw2_port.node_id,
        "is_touch1": is_touch1,
        "current1": current_ids[0],
        "current2": current_ids[1]
    }

func get_device_name() -> String:
    return "单刀双掷开关"

func _input(event: InputEvent) -> void:
    if event is InputEventMouseMotion and event.pressure == 1 and selected:
        just_moved = true
    if event is InputEventMouseButton and event.is_released() and event.button_index == MOUSE_BUTTON_LEFT and selected:
        if just_moved:
            just_moved = false
        else:
            is_touch1 = not is_touch1
            client.send_to_server({
                "command": "update_device",
                "device_id": device_id,
                "attributes": {
                    "is_touch1": is_touch1
                }
            })
