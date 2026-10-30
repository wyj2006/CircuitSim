class_name Oscilloscope extends Device

@onready var plotter: Plotter = $%Plotter
@onready var client: Client = $/root/Editor/Client
@onready var panel: Panel = $Panel

var PortScene = preload("res://components/port/port.tscn")
var line_len = 20

var _channel_num = 0
@onready var channel_num:
    get:
        return _channel_num
    set(value):
        var sep = panel.size.x / (value + 1)
        var x = - panel.size.x / 2 + sep
        var y = panel.size.y / 2
        var i = 0
        
        #添加新节点
        while i < value:
            var port_name = "Probe%d"%i
            var port: Port = get_node_or_null(port_name)
            if port == null:
                port = PortScene.instantiate()
                port.name = port_name
                add_child(port)
            port.position = Vector2(x, y + line_len)

            var line_name = "Line%d"%i
            var line: Line2D = $Lines.get_node_or_null(line_name)
            if line == null:
                line = Line2D.new()
                line.name = line_name
                line.default_color = Color.BLACK
                line.width = 1
                $Lines.add_child(line)
            else:
                line.clear_points()
            line.add_point(Vector2(x, y))
            line.add_point(Vector2(x, y + line_len))

            x += sep
            i += 1

        #移除多余的节点
        while i < _channel_num:
            var port_name = "Probe%d"%i
            var port: Port = get_node_or_null(port_name)
            if port != null:
                remove_child(port)
            var line_name = "Line%d"%i
            var line: Line2D = $Lines.get_node_or_null(line_name)
            if line != null:
                $Lines.remove_child(line)
            i += 1

        _channel_num = value

func _ready() -> void:
    client.new_solution.connect(func(time, solution):
        for port in get_children():
            if not port is Port:
                continue
            var key = "u%d" % port.node_id
            var value = solution.get(key, null)
            if value == null:
                return
            plotter.add_data(time, {key: value})
    )

func _process(_delta: float) -> void:
    $Panel.rotation = - rotation

func get_device_name() -> String:
    return "示波器"

func get_attributes() -> Array[Dictionary]:
    return [ {
        "display_name": "通道数",
        "attr_name": "channel_num",
        "type": "int"
    }]