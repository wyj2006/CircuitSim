class_name Plotter extends Control

@onready var tau_plot = $TauPlot

var dataset: TauPlot.Dataset
var variables = {}
var bindings: Array[TauXYSeriesBinding] = []
var config := TauXYConfig.new()

func _ready() -> void:
    dataset = TauPlot.Dataset.new(
        TauPlot.Dataset.Mode.PER_SERIES_X,
        TauPlot.Dataset.XElementType.NUMERIC,
        UINT32_MAX
    )

    var x_axis := TauAxisConfig.new()
    x_axis.title = "Time (s)"
    x_axis.include_zero_in_domain = false

    var y_axis := TauAxisConfig.new()
    y_axis.title = "Value"

    var cfg = TauLineConfig.new()

    var pane := TauPaneConfig.new()
    pane.y_left_axis = y_axis
    pane.overlays = [cfg]

    config.x_axis = x_axis
    config.panes = [pane]

func add_data(time: float, solution: Dictionary):
    for i in solution:
        if i not in variables:
            variables[i] = dataset.add_series(i, 10000)
            var b := TauXYSeriesBinding.new()
            b.series_id = variables[i]
            b.pane_index = 0
            b.overlay_type = TauXYSeriesBinding.PaneOverlayType.LINE
            b.y_axis_id = TauPlot.AxisId.LEFT
            bindings.append(b)

            tau_plot.plot_xy(dataset, config, bindings)
        if solution[i] == null or not is_finite(solution[i]):
            continue
        dataset.append_sample_to_one_series(variables[i], time, solution[i])

func clear():
    variables = {}
    bindings = []
    dataset = TauPlot.Dataset.new(
        TauPlot.Dataset.Mode.PER_SERIES_X,
        TauPlot.Dataset.XElementType.NUMERIC,
        UINT32_MAX
    )
