class_name Editable extends Node2D

signal moved(device)

@export var range_size: Vector2

@onready var camera: Camera2D = $/root/Editor/Camera2D
@onready var area: ReferenceRect = $Area
@onready var parent: Node2D = get_parent()

var selected := false

func _ready() -> void:
    area.size = range_size
    area.position = - range_size / 2

func _process(_delta: float) -> void:
    area.visible = selected

func _input(event: InputEvent) -> void:
    if event is InputEventMouseButton:
        if event.is_pressed() and event.button_index == MOUSE_BUTTON_LEFT:
            if Rect2(area.position, area.size).has_point(get_local_mouse_position()):
                selected = true
            else:
                selected = false
        if event.double_click and selected:
            $AttrEditorWin.show()
            
    if event is InputEventMouseMotion and selected and event.pressure == 1:
        parent.position += event.relative / camera.zoom
        moved.emit(get_parent())

    if Input.is_action_just_pressed("rotate_device") and selected:
        parent.rotation_degrees += 90
    if Input.is_action_just_pressed("delete_device") and selected:
        get_parent().get_parent().remove_child(get_parent())

func _on_attr_editor_win_close_requested() -> void:
    $AttrEditorWin.hide()
