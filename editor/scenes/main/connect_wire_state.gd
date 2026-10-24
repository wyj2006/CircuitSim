extends LimboState

@onready var net_list: NetList = $/root/Editor/NetList

var selected_port: Port:
    get:
        return blackboard.get_var(&"selected_port")
    set(value):
        blackboard.set_var(&"selected_port", value)

func _setup() -> void:
    add_event_handler(&"connect_wire", func(port: Port):
        if selected_port == null: # 在这里不会发生, 只是为了与idle_state保持一致
            selected_port = port
            return false
        if selected_port == port:
            selected_port = null
            return false
        net_list.connect_port(port, selected_port)
        selected_port = null

        return false
    )

func _exit() -> void:
    selected_port = null