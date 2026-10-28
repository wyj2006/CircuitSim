class_name NetList extends Node2D

@onready var editor: Editor = $/root/Editor
@onready var devices: Node2D = $/root/Editor/Devices
@onready var ports: Node2D = $/root/Editor/Ports

var WireScene = preload("res://systems/netlist/wire.tscn")
var PortScene = preload("res://components/port/port.tscn")

var connections: Dictionary[Port, Array] = {}
var wires_under_mouse: Array[Wire] = []
var router := OrthogonalRouter.new()

func connect_port(a: Port, b: Port):
    if a not in connections:
        connections[a] = []
    if b not in connections:
        connections[b] = []
    connections[a].append(b)
    connections[b].append(a)

    for child in get_children():
        if not child is Wire:
            continue
        if child.a_port == a and child.b_port == b:
            return
        if child.a_port == b and child.b_port == a:
            return
    var wire: Wire = WireScene.instantiate()
    wire.a_port = a
    wire.b_port = b
    add_child(wire)

func disconnect_port(a: Port, b: Port):
    if a in connections:
        connections[a].erase(b)
    if b in connections:
        connections[b].erase(a)

func _draw() -> void:
    if editor.active_state == editor.connect_wire_state:
        update_router()
        
        var a_pos = to_local(editor.connect_wire_state.selected_port.global_position)
        var b_pos = get_local_mouse_position()

        var start = Vector2i(a_pos)
        var end = Vector2i(b_pos)
        var points = find_path(start, end)

        if points.size() >= 2:
            draw_polyline(points, Color.BLACK, 1)

func _process(_delta: float) -> void:
    queue_redraw()

func update_router():
    router.clear()

    var port_pos = []

    #一定要让NetList在场景树中的Devices下面
    for device in devices.get_children():
        if not device is Device:
            continue
        var rect = Rect2(device.region.position + to_local(device.global_position), device.region.size)
        for x in range(rect.position.x, rect.end.x + 1):
            for y in range(rect.position.y, rect.end.y + 1):
                router.set_obstacle(Vector2i(x, y), false)

        for port in device.get_children():
            if not port is Port:
                continue
            port_pos.append(Vector2i(to_local(port.global_position)))

    for port in ports.get_children():
        if not port is Port:
            continue
        port_pos.append(Vector2i(to_local(port.global_position)))

    #避免导线重叠
    for wire in get_children():
        if not wire is Wire:
            continue
        wire.update_points()
        for i in range(wire.points.size() - 1):
            var a: Vector2i = wire.points[i]
            var b: Vector2i = wire.points[i + 1]

            if not (a.x == b.x or a.y == b.y):
                continue

            var dir = (b - a).sign()
            var k = a
            while k != b:
                if k not in port_pos:
                    router.set_obstacle(k, false)
                k += dir

func find_path(start: Vector2i, end: Vector2i):
    return router.find_path(start, end)

func _ready() -> void:
    update_router()

func _on_devices_child_entered_tree(node: Node) -> void:
    var editable: Editable = node.find_child("Editable")
    if editable == null:
        return
    editable.moved.connect(func(_device): update_router())

func _on_child_entered_tree(node: Node) -> void:
    if node is Wire:
        update_router.call_deferred()
    
func _unhandled_input(event: InputEvent) -> void:
    if event is InputEventMouseButton and event.is_released() and event.button_index == MOUSE_BUTTON_LEFT:
        if editor.active_state == editor.idle_state:
            wires_under_mouse = []
            var closest = null
            for child in get_children():
                if not child is Wire:
                    continue
                var t = child.mouse_on_wire()
                if t != null:
                    wires_under_mouse.append(child)
                    closest = child.to_global(t)

            if wires_under_mouse.size() > 0:
                var port: Port = PortScene.instantiate()
                port.position = $"../Ports".to_local(closest)
                $"../Ports".add_child(port)

                for wire in wires_under_mouse:
                    connect_port(port, wire.a_port)
                    connect_port(port, wire.b_port)
                    remove_child(wire)

                editor.hsm.dispatch(&"connect_wire", port)
        elif editor.active_state == editor.connect_wire_state:
            var port: Port = PortScene.instantiate()
            $"../Ports".add_child(port)

            var closest = get_global_mouse_position()

            for child in get_children():
                if not child is Wire:
                    continue
                var t = child.mouse_on_wire()
                if t == null:
                    continue
                closest = child.to_global(t)

                connect_port(port, child.a_port)
                connect_port(port, child.b_port)
                remove_child(child)
                
            port.position = $"../Ports".to_local(closest)

            editor.hsm.dispatch(&"connect_wire", port)
