class_name Editor extends Node2D

@onready var plot_window: Window = $PlotWindow
@onready var hsm: LimboHSM = $%HSM
@onready var connect_wire_state: LimboState = $%HSM/ConnectWire
@onready var idle_state: LimboState = $%HSM/Idle
@onready var add_device_state: LimboState = $%HSM/AddDevice
@onready var simulate_state: LimboHSM = $%HSM/Simulate
@onready var showv_sim_state: LimboState = $%HSM/Simulate/ShowVoltage

var active_state: LimboState:
    get:
        return hsm.get_active_state()
var leaf_state: LimboState:
    get:
        return hsm.get_leaf_state()

var client = StreamPeerTCP.new()
var server_data: PackedByteArray = []

var PlotterScene = preload("res://scenes/plotter/plotter.tscn")
var plotter: Plotter:
    get:
        return $%Plotter
var solution = {}

func _ready() -> void:
    hsm.add_transition(idle_state, add_device_state, &"add_device")
    hsm.add_transition(add_device_state, idle_state, &"confirm_add_device")
    hsm.add_transition(add_device_state, idle_state, &"cancel_add_device")

    hsm.add_transition(idle_state, connect_wire_state, &"connect_wire")
    hsm.add_transition(connect_wire_state, idle_state, &"connect_wire")
    hsm.add_transition(connect_wire_state, idle_state, &"cancel_connect_wire")

    hsm.add_transition(idle_state, simulate_state, &"simulate")
    hsm.add_transition(simulate_state, idle_state, &"simulate")

    hsm.add_transition(hsm.ANYSTATE, idle_state, &"error_occurred")

    hsm.initialize(self)
    hsm.set_active(true)

func _process(_delta: float) -> void:
    if $%ConnectTimer.is_stopped():
        client.poll()
        if client.get_status() != StreamPeerTCP.STATUS_CONNECTED:
            $%ConnectTimer.start()

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

                hsm.dispatch(&"error_occurred")

        index = server_data.find(0)

func send_to_server(data):
    client.poll()
    if client.get_status() != StreamPeerTCP.STATUS_CONNECTED:
        return
        
    client.put_data(JSON.stringify(data).to_utf8_buffer())
    client.put_u8(0)

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

func _on_device_lib_button_pressed() -> void:
    $DeviceLibWindow.show()

func _on_device_lib_window_close_requested() -> void:
    $DeviceLibWindow.hide()

func _input(event: InputEvent) -> void:
    if Input.is_action_just_pressed("cancel"):
        hsm.dispatch(&"cancel_add_device")
        hsm.dispatch(&"cancel_connect_wire")
    if event is InputEventMouseButton and event.is_pressed() and event.button_index == MOUSE_BUTTON_LEFT:
        hsm.dispatch(&"confirm_add_device")

func _on_sim_button_pressed() -> void:
    hsm.dispatch(&"simulate")

func _on_pause_button_pressed() -> void:
    simulate_state.dispatch(&"pause_simulate")

func _on_step_button_pressed() -> void:
    simulate_state.dispatch(&"step_simulate")

func _on_continuous_button_pressed() -> void:
    simulate_state.dispatch(&"continuous_simulate")
