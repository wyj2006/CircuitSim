class_name Client extends Node2D

signal new_solution(time: float, solution: Dictionary)

@onready var editor: Editor = $/root/Editor

var stream = StreamPeerTCP.new()
var server_data: PackedByteArray = []

var status:
    get:
        stream.poll()
        return stream.get_status()

func _process(_delta: float) -> void:
    if $%ConnectTimer.is_stopped():
        if status != StreamPeerTCP.STATUS_CONNECTED:
            $%ConnectTimer.start()

    if status != StreamPeerTCP.STATUS_CONNECTED:
        return

    var available_bytes = stream.get_available_bytes()
    if available_bytes != 0:
        server_data.append_array(stream.get_data(available_bytes)[1])
        
    var index = server_data.find(0)
    while index != -1:
        var json = JSON.parse_string(server_data.slice(0, index).get_string_from_utf8())
        server_data = server_data.slice(index + 1)

        match json["type"]:
            "simulate_solution":
                new_solution.emit(json["time"], json["solution"])
            "error":
                $%ErrorDialog.dialog_text = json["message"]
                $%ErrorDialog.popup()

                editor.hsm.dispatch(&"error_occurred")

        index = server_data.find(0)

func _on_connect_timer_timeout() -> void:
    match status:
        StreamPeerTCP.STATUS_NONE:
            stream.connect_to_host("127.0.0.1", 1025)
        StreamPeerTCP.STATUS_CONNECTING:
            pass
        StreamPeerTCP.STATUS_CONNECTED:
            stream.set_no_delay(true)
            $%ConnectTimer.stop()
        StreamPeerTCP.STATUS_ERROR:
            stream.disconnect_from_host()

func send_to_server(data):
    if status != StreamPeerTCP.STATUS_CONNECTED:
        return
        
    stream.put_data(JSON.stringify(data).to_utf8_buffer())
    stream.put_u8(0)