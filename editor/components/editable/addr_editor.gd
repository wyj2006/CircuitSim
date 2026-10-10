extends Control

@onready var device: Device = $"../../.."
@onready var label_template: Label = $LabelTemplate
@onready var spinbox_template: SpinBox = $SpinBoxTemplate

func _ready() -> void:
    var attrs = device.attributes
    for attr in attrs:
        var layout = HBoxContainer.new()
        
        var label = label_template.duplicate()
        label.text = attr["display_name"]
        label.show()
        layout.add_child(label)

        match attr["type"]:
            "float":
                var spinbox = spinbox_template.duplicate()
                spinbox.value = device.get(attr["attr_name"])
                spinbox.connect("value_changed", func(value): device.set(attr["attr_name"], value))
                spinbox.show()
                layout.add_child(spinbox)

        $VBoxContainer.add_child(layout)
