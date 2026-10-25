extends LimboState

@onready var editor: Editor = $/root/Editor
@onready var net_list: NetList = $/root/Editor/NetList

var devices:
    get:
        return $/root/Editor/Devices.get_children()

func allocate_node_id():
    var allocated = []
    var has_ref = false
    var cur_id = 0

    for i in net_list.connections:
        if i.is_inside_tree() and i.is_ref:
            has_ref = true
            _allocate_node_id(i, UINT32_MAX, allocated)

    if not has_ref:
        for i in net_list.connections:
            if not i.is_inside_tree():
                continue
            _allocate_node_id(i, UINT32_MAX, allocated)
            break

    for i in net_list.connections:
        if i in allocated:
            continue
        _allocate_node_id(i, cur_id, allocated)
        cur_id += 1

func _allocate_node_id(port: Port, cur_id: int, allocated: Array):
    if port in allocated:
        return
    allocated.append(port)
    port.node_id = cur_id
    for i in net_list.connections[port]:
        _allocate_node_id(i, cur_id, allocated)

func allocate_current_id():
    var cur_id = 0

    for device in devices:
        if not device is Device:
            continue
        for i in range(device.current_num):
            device.current_ids.append(cur_id)
            cur_id += 1

func allocate_device_id():
    var cur_id = 0

    for device in devices:
        if not device is Device:
            continue
        device.device_id = cur_id
        cur_id += 1

func construct():
    allocate_node_id()
    allocate_current_id()
    allocate_device_id()

    var circuit = []
    for device in devices:
        circuit.append((device as Device).construct())
    return circuit

func _enter() -> void:
    $/root/Editor/%SimButton.text = "退出模拟"
    $/root/Editor/%PauseButton.disabled = false
    $/root/Editor/%StepButton.disabled = false
    $/root/Editor/%ContinuousButton.disabled = false
    
    editor.plotter.clear()
    editor.plot_window.show()

    var circuit = construct()
    editor.send_to_server({
        "command": "reset_simulation",
        "circuit": circuit
    })

func _exit() -> void:
    $/root/Editor/%SimButton.text = "进入模拟"
    $/root/Editor/%PauseButton.disabled = true
    $/root/Editor/%StepButton.disabled = true
    $/root/Editor/%ContinuousButton.disabled = true

    editor.send_to_server({
        "command": "stop_simulation",
    })

func _setup() -> void:
    $/root/Editor/%SimButton.text = "进入模拟"
    $/root/Editor/%PauseButton.disabled = true
    $/root/Editor/%StepButton.disabled = true
    $/root/Editor/%ContinuousButton.disabled = true

    add_event_handler(&"pause_simulate", func():
        editor.send_to_server({
            "command": "stop_simulation",
        })
        return false
    )

    add_event_handler(&"step_simulate", func():
        editor.send_to_server({
            "command": "step_simulation",
        })
        return false
    )

    add_event_handler(&"continuous_simulate", func():
        editor.send_to_server({
            "command": "start_simulation",
        })
        return false
    )
