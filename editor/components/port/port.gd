class_name Port extends Node2D

const SET_REF_ID = 0
const DELETE_ID = 1

signal moved

@export var is_ref := false

@onready var net_list: NetList = $/root/Editor/NetList
@onready var area: ColorRect = $Area
@onready var editor: Editor = $/root/Editor
@onready var camera: Camera2D = $/root/Editor/Camera2D

var node_id := UINT32_MAX
var is_free:
    get:
        return is_inside_tree() and get_parent() == net_list
var just_moved = false

func _ready() -> void:
    if self not in net_list.connections:
        net_list.connections[self] = []

    if not is_free:
        $PopupMenu.remove_item(DELETE_ID)

func _process(_delta: float) -> void:
    global_rotation = 0
    
    area.visible = is_mouse_in_area()

func is_mouse_in_area():
    return Rect2(area.position, area.size).has_point(get_local_mouse_position())

func _input(event: InputEvent) -> void:
    if event is InputEventMouseMotion and event.pressure == 1 and is_free and (is_mouse_in_area() or just_moved):
        position += event.relative / camera.zoom
        just_moved = true
        moved.emit()
    if event is InputEventMouseButton and event.is_released() and event.button_index == MOUSE_BUTTON_LEFT and (is_mouse_in_area() or just_moved):
        get_viewport().set_input_as_handled()
        if just_moved:
            just_moved = false
        elif is_mouse_in_area():
            editor.hsm.dispatch(&"connect_wire", self)
    if event is InputEventMouseButton and event.is_pressed() and event.button_index == MOUSE_BUTTON_RIGHT and is_mouse_in_area():
        get_viewport().set_input_as_handled()
        $PopupMenu.position = DisplayServer.mouse_get_position()
        $PopupMenu.popup()

func _on_popup_menu_id_pressed(id: int) -> void:
    if id == SET_REF_ID:
        is_ref = not is_ref
        $PopupMenu.set_item_checked(SET_REF_ID, is_ref)
    if id == DELETE_ID:
        get_parent().remove_child(self)
