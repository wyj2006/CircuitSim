class_name VoltageProbe extends Device

@onready var editor: Editor = $/root/Editor
@onready var client: Client = $/root/Editor/Client

func _ready() -> void:
    client.new_solution.connect(func(_time, solution):
        var key = "u%d" % $Probe.node_id
        var value = solution.get(key, null)
        if value == null:
            $%Value.text = ""
        else:
            $%Value.text = str(value)
    )

func _process(_delta: float) -> void:
    $Panel.rotation = - rotation

func get_device_name() -> String:
    return "电压探针"
