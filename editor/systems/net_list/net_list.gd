class_name NetList extends Node2D

@onready var editor: Editor = $/root/Editor

var WireScene = preload("res://systems/net_list/wire.tscn")
var PortScene = preload("res://components/port/port.tscn")

var connections: Dictionary[Port, Array] = {}
var wires_under_mouse: Array[Wire] = []

var cell_size = Vector2i(1, 1)
var solid_points = {}

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
        var points = []
    
        var a_pos = to_local(editor.connect_wire_state.selected_port.global_position)
        var b_pos = get_local_mouse_position()

        var start = world_to_cell(a_pos)
        var end = world_to_cell(b_pos)
        var ids = get_id_path(start, end)

        if len(ids) >= 2:
            for id in ids:
                points.append(cell_to_world(id))
        else:
            points = [a_pos, b_pos]

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
            solid_points[Vector2i(x, y)] = true

func build_astar():
    solid_points = {}
    #一定要让NetList在场景树中的Devices下面
    for child in $/root/Editor/Devices.get_children():
        if not child is Device:
            continue
        set_obstacle(Rect2(child.region.position + to_local(child.global_position), child.region.size))

func get_id_path(start: Vector2i, end: Vector2i):
    var min_turned_point = UINT32_MAX
    var final_path = []

    for start_dir in [Vector2i(-1, 0), Vector2i(1, 0), Vector2i(0, -1), Vector2i(0, 1)]:
        var queue = [start]
        var priority = {start: 0}
        var came_from = {start: null}
        var cost = {start: 0}

        while not queue.is_empty():
            var current = queue.pop_back()
            if current == end:
                break
            for dir in [start_dir] if current == start else [Vector2i(-1, 0), Vector2i(1, 0), Vector2i(0, -1), Vector2i(0, 1)]:
                var next = current + dir
                #当前节点不在障碍内, 但下一个节点在就忽略这个节点
                if next in solid_points and current not in solid_points:
                    continue

                var turned = false
                if came_from[current] != null:
                    var last_dir = current - came_from[current]
                    turned = last_dir != dir

                var new_cost = cost[current] + abs(dir.x) + abs(dir.y) + (1 if turned else 0)

                if not (next not in cost or new_cost < cost[next]):
                    continue

                cost[next] = new_cost
                came_from[next] = current

                if next in priority:
                    queue.erase(next)
                priority[next] = new_cost + abs(end.x - next.x) + abs(end.y - next.y)
                for i in range(queue.size() + 1):
                    if i >= queue.size() or priority[queue[i]] <= priority[next]:
                        queue.insert(i, next)
                        break
        
        var turned_point = 0
        var path = [end]
        while came_from[path[0]] != null:
            path.insert(0, came_from[path[0]])
            if path.size() >= 3 and path[0] - path[1] != path[1] - path[2]:
                turned_point += 1
        
        if turned_point < min_turned_point:
            min_turned_point = turned_point
            final_path = path

    return final_path

func _ready() -> void:
    build_astar()

func _on_devices_child_entered_tree(node: Node) -> void:
    var editable: Editable = node.find_child("Editable")
    if editable == null:
        return
    editable.moved.connect(func(_device): build_astar())

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
