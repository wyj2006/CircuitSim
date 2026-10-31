class_name Wire extends Node2D

const DELETE_ID = 0

@export var a_port: Port
@export var b_port: Port

@onready var editor: Editor = $/root/Editor
@onready var net_list: NetList = get_parent()

var color = Color.BLACK
var points = []
var threshold = 4

func update_points():
    points = []
    
    var a_pos = net_list.to_local(a_port.global_position)
    var b_pos = net_list.to_local(b_port.global_position)

    var start = Vector2i(a_pos)
    var end = Vector2i(b_pos)
    points = net_list.find_path(start, end)

func _draw() -> void:
    if points.size() >= 2:
        draw_polyline(points, Color.BLACK, 1)

    var closest = mouse_on_wire()
    if closest != null:
        var size = Vector2(threshold, threshold)
        draw_rect(Rect2(closest - size / 2, size), Color.RED)

func on_port_tree_exiting():
    var parent = get_parent()
    if parent == null:
        return
    parent.remove_child(self)

func _ready() -> void:
    a_port.tree_exiting.connect(on_port_tree_exiting)
    b_port.tree_exiting.connect(on_port_tree_exiting)

    if not a_port.moved.is_connected(net_list.update_router.call_deferred):
        a_port.moved.connect(net_list.update_router.call_deferred)
    if not a_port.moved.is_connected(net_list.update_router.call_deferred):
        b_port.moved.connect(net_list.update_router.call_deferred)

    if a_port not in net_list.connections:
        net_list.connections[a_port] = []
    if b_port not in net_list.connections[a_port]:
        net_list.connections[a_port].append(b_port)
    if b_port not in net_list.connections:
        net_list.connections[b_port] = []
    if a_port not in net_list.connections[b_port]:
        net_list.connections[b_port].append(a_port)

func _process(_delta: float) -> void:
    queue_redraw()

func mouse_on_wire():
    var closest = null
    var mouse_pos = get_local_mouse_position()
    
    for i in range(points.size() - 1):
        var a = points[i]
        var b = points[i + 1]

        var t = Geometry2D.get_closest_point_to_segment(
            mouse_pos, a, b
        )

        if mouse_pos.distance_to(t) <= threshold and (closest == null or closest.distance_to(mouse_pos) > t.distance_to(mouse_pos)):
            closest = t

    return closest

func _on_tree_exiting() -> void:
    a_port.tree_exiting.disconnect(on_port_tree_exiting)
    b_port.tree_exiting.disconnect(on_port_tree_exiting)

    if a_port.moved.is_connected(net_list.update_router.call_deferred):
        a_port.moved.disconnect(net_list.update_router.call_deferred)
    if a_port.moved.is_connected(net_list.update_router.call_deferred):
        b_port.moved.disconnect(net_list.update_router.call_deferred)
    
    net_list.disconnect_port(a_port, b_port)

func _on_tree_exited() -> void:
    net_list.update_router()

func _unhandled_input(event: InputEvent) -> void:
    if event is InputEventMouseButton and event.is_pressed() and event.button_index == MOUSE_BUTTON_RIGHT and mouse_on_wire() != null:
        $%PopupMenu.position = DisplayServer.mouse_get_position()
        $%PopupMenu.popup()

func _on_popup_menu_id_pressed(id: int) -> void:
    if id == DELETE_ID:
        get_parent().remove_child(self)
