class_name Editable extends Node2D

const DELETE_ID = 0
const ATTREDIT_ID = 1

signal moved(device)
signal rotated(device)

@export var range_size: Vector2
@export var could_rotate = true

@onready var camera: Camera2D = $/root/Editor/Camera2D
@onready var area: ReferenceRect = $Area
@onready var parent: Node2D = get_parent()

var AttrEditorScene = preload("res://components/editable/attr_editor.tscn")
var selected := false

func _ready() -> void:
    area.size = range_size
    area.position = - range_size / 2

func _process(_delta: float) -> void:
    area.visible = selected

func _input(event: InputEvent) -> void:
    if event is InputEventMouseButton:
        if event.is_pressed():
            if Rect2(area.position, area.size).has_point(get_local_mouse_position()):
                selected = true
            else:
                selected = false
        if event.double_click and selected:
            $AttrEditorWin.show()
            
    if event is InputEventMouseMotion and selected and event.pressure == 1:
        parent.position += event.relative / camera.zoom
        moved.emit(get_parent())

    if Input.is_action_just_pressed("rotate_device") and selected and could_rotate:
        parent.rotation_degrees += 90
        rotated.emit(get_parent())
        
    if Input.is_action_just_pressed("delete_device") and selected:
        get_parent().get_parent().remove_child(get_parent())

    if event is InputEventMouseButton and event.is_pressed() and event.button_index == MOUSE_BUTTON_RIGHT and selected:
        $%PopupMenu.position = DisplayServer.mouse_get_position()
        $%PopupMenu.popup()

func _on_attr_editor_win_close_requested() -> void:
    $AttrEditorWin.hide()

func _on_attr_editor_win_visibility_changed() -> void:
    if $AttrEditorWin.visible:
        $AttrEditorWin.add_child(AttrEditorScene.instantiate())
    else:
        for child in $AttrEditorWin.get_children():
            $AttrEditorWin.remove_child.call_deferred(child)

func _on_popup_menu_id_pressed(id: int) -> void:
    match id:
        DELETE_ID:
            get_parent().get_parent().remove_child(get_parent())
        ATTREDIT_ID:
            $AttrEditorWin.show()