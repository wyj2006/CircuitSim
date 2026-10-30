extends Editor

func _ready() -> void:
    super()

    $%Oscilloscope.channel_num = 1
    $NetList.connect_port($%Oscilloscope/Probe0, $%Inductor/PosPort)