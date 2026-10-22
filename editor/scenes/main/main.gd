class_name Editor extends Node2D

@onready var net_list: NetList = $NetList
@onready var plot_window: Window = $PlotWindow

enum State {Idle, ConnectWire, Simulate, AddDevice}

var state: State = State.Idle
var selected_port = null
var device_to_add = null

var client = StreamPeerTCP.new()
var server_data: PackedByteArray = []

var PlotterScene = preload("res://scenes/plotter/plotter.tscn")
var plotter: Plotter:
    get:
        return $%Plotter
var solution = {}

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

    var devices = $Devices.get_children()
    for device in devices:
        if not device is Device:
            continue
        for i in range(device.current_num):
            device.current_ids.append(cur_id)
            cur_id += 1

func allocate_device_id():
    var cur_id = 0

    var devices = $Devices.get_children()
    for device in devices:
        if not device is Device:
            continue
        device.device_id = cur_id
        cur_id += 1

func construct():
    allocate_node_id()
    allocate_current_id()
    allocate_device_id()

    var devices = $Devices.get_children()
    var circuit = []
    for device in devices:
        circuit.append((device as Device).construct())
    return circuit

func _process(_delta: float) -> void:
    $%SimButton.disabled = state == State.ConnectWire

    if $%ConnectTimer.is_stopped():
        client.poll()
        if client.get_status() != StreamPeerTCP.STATUS_CONNECTED:
            $%ConnectTimer.start()

    if state == State.AddDevice:
        device_to_add.position = $Devices.to_local(get_global_mouse_position())

    client.poll()
    if client.get_status() != StreamPeerTCP.STATUS_CONNECTED:
        return

    var available_bytes = client.get_available_bytes()
    if available_bytes != 0:
        server_data.append_array(client.get_data(available_bytes)[1])
        
    var index = server_data.find(0)
    while index != -1:
        var json = JSON.parse_string(server_data.slice(0, index).get_string_from_utf8())
        server_data = server_data.slice(index + 1)

        match json["type"]:
            "simulate_solution":
                var time = json["time"]
                solution = json["solution"]
                $%TimeLabel.text = str(time)

                plotter.add_data(time, solution)
            "error":
                $%ErrorDialog.dialog_text = json["message"]
                $%ErrorDialog.popup()

                if state == State.Simulate:
                    $%SimButton.pressed.emit()

        index = server_data.find(0)

func send_to_server(data):
    client.put_data(JSON.stringify(data).to_utf8_buffer())
    client.put_u8(0)

func _on_sim_button_pressed() -> void:
    match state:
        State.Idle:
            state = State.Simulate
            $%SimButton.text = "停止模拟"
            $PlotWindow.show()
            plotter.clear()

            var circuit = construct()
            send_to_server({
                "command": "reset_simulation",
                "circuit": circuit
            })
        State.Simulate:
            state = State.Idle
            $%SimButton.text = "开始模拟"

func _on_connect_timer_timeout() -> void:
    client.poll()
    match client.get_status():
        StreamPeerTCP.STATUS_NONE:
            $%ConnectLabel.text = "连接中..."
            client.connect_to_host("127.0.0.1", 1025)
        StreamPeerTCP.STATUS_CONNECTING:
            pass
        StreamPeerTCP.STATUS_CONNECTED:
            $%ConnectLabel.text = "已连接"
            client.set_no_delay(true)
            $%ConnectTimer.stop()
        StreamPeerTCP.STATUS_ERROR:
            client.disconnect_from_host()

func _on_step_timer_timeout() -> void:
    if state != State.Simulate:
        return
    send_to_server({"command": "step_simulation"})

func _on_device_lib_button_pressed() -> void:
    $DeviceLibWindow.show()

func _on_device_lib_window_close_requested() -> void:
    $DeviceLibWindow.hide()

func _input(event: InputEvent) -> void:
    if state != Editor.State.AddDevice:
        return
    if Input.is_action_just_pressed("cancel"):
        state = Editor.State.Idle
        device_to_add.get_parent().remove_child(device_to_add)
        device_to_add = null
    if event is InputEventMouseButton and event.is_pressed() and event.button_index == MOUSE_BUTTON_LEFT:
        state = Editor.State.Idle
