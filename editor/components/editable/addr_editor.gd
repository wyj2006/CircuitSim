extends Control

@onready var device: Device = $"../../.."
@onready var label_template: Label = $LabelTemplate
@onready var spinbox_template: SpinBox = $SpinBoxTemplate
@onready var expr_editor_template: ExprEditor = $ExprEditorTemplate

func _ready() -> void:
    var attrs = device.attributes
    for attr in attrs:
        var layout = HBoxContainer.new()
        
        var label = label_template.duplicate()
        label.text = attr["display_name"]
        label.show()
        layout.add_child(label)

        match attr["type"]:
            "float", "int":
                var spinbox = spinbox_template.duplicate()
                
                if attr["type"] == "int":
                    spinbox.step = 1
                    spinbox.rounded = true

                spinbox.value = device.get(attr["attr_name"])
                spinbox.connect("value_changed", func(value): device.set(attr["attr_name"], value))
                spinbox.show()

                $Timer.timeout.connect(func():
                    var val = device.get(attr["attr_name"])
                    if val != spinbox.value:
                        spinbox.value = val
                )

                layout.add_child(spinbox)
            "expr":
                var expr_editor = expr_editor_template.duplicate()
                expr_editor.expr_str = device.get(attr["attr_name"])
                expr_editor.expr_changed.connect(func(value): device.set(attr["attr_name"], value))
                expr_editor.show()

                $Timer.timeout.connect(func():
                    var val = device.get(attr["attr_name"])
                    if val != expr_editor.expr_str:
                        expr_editor.expr_str = val
                )

                layout.add_child(expr_editor)

        $VBoxContainer.add_child(layout)
