extends LimboState

@onready var devices: Node2D = $/root/Editor/Devices
@onready var net_list: NetList = $/root/Editor/NetList

var device_to_add: Device:
    get:
        return blackboard.get_var(&"device_to_add")
    set(value):
        blackboard.set_var(&"device_to_add", value)

var selected_port: Port:
    get:
        return blackboard.get_var(&"selected_port")
    set(value):
        blackboard.set_var(&"selected_port", value)

func _setup() -> void:
    add_event_handler(&"add_device", func(device: Device):
        device_to_add = device
        devices.add_child(device)

        return false
    )
    add_event_handler(&"connect_wire", func(port: Port):
        if selected_port == null:
            selected_port = port
            return false
        # 在这里不会发生, 只是为了与connect_wire_state保持一致
        if selected_port == port:
            selected_port = null
            return false
        net_list.connect_port(port, selected_port)
        selected_port = null

        return false
    )
