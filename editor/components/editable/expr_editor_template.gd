class_name ExprEditor extends HBoxContainer

signal expr_changed(String)

@onready var expr = Expression.new()

var expr_str:
    get:
        return $ExprText.text
    set(value):
        $ExprText.text = value

func _on_expr_text_text_changed(new_text: String) -> void:
    $ErrorLabel.text = ""
    if expr.parse(new_text, ["t"]) == OK:
        expr_changed.emit(new_text)
        return
    $ErrorLabel.text = expr.get_error_text()
