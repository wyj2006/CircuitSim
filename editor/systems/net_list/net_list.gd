class_name NetList extends Node2D

@onready var editor: Editor = $/root/Editor

var WireScene = preload("res://systems/net_list/wire.tscn")
var PortScene = preload("res://components/port/port.tscn")
var connections: Dictionary[Port, Array] = {}
var astar: AStarGrid2D = AStarGrid2D.new()
var cell_size = Vector2i(1, 1)
var wires_under_mouse: Array[Wire] = []

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
    match editor.state:
        Editor.State.ConnectWire:
            var points = []
    
            var a_pos = to_local(editor.selected_port.global_position)
            var b_pos = get_local_mouse_position()

            var start = world_to_cell(a_pos)
            var end = world_to_cell(b_pos)
            var ids = astar.get_id_path(start, end)

            if len(ids) >= 2:
                for id in ids:
                    points.append(cell_to_world(id))
            else:
                points = [a_pos, b_pos]

            points = simplify_path(points)
            if points.size() < 2:
                return

            draw_polyline(points, Color.BLACK, 1)

func _process(_delta: float) -> void:
    queue_redraw()

func world_to_cell(pos: Vector2) -> Vector2i:
    return Vector2i(ceili(pos.x / cell_size.x), ceili(pos.y / cell_size.y))

func cell_to_world(id: Vector2i) -> Vector2:
    return Vector2(id.x * cell_size.x, id.y * cell_size.y)

func set_obstacle(rect: Rect2):
    var min_id = world_to_cell(rect.position)
    var max_id = world_to_cell(rect.end)

    for x in range(min_id.x, max_id.x + 1):
        for y in range(min_id.y, max_id.y + 1):
            var id = Vector2i(x, y)
            astar.set_point_solid(id, true)

func build_astar():
    astar.clear()
    astar.cell_size = cell_size
    astar.region = Rect2i(-500, -500, 1000, 1000)
    astar.update()

    #一定要让NetList在场景树中的Devices下面
    for child in $/root/Editor/Devices.get_children():
        if not child is Device:
            continue
        set_obstacle(Rect2(child.region.position + to_local(child.global_position), child.region.size))

func _ready() -> void:
    astar.default_compute_heuristic = AStarGrid2D.HEURISTIC_OCTILE
    astar.default_estimate_heuristic = AStarGrid2D.HEURISTIC_OCTILE

    build_astar()

func _on_devices_child_entered_tree(node: Node) -> void:
    var editable: Editable = node.find_child("Editable")
    if editable == null:
        return
    editable.moved.connect(func(_device): build_astar())

func simplify_path(path) -> Array:
    if path.size() <= 2:
        return path

    var result: Array[Vector2i] = [path[0]]
    var prev_dir = path[1] - path[0]

    for i in range(2, path.size()):
        var dir = path[i] - path[i - 1]
        if dir != prev_dir:
            result.append(path[i - 1])
        prev_dir = dir

    result.append(path[-1])
    return result

func _unhandled_input(event: InputEvent) -> void:
    if event is InputEventMouseButton and event.is_released() and event.button_index == MOUSE_BUTTON_LEFT:
        match editor.state:
            Editor.State.Idle:
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
                    editor.state = Editor.State.ConnectWire
                    
                    var port: Port = PortScene.instantiate()
                    port.position = $"../Ports".to_local(closest)
                    $"../Ports".add_child(port)
                    editor.selected_port = port

                    for wire in wires_under_mouse:
                        connect_port(editor.selected_port, wire.a_port)
                        connect_port(editor.selected_port, wire.b_port)
                        remove_child(wire)

            Editor.State.ConnectWire:
                editor.state = Editor.State.Idle

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

                connect_port(port, editor.selected_port)

                editor.selected_port = null
