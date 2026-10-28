class_name Oscilloscope extends Device

@onready var editor: Editor = $/root/Editor
@onready var plotter: Plotter = $%Plotter

func _process(_delta: float) -> void:
    $Panel.rotation = - rotation

    var key = "u%d" % $Probe.node_id
    var value = editor.solution.get(key, null)
    if value == null:
        return
    plotter.add_data(editor.sim_time, {key: value})

func get_device_name() -> String:
    return "示波器"
