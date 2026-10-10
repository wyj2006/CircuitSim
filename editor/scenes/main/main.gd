class_name Editor extends Node2D

@onready var net_list: NetList = $NetList
@onready var plot_window: Window = $PlotWindow

enum State {Idle, ConnectWire, Simulate, AddDevice}

var state: State = State.Idle
var selected_port = null
var device_to_add = null

var client = StreamPeerTCP.new()
var PlotterScene = preload("res://scenes/plotter/plotter.tscn")
var plotter: Plotter = null
var solution = {}

func allocate_node_id():
    var allocated = []
    var has_ref = false
    var cur_id = 0

    for i in net_list.connections:
        if i.is_ref:
            has_ref = true
            _allocate_node_id(i, UINT32_MAX, allocated)

    if not has_ref:
        _allocate_node_id(net_list.connections.keys()[0], UINT32_MAX, allocated)

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

func allocate_branch_id():
    var cur_id = 0

    var devices = $Devices.get_children()
    for device in devices:
        var ports: Array[Port] = []
        for child in device.get_children():
            if child is Port:
                ports.append(child)
        if len(ports) <= 2:
            for port in ports:
                port.branch_id = cur_id
            cur_id += 1
        else:
            for port in ports:
                port.branch_id = cur_id
                cur_id += 1

func construct():
    allocate_node_id()
    allocate_branch_id()

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

func _on_sim_button_pressed() -> void:
    match state:
        State.Idle:
            state = State.Simulate
            $%SimButton.text = "停止模拟"

            var circuit = construct()
            print(circuit)
            client.put_data(JSON.stringify({
                "command": "reset_simulation",
                "circuit": circuit
            }).to_utf8_buffer())
            client.put_u8(0)
        State.Simulate:
            state = State.Idle
            $%SimButton.text = "开始模拟"

            plot_window.hide()
            plot_window.remove_child(plotter)
            plotter.queue_free()
            plotter = null

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
    client.put_data(JSON.stringify({"command": "step_simulation"}).to_utf8_buffer())
    client.put_u8(0)

    var available_bytes = client.get_available_bytes()
    if available_bytes == 0:
        return
    var data: PackedByteArray = client.get_data(available_bytes)[1]
    var json = JSON.parse_string(data.get_string_from_utf8())
    if json == null:
        return
    if json["type"] != "simulate_solution":
        return
    var time = json["time"]
    solution = json["solution"]
    print(time)
    print(solution)
    $%TimeLabel.text = str(time)

    if plotter == null:
        plotter = PlotterScene.instantiate()
        for i in solution:
            plotter.variables[i] = 0
        plot_window.add_child(plotter)
        plot_window.show()
    plotter.add_data(time, solution)

func _on_device_lib_button_pressed() -> void:
    $DeviceLibWindow.show()

func _on_device_lib_window_close_requested() -> void:
    $DeviceLibWindow.hide()
