extends Camera2D

func _input(event: InputEvent) -> void:
    if event is InputEventMouseButton:
        match event.button_index:
            MOUSE_BUTTON_WHEEL_UP:
                zoom *= 1.2
            MOUSE_BUTTON_WHEEL_DOWN:
                if zoom / 1.2 > Vector2(0.01, 0.01):
                    zoom /= 1.2
    if event is InputEventMouseMotion and Input.is_mouse_button_pressed(MOUSE_BUTTON_MIDDLE):
        position -= event.relative / zoom