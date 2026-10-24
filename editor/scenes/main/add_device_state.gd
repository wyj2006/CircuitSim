extends LimboState

@onready var devices: Node2D = $/root/Editor/Devices

var device_to_add: Device:
    get:
        return blackboard.get_var(&"device_to_add")
    set(value):
        blackboard.set_var(&"device_to_add", value)

func _update(_delta: float) -> void:
    device_to_add.position = devices.get_local_mouse_position()

func _setup() -> void:
    add_event_handler(&"cancel_add_device", func():
        device_to_add.get_parent().remove_child(device_to_add)
        device_to_add = null

        return false
    )

func _exit() -> void:
    device_to_add = null