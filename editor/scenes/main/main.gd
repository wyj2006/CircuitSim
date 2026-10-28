class_name Editor extends Node2D

@onready var VoltageProbeScene = preload("res://devices/voltageprobe/voltage_probe.tscn")
@onready var hsm: LimboHSM = $%HSM
@onready var connect_wire_state: LimboState = $%HSM/ConnectWire
@onready var idle_state: LimboState = $%HSM/Idle
@onready var add_device_state: LimboState = $%HSM/AddDevice
@onready var simulate_state: LimboState = $%HSM/Simulate
@onready var client: Client = $Client

var active_state: LimboState:
    get:
        return hsm.get_active_state()
var leaf_state: LimboState:
    get:
        return hsm.get_leaf_state()

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
    if client.status == StreamPeerTCP.STATUS_CONNECTED:
        $%ConnectLabel.text = "已连接"
    else:
        $%ConnectLabel.text = "连接中..."

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

func _on_v_probe_button_pressed() -> void:
    hsm.dispatch(&"add_device", VoltageProbeScene.instantiate())

func _on_client_new_solution(time: float, _solution: Dictionary) -> void:
    $%TimeLabel.text = str(time)
