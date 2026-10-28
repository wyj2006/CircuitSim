class_name Oscilloscope extends Device

@onready var plotter: Plotter = $%Plotter
@onready var client: Client = $/root/Editor/Client

func _ready() -> void:
    client.new_solution.connect(func(time, solution):
        var key = "u%d" % $Probe.node_id
        var value = solution.get(key, null)
        if value == null:
            return
        plotter.add_data(time, {key: value})
    )

func _process(_delta: float) -> void:
    $Panel.rotation = - rotation

func get_device_name() -> String:
    return "示波器"
