//! Demo-code for showing how egui is used.
//!
//! This library can be used to test 3rd party egui integrations (see for instance <https://github.com/not-fl3/egui-miniquad/blob/master/examples/demo.rs>).
//!
//! The demo is also used in benchmarks and tests.
//!
//! ## Feature flags
#![cfg_attr(feature = "document-features", doc = document_features::document_features!())]
#![allow(clippy::float_cmp)]
#![allow(clippy::manual_range_contains)]
#![forbid(unsafe_code)]

mod plot_demo {
    use std::f64::consts::TAU;
    use std::ops::RangeInclusive;

    use egui::*;

    use egui_plot::{
        Arrows, AxisBools, AxisHints, Bar, BarChart, BoxElem, BoxPlot, BoxSpread,
        CoordinatesFormatter, Corner, GridInput, GridMark, HLine, Legend, Line, LineStyle,
        MarkerShape, Plot, PlotImage, PlotPoint, PlotPoints, PlotResponse, Points, Polygon, Text,
        VLine,
    };

    // ----------------------------------------------------------------------------

    #[derive(PartialEq, Eq)]
    enum Panel {
        Lines,
        Markers,
        Legend,
        Charts,
        Items,
        Interaction,
        CustomAxes,
        LinkedAxes,
    }

    impl Default for Panel {
        fn default() -> Self {
            Self::Lines
        }
    }

    // ----------------------------------------------------------------------------

    #[derive(PartialEq, Default)]
    pub struct PlotDemo {
        line_demo: LineDemo,
        marker_demo: MarkerDemo,
        legend_demo: LegendDemo,
        charts_demo: ChartsDemo,
        items_demo: ItemsDemo,
        interaction_demo: InteractionDemo,
        custom_axes_demo: CustomAxesDemo,
        linked_axes_demo: LinkedAxesDemo,
        open_panel: Panel,
    }

    impl super::Demo for PlotDemo {
        fn name(&self) -> &'static str {
            "🗠 Plot"
        }

        fn show(&mut self, ctx: &Context, open: &mut bool) {
            use super::View as _;
            Window::new(self.name())
                .open(open)
                .default_size(vec2(400.0, 400.0))
                .vscroll(false)
                .show(ctx, |ui| self.ui(ui));
        }
    }

    impl super::View for PlotDemo {
        fn ui(&mut self, ui: &mut Ui) {
            ui.horizontal(|ui| {
                egui::reset_button(ui, self);
                ui.collapsing("Instructions", |ui| {
                    ui.label("Pan by dragging, or scroll (+ shift = horizontal).");
                    ui.label("Box zooming: Right click to zoom in and zoom out using a selection.");
                    if cfg!(target_arch = "wasm32") {
                        ui.label("Zoom with ctrl / ⌘ + pointer wheel, or with pinch gesture.");
                    } else if cfg!(target_os = "macos") {
                        ui.label("Zoom with ctrl / ⌘ + scroll.");
                    } else {
                        ui.label("Zoom with ctrl + scroll.");
                    }
                    ui.label("Reset view with double-click.");
                    ui.add(crate::egui_github_link_file!());
                });
            });
            ui.separator();
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.open_panel, Panel::Lines, "Lines");
                ui.selectable_value(&mut self.open_panel, Panel::Markers, "Markers");
                ui.selectable_value(&mut self.open_panel, Panel::Legend, "Legend");
                ui.selectable_value(&mut self.open_panel, Panel::Charts, "Charts");
                ui.selectable_value(&mut self.open_panel, Panel::Items, "Items");
                ui.selectable_value(&mut self.open_panel, Panel::Interaction, "Interaction");
                ui.selectable_value(&mut self.open_panel, Panel::CustomAxes, "Custom Axes");
                ui.selectable_value(&mut self.open_panel, Panel::LinkedAxes, "Linked Axes");
            });
            ui.separator();

            match self.open_panel {
                Panel::Lines => {
                    self.line_demo.ui(ui);
                }
                Panel::Markers => {
                    self.marker_demo.ui(ui);
                }
                Panel::Legend => {
                    self.legend_demo.ui(ui);
                }
                Panel::Charts => {
                    self.charts_demo.ui(ui);
                }
                Panel::Items => {
                    self.items_demo.ui(ui);
                }
                Panel::Interaction => {
                    self.interaction_demo.ui(ui);
                }
                Panel::CustomAxes => {
                    self.custom_axes_demo.ui(ui);
                }
                Panel::LinkedAxes => {
                    self.linked_axes_demo.ui(ui);
                }
            }
        }
    }

    // ----------------------------------------------------------------------------

    #[derive(Copy, Clone, PartialEq)]
    struct LineDemo {
        animate: bool,
        time: f64,
        circle_radius: f64,
        circle_center: Pos2,
        square: bool,
        proportional: bool,
        coordinates: bool,
        show_axes: bool,
        show_grid: bool,
        line_style: LineStyle,
    }

    impl Default for LineDemo {
        fn default() -> Self {
            Self {
                animate: !cfg!(debug_assertions),
                time: 0.0,
                circle_radius: 1.5,
                circle_center: Pos2::new(0.0, 0.0),
                square: false,
                proportional: true,
                coordinates: true,
                show_axes: true,
                show_grid: true,
                line_style: LineStyle::Solid,
            }
        }
    }

    impl LineDemo {
        fn options_ui(&mut self, ui: &mut Ui) {
            let Self {
                animate,
                time: _,
                circle_radius,
                circle_center,
                square,
                proportional,
                coordinates,
                show_axes,
                show_grid,
                line_style,
            } = self;

            ui.horizontal(|ui| {
                ui.group(|ui| {
                    ui.vertical(|ui| {
                        ui.label("Circle:");
                        ui.add(
                            egui::DragValue::new(circle_radius)
                                .speed(0.1)
                                .clamp_range(0.0..=f64::INFINITY)
                                .prefix("r: "),
                        );
                        ui.horizontal(|ui| {
                            ui.add(
                                egui::DragValue::new(&mut circle_center.x)
                                    .speed(0.1)
                                    .prefix("x: "),
                            );
                            ui.add(
                                egui::DragValue::new(&mut circle_center.y)
                                    .speed(1.0)
                                    .prefix("y: "),
                            );
                        });
                    });
                });

                ui.vertical(|ui| {
                    ui.checkbox(show_axes, "Show axes");
                    ui.checkbox(show_grid, "Show grid");
                    ui.checkbox(coordinates, "Show coordinates on hover")
                        .on_hover_text("Can take a custom formatting function.");
                });

                ui.vertical(|ui| {
                    ui.style_mut().wrap = Some(false);
                    ui.checkbox(animate, "Animate");
                    ui.checkbox(square, "Square view")
                        .on_hover_text("Always keep the viewport square.");
                    ui.checkbox(proportional, "Proportional data axes")
                        .on_hover_text("Tick are the same size on both axes.");

                    ComboBox::from_label("Line style")
                        .selected_text(line_style.to_string())
                        .show_ui(ui, |ui| {
                            for style in &[
                                LineStyle::Solid,
                                LineStyle::dashed_dense(),
                                LineStyle::dashed_loose(),
                                LineStyle::dotted_dense(),
                                LineStyle::dotted_loose(),
                            ] {
                                ui.selectable_value(line_style, *style, style.to_string());
                            }
                        });
                });
            });
        }

        fn circle(&self) -> Line {
            let n = 512;
            let circle_points: PlotPoints = (0..=n)
                .map(|i| {
                    let t = remap(i as f64, 0.0..=(n as f64), 0.0..=TAU);
                    let r = self.circle_radius;
                    [
                        r * t.cos() + self.circle_center.x as f64,
                        r * t.sin() + self.circle_center.y as f64,
                    ]
                })
                .collect();
            Line::new(circle_points)
                .color(Color32::from_rgb(100, 200, 100))
                .style(self.line_style)
                .name("circle")
        }

        fn sin(&self) -> Line {
            let time = self.time;
            Line::new(PlotPoints::from_explicit_callback(
                move |x| 0.5 * (2.0 * x).sin() * time.sin(),
                ..,
                512,
            ))
            .color(Color32::from_rgb(200, 100, 100))
            .style(self.line_style)
            .name("wave")
        }

        fn thingy(&self) -> Line {
            let time = self.time;
            Line::new(PlotPoints::from_parametric_callback(
                move |t| ((2.0 * t + time).sin(), (3.0 * t).sin()),
                0.0..=TAU,
                256,
            ))
            .color(Color32::from_rgb(100, 150, 250))
            .style(self.line_style)
            .name("x = sin(2t), y = sin(3t)")
        }
    }

    impl LineDemo {
        fn ui(&mut self, ui: &mut Ui) -> Response {
            self.options_ui(ui);

            if self.animate {
                ui.ctx().request_repaint();
                self.time += ui.input(|i| i.unstable_dt).at_most(1.0 / 30.0) as f64;
            };
            let mut plot = Plot::new("lines_demo")
                .legend(Legend::default())
                .y_axis_width(4)
                .show_axes(self.show_axes)
                .show_grid(self.show_grid);
            if self.square {
                plot = plot.view_aspect(1.0);
            }
            if self.proportional {
                plot = plot.data_aspect(1.0);
            }
            if self.coordinates {
                plot =
                    plot.coordinates_formatter(Corner::LeftBottom, CoordinatesFormatter::default());
            }
            plot.show(ui, |plot_ui| {
                plot_ui.line(self.circle());
                plot_ui.line(self.sin());
                plot_ui.line(self.thingy());
            })
            .response
        }
    }

    // ----------------------------------------------------------------------------

    #[derive(PartialEq)]
    struct MarkerDemo {
        fill_markers: bool,
        marker_radius: f32,
        automatic_colors: bool,
        marker_color: Color32,
    }

    impl Default for MarkerDemo {
        fn default() -> Self {
            Self {
                fill_markers: true,
                marker_radius: 5.0,
                automatic_colors: true,
                marker_color: Color32::GREEN,
            }
        }
    }

    impl MarkerDemo {
        fn markers(&self) -> Vec<Points> {
            MarkerShape::all()
                .enumerate()
                .map(|(i, marker)| {
                    let y_offset = i as f64 * 0.5 + 1.0;
                    let mut points = Points::new(vec![
                        [1.0, 0.0 + y_offset],
                        [2.0, 0.5 + y_offset],
                        [3.0, 0.0 + y_offset],
                        [4.0, 0.5 + y_offset],
                        [5.0, 0.0 + y_offset],
                        [6.0, 0.5 + y_offset],
                    ])
                    .name(format!("{marker:?}"))
                    .filled(self.fill_markers)
                    .radius(self.marker_radius)
                    .shape(marker);

                    if !self.automatic_colors {
                        points = points.color(self.marker_color);
                    }

                    points
                })
                .collect()
        }

        fn ui(&mut self, ui: &mut Ui) -> Response {
            ui.horizontal(|ui| {
                ui.checkbox(&mut self.fill_markers, "Fill");
                ui.add(
                    egui::DragValue::new(&mut self.marker_radius)
                        .speed(0.1)
                        .clamp_range(0.0..=f64::INFINITY)
                        .prefix("Radius: "),
                );
                ui.checkbox(&mut self.automatic_colors, "Automatic colors");
                if !self.automatic_colors {
                    ui.color_edit_button_srgba(&mut self.marker_color);
                }
            });

            let markers_plot = Plot::new("markers_demo")
                .data_aspect(1.0)
                .legend(Legend::default());
            markers_plot
                .show(ui, |plot_ui| {
                    for marker in self.markers() {
                        plot_ui.points(marker);
                    }
                })
                .response
        }
    }

    // ----------------------------------------------------------------------------

    #[derive(Default, PartialEq)]
    struct LegendDemo {
        config: Legend,
    }

    impl LegendDemo {
        fn line_with_slope(slope: f64) -> Line {
            Line::new(PlotPoints::from_explicit_callback(
                move |x| slope * x,
                ..,
                100,
            ))
        }

        fn sin() -> Line {
            Line::new(PlotPoints::from_explicit_callback(
                move |x| x.sin(),
                ..,
                100,
            ))
        }

        fn cos() -> Line {
            Line::new(PlotPoints::from_explicit_callback(
                move |x| x.cos(),
                ..,
                100,
            ))
        }

        fn ui(&mut self, ui: &mut Ui) -> Response {
            let LegendDemo { config } = self;

            egui::Grid::new("settings").show(ui, |ui| {
                ui.label("Text style:");
                ui.horizontal(|ui| {
                    let all_text_styles = ui.style().text_styles();
                    for style in all_text_styles {
                        ui.selectable_value(
                            &mut config.text_style,
                            style.clone(),
                            style.to_string(),
                        );
                    }
                });
                ui.end_row();

                ui.label("Position:");
                ui.horizontal(|ui| {
                    Corner::all().for_each(|position| {
                        ui.selectable_value(
                            &mut config.position,
                            position,
                            format!("{position:?}"),
                        );
                    });
                });
                ui.end_row();

                ui.label("Opacity:");
                ui.add(
                    egui::DragValue::new(&mut config.background_alpha)
                        .speed(0.02)
                        .clamp_range(0.0..=1.0),
                );
                ui.end_row();
            });
            let legend_plot = Plot::new("legend_demo")
                .y_axis_width(2)
                .legend(config.clone())
                .data_aspect(1.0);
            legend_plot
                .show(ui, |plot_ui| {
                    plot_ui.line(LegendDemo::line_with_slope(0.5).name("lines"));
                    plot_ui.line(LegendDemo::line_with_slope(1.0).name("lines"));
                    plot_ui.line(LegendDemo::line_with_slope(2.0).name("lines"));
                    plot_ui.line(LegendDemo::sin().name("sin(x)"));
                    plot_ui.line(LegendDemo::cos().name("cos(x)"));
                })
                .response
        }
    }

    // ----------------------------------------------------------------------------

    #[derive(PartialEq, Default)]
    struct CustomAxesDemo {}

    impl CustomAxesDemo {
        const MINS_PER_DAY: f64 = 24.0 * 60.0;
        const MINS_PER_H: f64 = 60.0;

        fn logistic_fn() -> Line {
            fn days(min: f64) -> f64 {
                CustomAxesDemo::MINS_PER_DAY * min
            }

            let values = PlotPoints::from_explicit_callback(
                move |x| 1.0 / (1.0 + (-2.5 * (x / CustomAxesDemo::MINS_PER_DAY - 2.0)).exp()),
                days(0.0)..days(5.0),
                100,
            );
            Line::new(values)
        }

        #[allow(clippy::needless_pass_by_value)]
        fn x_grid(input: GridInput) -> Vec<GridMark> {
            // Note: this always fills all possible marks. For optimization, `input.bounds`
            // could be used to decide when the low-interval grids (minutes) should be added.

            let mut marks = vec![];

            let (min, max) = input.bounds;
            let min = min.floor() as i32;
            let max = max.ceil() as i32;

            for i in min..=max {
                let step_size = if i % Self::MINS_PER_DAY as i32 == 0 {
                    // 1 day
                    Self::MINS_PER_DAY
                } else if i % Self::MINS_PER_H as i32 == 0 {
                    // 1 hour
                    Self::MINS_PER_H
                } else if i % 5 == 0 {
                    // 5min
                    5.0
                } else {
                    // skip grids below 5min
                    continue;
                };

                marks.push(GridMark {
                    value: i as f64,
                    step_size,
                });
            }

            marks
        }

        #[allow(clippy::unused_self)]
        fn ui(&mut self, ui: &mut Ui) -> Response {
            const MINS_PER_DAY: f64 = CustomAxesDemo::MINS_PER_DAY;
            const MINS_PER_H: f64 = CustomAxesDemo::MINS_PER_H;

            fn day(x: f64) -> f64 {
                (x / MINS_PER_DAY).floor()
            }

            fn hour(x: f64) -> f64 {
                (x.rem_euclid(MINS_PER_DAY) / MINS_PER_H).floor()
            }

            fn minute(x: f64) -> f64 {
                x.rem_euclid(MINS_PER_H).floor()
            }

            fn percent(y: f64) -> f64 {
                100.0 * y
            }

            let x_fmt = |x, _digits, _range: &RangeInclusive<f64>| {
                if x < 0.0 * MINS_PER_DAY || x >= 5.0 * MINS_PER_DAY {
                    // No labels outside value bounds
                    String::new()
                } else if is_approx_integer(x / MINS_PER_DAY) {
                    // Days
                    format!("Day {}", day(x))
                } else {
                    // Hours and minutes
                    format!("{h}:{m:02}", h = hour(x), m = minute(x))
                }
            };

            let y_fmt = |y, _digits, _range: &RangeInclusive<f64>| {
                // Display only integer percentages
                if !is_approx_zero(y) && is_approx_integer(100.0 * y) {
                    format!("{:.0}%", percent(y))
                } else {
                    String::new()
                }
            };

            let label_fmt = |_s: &str, val: &PlotPoint| {
                format!(
                    "Day {d}, {h}:{m:02}\n{p:.2}%",
                    d = day(val.x),
                    h = hour(val.x),
                    m = minute(val.x),
                    p = percent(val.y)
                )
            };

            ui.label("Zoom in on the X-axis to see hours and minutes");

            let x_axes = vec![
                AxisHints::default().label("Time").formatter(x_fmt),
                AxisHints::default().label("Value"),
            ];
            let y_axes = vec![
                AxisHints::default()
                    .label("Percent")
                    .formatter(y_fmt)
                    .max_digits(4),
                AxisHints::default()
                    .label("Absolute")
                    .placement(egui_plot::HPlacement::Right),
            ];
            Plot::new("custom_axes")
                .data_aspect(2.0 * MINS_PER_DAY as f32)
                .custom_x_axes(x_axes)
                .custom_y_axes(y_axes)
                .x_grid_spacer(CustomAxesDemo::x_grid)
                .label_formatter(label_fmt)
                .show(ui, |plot_ui| {
                    plot_ui.line(CustomAxesDemo::logistic_fn());
                })
                .response
        }
    }

    // ----------------------------------------------------------------------------

    #[derive(PartialEq)]
    struct LinkedAxesDemo {
        link_x: bool,
        link_y: bool,
        link_cursor_x: bool,
        link_cursor_y: bool,
    }

    impl Default for LinkedAxesDemo {
        fn default() -> Self {
            Self {
                link_x: true,
                link_y: true,
                link_cursor_x: true,
                link_cursor_y: true,
            }
        }
    }

    impl LinkedAxesDemo {
        fn line_with_slope(slope: f64) -> Line {
            Line::new(PlotPoints::from_explicit_callback(
                move |x| slope * x,
                ..,
                100,
            ))
        }

        fn sin() -> Line {
            Line::new(PlotPoints::from_explicit_callback(
                move |x| x.sin(),
                ..,
                100,
            ))
        }

        fn cos() -> Line {
            Line::new(PlotPoints::from_explicit_callback(
                move |x| x.cos(),
                ..,
                100,
            ))
        }

        fn configure_plot(plot_ui: &mut egui_plot::PlotUi) {
            plot_ui.line(LinkedAxesDemo::line_with_slope(0.5));
            plot_ui.line(LinkedAxesDemo::line_with_slope(1.0));
            plot_ui.line(LinkedAxesDemo::line_with_slope(2.0));
            plot_ui.line(LinkedAxesDemo::sin());
            plot_ui.line(LinkedAxesDemo::cos());
        }

        fn ui(&mut self, ui: &mut Ui) -> Response {
            ui.horizontal(|ui| {
                ui.label("Linked axes:");
                ui.checkbox(&mut self.link_x, "X");
                ui.checkbox(&mut self.link_y, "Y");
            });
            ui.horizontal(|ui| {
                ui.label("Linked cursors:");
                ui.checkbox(&mut self.link_cursor_x, "X");
                ui.checkbox(&mut self.link_cursor_y, "Y");
            });

            let link_group_id = ui.id().with("linked_demo");
            ui.horizontal(|ui| {
                Plot::new("left-top")
                    .data_aspect(1.0)
                    .width(250.0)
                    .height(250.0)
                    .link_axis(link_group_id, self.link_x, self.link_y)
                    .link_cursor(link_group_id, self.link_cursor_x, self.link_cursor_y)
                    .show(ui, LinkedAxesDemo::configure_plot);
                Plot::new("right-top")
                    .data_aspect(2.0)
                    .width(150.0)
                    .height(250.0)
                    .y_axis_width(3)
                    .y_axis_label("y")
                    .y_axis_position(egui_plot::HPlacement::Right)
                    .link_axis(link_group_id, self.link_x, self.link_y)
                    .link_cursor(link_group_id, self.link_cursor_x, self.link_cursor_y)
                    .show(ui, LinkedAxesDemo::configure_plot);
            });
            Plot::new("left-bottom")
                .data_aspect(0.5)
                .width(250.0)
                .height(150.0)
                .x_axis_label("x")
                .link_axis(link_group_id, self.link_x, self.link_y)
                .link_cursor(link_group_id, self.link_cursor_x, self.link_cursor_y)
                .show(ui, LinkedAxesDemo::configure_plot)
                .response
        }
    }

    // ----------------------------------------------------------------------------

    #[derive(PartialEq, Default)]
    struct ItemsDemo {
        texture: Option<egui::TextureHandle>,
    }

    impl ItemsDemo {
        fn ui(&mut self, ui: &mut Ui) -> Response {
            let n = 100;
            let mut sin_values: Vec<_> = (0..=n)
                .map(|i| remap(i as f64, 0.0..=n as f64, -TAU..=TAU))
                .map(|i| [i, i.sin()])
                .collect();

            let line = Line::new(sin_values.split_off(n / 2)).fill(-1.5);
            let polygon = Polygon::new(PlotPoints::from_parametric_callback(
                |t| (4.0 * t.sin() + 2.0 * t.cos(), 4.0 * t.cos() + 2.0 * t.sin()),
                0.0..TAU,
                100,
            ));
            let points = Points::new(sin_values).stems(-1.5).radius(1.0);

            let arrows = {
                let pos_radius = 8.0;
                let tip_radius = 7.0;
                let arrow_origins = PlotPoints::from_parametric_callback(
                    |t| (pos_radius * t.sin(), pos_radius * t.cos()),
                    0.0..TAU,
                    36,
                );
                let arrow_tips = PlotPoints::from_parametric_callback(
                    |t| (tip_radius * t.sin(), tip_radius * t.cos()),
                    0.0..TAU,
                    36,
                );
                Arrows::new(arrow_origins, arrow_tips)
            };

            let texture: &egui::TextureHandle = self.texture.get_or_insert_with(|| {
                ui.ctx()
                    .load_texture("plot_demo", egui::ColorImage::example(), Default::default())
            });
            let image = PlotImage::new(
                texture,
                PlotPoint::new(0.0, 10.0),
                5.0 * vec2(texture.aspect_ratio(), 1.0),
            );

            let plot = Plot::new("items_demo")
                .legend(Legend::default().position(Corner::RightBottom))
                .show_x(false)
                .show_y(false)
                .data_aspect(1.0);
            plot.show(ui, |plot_ui| {
                plot_ui.hline(HLine::new(9.0).name("Lines horizontal"));
                plot_ui.hline(HLine::new(-9.0).name("Lines horizontal"));
                plot_ui.vline(VLine::new(9.0).name("Lines vertical"));
                plot_ui.vline(VLine::new(-9.0).name("Lines vertical"));
                plot_ui.line(line.name("Line with fill"));
                plot_ui.polygon(polygon.name("Convex polygon"));
                plot_ui.points(points.name("Points with stems"));
                plot_ui.text(Text::new(PlotPoint::new(-3.0, -3.0), "wow").name("Text"));
                plot_ui.text(Text::new(PlotPoint::new(-2.0, 2.5), "so graph").name("Text"));
                plot_ui.text(Text::new(PlotPoint::new(3.0, 3.0), "much color").name("Text"));
                plot_ui.text(Text::new(PlotPoint::new(2.5, -2.0), "such plot").name("Text"));
                plot_ui.image(image.name("Image"));
                plot_ui.arrows(arrows.name("Arrows"));
            })
            .response
        }
    }

    // ----------------------------------------------------------------------------

    #[derive(Default, PartialEq)]
    struct InteractionDemo {}

    impl InteractionDemo {
        #[allow(clippy::unused_self)]
        fn ui(&mut self, ui: &mut Ui) -> Response {
            let plot = Plot::new("interaction_demo").height(300.0);

            let PlotResponse {
                response,
                inner:
                    (screen_pos, pointer_coordinate, pointer_coordinate_drag_delta, bounds, hovered),
                ..
            } = plot.show(ui, |plot_ui| {
                (
                    plot_ui.screen_from_plot(PlotPoint::new(0.0, 0.0)),
                    plot_ui.pointer_coordinate(),
                    plot_ui.pointer_coordinate_drag_delta(),
                    plot_ui.plot_bounds(),
                    plot_ui.response().hovered(),
                )
            });

            ui.label(format!(
                "plot bounds: min: {:.02?}, max: {:.02?}",
                bounds.min(),
                bounds.max()
            ));
            ui.label(format!(
                "origin in screen coordinates: x: {:.02}, y: {:.02}",
                screen_pos.x, screen_pos.y
            ));
            ui.label(format!("plot hovered: {hovered}"));
            let coordinate_text = if let Some(coordinate) = pointer_coordinate {
                format!("x: {:.02}, y: {:.02}", coordinate.x, coordinate.y)
            } else {
                "None".to_owned()
            };
            ui.label(format!("pointer coordinate: {coordinate_text}"));
            let coordinate_text = format!(
                "x: {:.02}, y: {:.02}",
                pointer_coordinate_drag_delta.x, pointer_coordinate_drag_delta.y
            );
            ui.label(format!("pointer coordinate drag delta: {coordinate_text}"));

            response
        }
    }

    // ----------------------------------------------------------------------------

    #[derive(PartialEq, Eq)]
    enum Chart {
        GaussBars,
        StackedBars,
        BoxPlot,
    }

    impl Default for Chart {
        fn default() -> Self {
            Self::GaussBars
        }
    }

    #[derive(PartialEq)]
    struct ChartsDemo {
        chart: Chart,
        vertical: bool,
        allow_zoom: AxisBools,
        allow_drag: AxisBools,
    }

    impl Default for ChartsDemo {
        fn default() -> Self {
            Self {
                vertical: true,
                chart: Chart::default(),
                allow_zoom: true.into(),
                allow_drag: true.into(),
            }
        }
    }

    impl ChartsDemo {
        fn ui(&mut self, ui: &mut Ui) -> Response {
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.label("Type:");
                    ui.horizontal(|ui| {
                        ui.selectable_value(&mut self.chart, Chart::GaussBars, "Histogram");
                        ui.selectable_value(
                            &mut self.chart,
                            Chart::StackedBars,
                            "Stacked Bar Chart",
                        );
                        ui.selectable_value(&mut self.chart, Chart::BoxPlot, "Box Plot");
                    });
                    ui.label("Orientation:");
                    ui.horizontal(|ui| {
                        ui.selectable_value(&mut self.vertical, true, "Vertical");
                        ui.selectable_value(&mut self.vertical, false, "Horizontal");
                    });
                });
                ui.vertical(|ui| {
                    ui.group(|ui| {
                        ui.add_enabled_ui(self.chart != Chart::StackedBars, |ui| {
                            ui.horizontal(|ui| {
                                ui.label("Allow zoom:");
                                ui.checkbox(&mut self.allow_zoom.x, "X");
                                ui.checkbox(&mut self.allow_zoom.y, "Y");
                            });
                        });
                        ui.horizontal(|ui| {
                            ui.label("Allow drag:");
                            ui.checkbox(&mut self.allow_drag.x, "X");
                            ui.checkbox(&mut self.allow_drag.y, "Y");
                        });
                    });
                });
            });
            match self.chart {
                Chart::GaussBars => self.bar_gauss(ui),
                Chart::StackedBars => self.bar_stacked(ui),
                Chart::BoxPlot => self.box_plot(ui),
            }
        }

        fn bar_gauss(&self, ui: &mut Ui) -> Response {
            let mut chart = BarChart::new(
                (-395..=395)
                    .step_by(10)
                    .map(|x| x as f64 * 0.01)
                    .map(|x| {
                        (
                            x,
                            (-x * x / 2.0).exp() / (2.0 * std::f64::consts::PI).sqrt(),
                        )
                    })
                    // The 10 factor here is purely for a nice 1:1 aspect ratio
                    .map(|(x, f)| Bar::new(x, f * 10.0).width(0.095))
                    .collect(),
            )
            .color(Color32::LIGHT_BLUE)
            .name("Normal Distribution");
            if !self.vertical {
                chart = chart.horizontal();
            }

            Plot::new("Normal Distribution Demo")
                .legend(Legend::default())
                .clamp_grid(true)
                .y_axis_width(3)
                .allow_zoom(self.allow_zoom)
                .allow_drag(self.allow_drag)
                .show(ui, |plot_ui| plot_ui.bar_chart(chart))
                .response
        }

        fn bar_stacked(&self, ui: &mut Ui) -> Response {
            let mut chart1 = BarChart::new(vec![
                Bar::new(0.5, 1.0).name("Day 1"),
                Bar::new(1.5, 3.0).name("Day 2"),
                Bar::new(2.5, 1.0).name("Day 3"),
                Bar::new(3.5, 2.0).name("Day 4"),
                Bar::new(4.5, 4.0).name("Day 5"),
            ])
            .width(0.7)
            .name("Set 1");

            let mut chart2 = BarChart::new(vec![
                Bar::new(0.5, 1.0),
                Bar::new(1.5, 1.5),
                Bar::new(2.5, 0.1),
                Bar::new(3.5, 0.7),
                Bar::new(4.5, 0.8),
            ])
            .width(0.7)
            .name("Set 2")
            .stack_on(&[&chart1]);

            let mut chart3 = BarChart::new(vec![
                Bar::new(0.5, -0.5),
                Bar::new(1.5, 1.0),
                Bar::new(2.5, 0.5),
                Bar::new(3.5, -1.0),
                Bar::new(4.5, 0.3),
            ])
            .width(0.7)
            .name("Set 3")
            .stack_on(&[&chart1, &chart2]);

            let mut chart4 = BarChart::new(vec![
                Bar::new(0.5, 0.5),
                Bar::new(1.5, 1.0),
                Bar::new(2.5, 0.5),
                Bar::new(3.5, -0.5),
                Bar::new(4.5, -0.5),
            ])
            .width(0.7)
            .name("Set 4")
            .stack_on(&[&chart1, &chart2, &chart3]);

            if !self.vertical {
                chart1 = chart1.horizontal();
                chart2 = chart2.horizontal();
                chart3 = chart3.horizontal();
                chart4 = chart4.horizontal();
            }

            Plot::new("Stacked Bar Chart Demo")
                .legend(Legend::default())
                .data_aspect(1.0)
                .allow_drag(self.allow_drag)
                .show(ui, |plot_ui| {
                    plot_ui.bar_chart(chart1);
                    plot_ui.bar_chart(chart2);
                    plot_ui.bar_chart(chart3);
                    plot_ui.bar_chart(chart4);
                })
                .response
        }

        fn box_plot(&self, ui: &mut Ui) -> Response {
            let yellow = Color32::from_rgb(248, 252, 168);
            let mut box1 = BoxPlot::new(vec![
                BoxElem::new(0.5, BoxSpread::new(1.5, 2.2, 2.5, 2.6, 3.1)).name("Day 1"),
                BoxElem::new(2.5, BoxSpread::new(0.4, 1.0, 1.1, 1.4, 2.1)).name("Day 2"),
                BoxElem::new(4.5, BoxSpread::new(1.7, 2.0, 2.2, 2.5, 2.9)).name("Day 3"),
            ])
            .name("Experiment A");

            let mut box2 = BoxPlot::new(vec![
                BoxElem::new(1.0, BoxSpread::new(0.2, 0.5, 1.0, 2.0, 2.7)).name("Day 1"),
                BoxElem::new(3.0, BoxSpread::new(1.5, 1.7, 2.1, 2.9, 3.3))
                    .name("Day 2: interesting")
                    .stroke(Stroke::new(1.5, yellow))
                    .fill(yellow.linear_multiply(0.2)),
                BoxElem::new(5.0, BoxSpread::new(1.3, 2.0, 2.3, 2.9, 4.0)).name("Day 3"),
            ])
            .name("Experiment B");

            let mut box3 = BoxPlot::new(vec![
                BoxElem::new(1.5, BoxSpread::new(2.1, 2.2, 2.6, 2.8, 3.0)).name("Day 1"),
                BoxElem::new(3.5, BoxSpread::new(1.3, 1.5, 1.9, 2.2, 2.4)).name("Day 2"),
                BoxElem::new(5.5, BoxSpread::new(0.2, 0.4, 1.0, 1.3, 1.5)).name("Day 3"),
            ])
            .name("Experiment C");

            if !self.vertical {
                box1 = box1.horizontal();
                box2 = box2.horizontal();
                box3 = box3.horizontal();
            }

            Plot::new("Box Plot Demo")
                .legend(Legend::default())
                .allow_zoom(self.allow_zoom)
                .allow_drag(self.allow_drag)
                .show(ui, |plot_ui| {
                    plot_ui.box_plot(box1);
                    plot_ui.box_plot(box2);
                    plot_ui.box_plot(box3);
                })
                .response
        }
    }

    fn is_approx_zero(val: f64) -> bool {
        val.abs() < 1e-6
    }

    fn is_approx_integer(val: f64) -> bool {
        val.fract().abs() < 1e-6
    }
}

pub use color_test::ColorTest;
pub use demo::DemoWindows;
use plot_demo::PlotDemo;

/// View some Rust code with syntax highlighting and selection.
pub(crate) fn rust_view_ui(ui: &mut egui::Ui, code: &str) {
    let language = "rs";
    let theme = egui_extras::syntax_highlighting::CodeTheme::from_memory(ui.ctx());
    egui_extras::syntax_highlighting::code_view_ui(ui, &theme, code, language);
}

// ----------------------------------------------------------------------------

/// Create a [`Hyperlink`](egui::Hyperlink) to this egui source code file on github.
#[macro_export]
macro_rules! egui_github_link_file {
    () => {
        $crate::egui_github_link_file!("(source code)")
    };
    ($label: expr) => {
        egui::github_link_file!(
            "https://github.com/emilk/egui/blob/master/",
            egui::RichText::new($label).small()
        )
    };
}

/// Create a [`Hyperlink`](egui::Hyperlink) to this egui source code file and line on github.
#[macro_export]
macro_rules! egui_github_link_file_line {
    () => {
        $crate::egui_github_link_file_line!("(source code)")
    };
    ($label: expr) => {
        egui::github_link_file_line!(
            "https://github.com/emilk/egui/blob/master/",
            egui::RichText::new($label).small()
        )
    };
}

// ----------------------------------------------------------------------------

pub const LOREM_IPSUM: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";

pub const LOREM_IPSUM_LONG: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.

Curabitur pretium tincidunt lacus. Nulla gravida orci a odio. Nullam various, turpis et commodo pharetra, est eros bibendum elit, nec luctus magna felis sollicitudin mauris. Integer in mauris eu nibh euismod gravida. Duis ac tellus et risus vulputate vehicula. Donec lobortis risus a elit. Etiam tempor. Ut ullamcorper, ligula eu tempor congue, eros est euismod turpis, id tincidunt sapien risus a quam. Maecenas fermentum consequat mi. Donec fermentum. Pellentesque malesuada nulla a mi. Duis sapien sem, aliquet nec, commodo eget, consequat quis, neque. Aliquam faucibus, elit ut dictum aliquet, felis nisl adipiscing sapien, sed malesuada diam lacus eget erat. Cras mollis scelerisque nunc. Nullam arcu. Aliquam consequat. Curabitur augue lorem, dapibus quis, laoreet et, pretium ac, nisi. Aenean magna nisl, mollis quis, molestie eu, feugiat in, orci. In hac habitasse platea dictumst.";

// ----------------------------------------------------------------------------

#[test]
fn test_egui_e2e() {
    let mut demo_windows = crate::DemoWindows::default();
    let ctx = egui::Context::default();
    let raw_input = egui::RawInput::default();

    const NUM_FRAMES: usize = 5;
    for _ in 0..NUM_FRAMES {
        let full_output = ctx.run(raw_input.clone(), |ctx| {
            demo_windows.ui(ctx);
        });
        let clipped_primitives = ctx.tessellate(full_output.shapes);
        assert!(!clipped_primitives.is_empty());
    }
}

#[test]
fn test_egui_zero_window_size() {
    let mut demo_windows = crate::DemoWindows::default();
    let ctx = egui::Context::default();
    let raw_input = egui::RawInput {
        screen_rect: Some(egui::Rect::from_min_max(egui::Pos2::ZERO, egui::Pos2::ZERO)),
        ..Default::default()
    };

    const NUM_FRAMES: usize = 5;
    for _ in 0..NUM_FRAMES {
        let full_output = ctx.run(raw_input.clone(), |ctx| {
            demo_windows.ui(ctx);
        });
        let clipped_primitives = ctx.tessellate(full_output.shapes);
        assert!(
            clipped_primitives.is_empty(),
            "There should be nothing to show, has at least one primitive with clip_rect: {:?}",
            clipped_primitives[0].clip_rect
        );
    }
}

// ----------------------------------------------------------------------------

/// Detect narrow screens. This is used to show a simpler UI on mobile devices,
/// especially for the web demo at <https://egui.rs>.
pub fn is_mobile(ctx: &egui::Context) -> bool {
    let screen_size = ctx.screen_rect().size();
    screen_size.x < 550.0
}

mod color_test {
    use std::collections::HashMap;

    use egui::{widgets::color_picker::show_color, TextureOptions, *};

    const GRADIENT_SIZE: Vec2 = vec2(256.0, 18.0);

    const BLACK: Color32 = Color32::BLACK;
    const GREEN: Color32 = Color32::GREEN;
    const RED: Color32 = Color32::RED;
    const TRANSPARENT: Color32 = Color32::TRANSPARENT;
    const WHITE: Color32 = Color32::WHITE;

    /// A test for sanity-checking and diagnosing egui rendering backends.
    #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
    pub struct ColorTest {
        #[cfg_attr(feature = "serde", serde(skip))]
        tex_mngr: TextureManager,
        vertex_gradients: bool,
        texture_gradients: bool,
    }

    impl Default for ColorTest {
        fn default() -> Self {
            Self {
                tex_mngr: Default::default(),
                vertex_gradients: true,
                texture_gradients: true,
            }
        }
    }

    impl ColorTest {
        pub fn ui(&mut self, ui: &mut Ui) {
            ui.set_max_width(680.0);

            ui.vertical_centered(|ui| {
                ui.add(crate::egui_github_link_file!());
            });

            ui.horizontal_wrapped(|ui|{
            ui.label("This is made to test that the egui painter backend is set up correctly.");
            ui.add(egui::Label::new("❓").sense(egui::Sense::click()))
                .on_hover_text("The texture sampling should be sRGB-aware, and every other color operation should be done in gamma-space (sRGB). All colors should use pre-multiplied alpha");
        });
            ui.label("If the rendering is done right, all groups of gradients will look uniform.");

            ui.horizontal(|ui| {
                ui.checkbox(&mut self.vertex_gradients, "Vertex gradients");
                ui.checkbox(&mut self.texture_gradients, "Texture gradients");
            });

            ui.heading("sRGB color test");
            ui.label("Use a color picker to ensure this color is (255, 165, 0) / #ffa500");
            ui.scope(|ui| {
                ui.spacing_mut().item_spacing.y = 0.0; // No spacing between gradients
                let g = Gradient::one_color(Color32::from_rgb(255, 165, 0));
                self.vertex_gradient(ui, "orange rgb(255, 165, 0) - vertex", WHITE, &g);
                self.tex_gradient(ui, "orange rgb(255, 165, 0) - texture", WHITE, &g);
            });

            ui.separator();

            ui.label("Test that vertex color times texture color is done in gamma space:");
            ui.scope(|ui| {
                ui.spacing_mut().item_spacing.y = 0.0; // No spacing between gradients

                let tex_color = Color32::from_rgb(64, 128, 255);
                let vertex_color = Color32::from_rgb(128, 196, 196);
                let ground_truth = mul_color_gamma(tex_color, vertex_color);

                ui.horizontal(|ui| {
                    let color_size = ui.spacing().interact_size;
                    ui.label("texture");
                    show_color(ui, tex_color, color_size);
                    ui.label(" * ");
                    show_color(ui, vertex_color, color_size);
                    ui.label(" vertex color =");
                });
                {
                    let g = Gradient::one_color(ground_truth);
                    self.vertex_gradient(ui, "Ground truth (vertices)", WHITE, &g);
                    self.tex_gradient(ui, "Ground truth (texture)", WHITE, &g);
                }

                ui.horizontal(|ui| {
                    let g = Gradient::one_color(tex_color);
                    let tex = self.tex_mngr.get(ui.ctx(), &g);
                    let texel_offset = 0.5 / (g.0.len() as f32);
                    let uv =
                        Rect::from_min_max(pos2(texel_offset, 0.0), pos2(1.0 - texel_offset, 1.0));
                    ui.add(
                        Image::from_texture((tex.id(), GRADIENT_SIZE))
                            .tint(vertex_color)
                            .uv(uv),
                    )
                    .on_hover_text(format!("A texture that is {} texels wide", g.0.len()));
                    ui.label("GPU result");
                });
            });

            ui.separator();

            // TODO(emilk): test color multiplication (image tint),
            // to make sure vertex and texture color multiplication is done in linear space.

            ui.label("Gamma interpolation:");
            self.show_gradients(ui, WHITE, (RED, GREEN), Interpolation::Gamma);

            ui.separator();

            self.show_gradients(ui, RED, (TRANSPARENT, GREEN), Interpolation::Gamma);

            ui.separator();

            self.show_gradients(ui, WHITE, (TRANSPARENT, GREEN), Interpolation::Gamma);

            ui.separator();

            self.show_gradients(ui, BLACK, (BLACK, WHITE), Interpolation::Gamma);
            ui.separator();
            self.show_gradients(ui, WHITE, (BLACK, TRANSPARENT), Interpolation::Gamma);
            ui.separator();
            self.show_gradients(ui, BLACK, (TRANSPARENT, WHITE), Interpolation::Gamma);
            ui.separator();

            ui.label("Additive blending: add more and more blue to the red background:");
            self.show_gradients(
                ui,
                RED,
                (TRANSPARENT, Color32::from_rgb_additive(0, 0, 255)),
                Interpolation::Gamma,
            );

            ui.separator();

            ui.label("Linear interpolation (texture sampling):");
            self.show_gradients(ui, WHITE, (RED, GREEN), Interpolation::Linear);

            ui.separator();

            pixel_test(ui);

            ui.separator();
            ui.label("Testing text rendering:");

            text_on_bg(ui, Color32::from_gray(200), Color32::from_gray(230)); // gray on gray
            text_on_bg(ui, Color32::from_gray(140), Color32::from_gray(28)); // dark mode normal text

            // Matches Mac Font book (useful for testing):
            text_on_bg(ui, Color32::from_gray(39), Color32::from_gray(255));
            text_on_bg(ui, Color32::from_gray(220), Color32::from_gray(30));

            ui.separator();

            blending_and_feathering_test(ui);
        }

        fn show_gradients(
            &mut self,
            ui: &mut Ui,
            bg_fill: Color32,
            (left, right): (Color32, Color32),
            interpolation: Interpolation,
        ) {
            let is_opaque = left.is_opaque() && right.is_opaque();

            ui.horizontal(|ui| {
                let color_size = ui.spacing().interact_size;
                if !is_opaque {
                    ui.label("Background:");
                    show_color(ui, bg_fill, color_size);
                }
                ui.label("gradient");
                show_color(ui, left, color_size);
                ui.label("-");
                show_color(ui, right, color_size);
            });

            ui.scope(|ui| {
                ui.spacing_mut().item_spacing.y = 0.0; // No spacing between gradients
                if is_opaque {
                    let g = Gradient::ground_truth_gradient(left, right, interpolation);
                    self.vertex_gradient(ui, "Ground Truth (CPU gradient) - vertices", bg_fill, &g);
                    self.tex_gradient(ui, "Ground Truth (CPU gradient) - texture", bg_fill, &g);
                } else {
                    let g = Gradient::ground_truth_gradient(left, right, interpolation)
                        .with_bg_fill(bg_fill);
                    self.vertex_gradient(
                        ui,
                        "Ground Truth (CPU gradient, CPU blending) - vertices",
                        bg_fill,
                        &g,
                    );
                    self.tex_gradient(
                        ui,
                        "Ground Truth (CPU gradient, CPU blending) - texture",
                        bg_fill,
                        &g,
                    );
                    let g = Gradient::ground_truth_gradient(left, right, interpolation);
                    self.vertex_gradient(ui, "CPU gradient, GPU blending - vertices", bg_fill, &g);
                    self.tex_gradient(ui, "CPU gradient, GPU blending - texture", bg_fill, &g);
                }

                let g = Gradient::endpoints(left, right);

                match interpolation {
                    Interpolation::Linear => {
                        // texture sampler is sRGBA aware, and should therefore be linear
                        self.tex_gradient(
                            ui,
                            "Texture of width 2 (test texture sampler)",
                            bg_fill,
                            &g,
                        );
                    }
                    Interpolation::Gamma => {
                        // vertex shader uses gamma
                        self.vertex_gradient(
                            ui,
                            "Triangle mesh of width 2 (test vertex decode and interpolation)",
                            bg_fill,
                            &g,
                        );
                    }
                }
            });
        }

        fn tex_gradient(
            &mut self,
            ui: &mut Ui,
            label: &str,
            bg_fill: Color32,
            gradient: &Gradient,
        ) {
            if !self.texture_gradients {
                return;
            }
            ui.horizontal(|ui| {
                let tex = self.tex_mngr.get(ui.ctx(), gradient);
                let texel_offset = 0.5 / (gradient.0.len() as f32);
                let uv = Rect::from_min_max(pos2(texel_offset, 0.0), pos2(1.0 - texel_offset, 1.0));
                ui.add(
                    Image::from_texture((tex.id(), GRADIENT_SIZE))
                        .bg_fill(bg_fill)
                        .uv(uv),
                )
                .on_hover_text(format!(
                    "A texture that is {} texels wide",
                    gradient.0.len()
                ));
                ui.label(label);
            });
        }

        fn vertex_gradient(
            &mut self,
            ui: &mut Ui,
            label: &str,
            bg_fill: Color32,
            gradient: &Gradient,
        ) {
            if !self.vertex_gradients {
                return;
            }
            ui.horizontal(|ui| {
                vertex_gradient(ui, bg_fill, gradient).on_hover_text(format!(
                    "A triangle mesh that is {} vertices wide",
                    gradient.0.len()
                ));
                ui.label(label);
            });
        }
    }

    fn vertex_gradient(ui: &mut Ui, bg_fill: Color32, gradient: &Gradient) -> Response {
        use egui::epaint::*;
        let (rect, response) = ui.allocate_at_least(GRADIENT_SIZE, Sense::hover());
        if bg_fill != Default::default() {
            let mut mesh = Mesh::default();
            mesh.add_colored_rect(rect, bg_fill);
            ui.painter().add(Shape::mesh(mesh));
        }
        {
            let n = gradient.0.len();
            assert!(n >= 2);
            let mut mesh = Mesh::default();
            for (i, &color) in gradient.0.iter().enumerate() {
                let t = i as f32 / (n as f32 - 1.0);
                let x = lerp(rect.x_range(), t);
                mesh.colored_vertex(pos2(x, rect.top()), color);
                mesh.colored_vertex(pos2(x, rect.bottom()), color);
                if i < n - 1 {
                    let i = i as u32;
                    mesh.add_triangle(2 * i, 2 * i + 1, 2 * i + 2);
                    mesh.add_triangle(2 * i + 1, 2 * i + 2, 2 * i + 3);
                }
            }
            ui.painter().add(Shape::mesh(mesh));
        }
        response
    }

    #[derive(Clone, Copy)]
    enum Interpolation {
        Linear,
        Gamma,
    }

    #[derive(Clone, Hash, PartialEq, Eq)]
    struct Gradient(pub Vec<Color32>);

    impl Gradient {
        pub fn one_color(srgba: Color32) -> Self {
            Self(vec![srgba, srgba])
        }

        pub fn endpoints(left: Color32, right: Color32) -> Self {
            Self(vec![left, right])
        }

        pub fn ground_truth_gradient(
            left: Color32,
            right: Color32,
            interpolation: Interpolation,
        ) -> Self {
            match interpolation {
                Interpolation::Linear => Self::ground_truth_linear_gradient(left, right),
                Interpolation::Gamma => Self::ground_truth_gamma_gradient(left, right),
            }
        }

        pub fn ground_truth_linear_gradient(left: Color32, right: Color32) -> Self {
            let left = Rgba::from(left);
            let right = Rgba::from(right);

            let n = 255;
            Self(
                (0..=n)
                    .map(|i| {
                        let t = i as f32 / n as f32;
                        Color32::from(lerp(left..=right, t))
                    })
                    .collect(),
            )
        }

        pub fn ground_truth_gamma_gradient(left: Color32, right: Color32) -> Self {
            let n = 255;
            Self(
                (0..=n)
                    .map(|i| {
                        let t = i as f32 / n as f32;
                        lerp_color_gamma(left, right, t)
                    })
                    .collect(),
            )
        }

        /// Do premultiplied alpha-aware blending of the gradient on top of the fill color
        /// in gamma-space.
        pub fn with_bg_fill(self, bg: Color32) -> Self {
            Self(
                self.0
                    .into_iter()
                    .map(|fg| {
                        let a = fg.a() as f32 / 255.0;
                        Color32::from_rgba_premultiplied(
                            (bg[0] as f32 * (1.0 - a) + fg[0] as f32).round() as u8,
                            (bg[1] as f32 * (1.0 - a) + fg[1] as f32).round() as u8,
                            (bg[2] as f32 * (1.0 - a) + fg[2] as f32).round() as u8,
                            (bg[3] as f32 * (1.0 - a) + fg[3] as f32).round() as u8,
                        )
                    })
                    .collect(),
            )
        }

        pub fn to_pixel_row(&self) -> Vec<Color32> {
            self.0.clone()
        }
    }

    #[derive(Default)]
    struct TextureManager(HashMap<Gradient, TextureHandle>);

    impl TextureManager {
        fn get(&mut self, ctx: &egui::Context, gradient: &Gradient) -> &TextureHandle {
            self.0.entry(gradient.clone()).or_insert_with(|| {
                let pixels = gradient.to_pixel_row();
                let width = pixels.len();
                let height = 1;
                ctx.load_texture(
                    "color_test_gradient",
                    epaint::ColorImage {
                        size: [width, height],
                        pixels,
                    },
                    TextureOptions::LINEAR,
                )
            })
        }
    }

    fn pixel_test(ui: &mut Ui) {
        ui.label("Each subsequent square should be one physical pixel larger than the previous. They should be exactly one physical pixel apart. They should be perfectly aligned to the pixel grid.");

        let color = if ui.style().visuals.dark_mode {
            egui::Color32::WHITE
        } else {
            egui::Color32::BLACK
        };

        let pixels_per_point = ui.ctx().pixels_per_point();
        let num_squares: u32 = 8;
        let size_pixels = vec2(
            ((num_squares + 1) * (num_squares + 2) / 2) as f32,
            num_squares as f32,
        );
        let size_points = size_pixels / pixels_per_point + Vec2::splat(2.0);
        let (response, painter) = ui.allocate_painter(size_points, Sense::hover());

        let mut cursor_pixel = Pos2::new(
            response.rect.min.x * pixels_per_point,
            response.rect.min.y * pixels_per_point,
        )
        .ceil();
        for size in 1..=num_squares {
            let rect_points = Rect::from_min_size(
                Pos2::new(
                    cursor_pixel.x / pixels_per_point,
                    cursor_pixel.y / pixels_per_point,
                ),
                Vec2::splat(size as f32) / pixels_per_point,
            );
            painter.rect_filled(rect_points, 0.0, color);
            cursor_pixel.x += (1 + size) as f32;
        }
    }

    fn blending_and_feathering_test(ui: &mut Ui) {
        let size = vec2(512.0, 512.0);
        let (response, painter) = ui.allocate_painter(size, Sense::hover());
        let rect = response.rect;

        let mut top_half = rect;
        top_half.set_bottom(top_half.center().y);
        painter.rect_filled(top_half, 0.0, Color32::BLACK);
        paint_fine_lines_and_text(&painter, top_half, Color32::WHITE);

        let mut bottom_half = rect;
        bottom_half.set_top(bottom_half.center().y);
        painter.rect_filled(bottom_half, 0.0, Color32::WHITE);
        paint_fine_lines_and_text(&painter, bottom_half, Color32::BLACK);
    }

    fn text_on_bg(ui: &mut egui::Ui, fg: Color32, bg: Color32) {
        assert!(fg.is_opaque());
        assert!(bg.is_opaque());

        ui.horizontal(|ui| {
            ui.label(
                RichText::from("▣ The quick brown fox jumps over the lazy dog and runs away.")
                    .background_color(bg)
                    .color(fg),
            );
            ui.label(format!(
                "({} {} {}) on ({} {} {})",
                fg.r(),
                fg.g(),
                fg.b(),
                bg.r(),
                bg.g(),
                bg.b(),
            ));
        });
    }

    fn paint_fine_lines_and_text(painter: &egui::Painter, mut rect: Rect, color: Color32) {
        {
            let mut y = 0.0;
            for opacity in [1.00, 0.50, 0.25, 0.10, 0.05, 0.02, 0.01, 0.00] {
                painter.text(
                    rect.center_top() + vec2(0.0, y),
                    Align2::LEFT_TOP,
                    format!("{:.0}% white", 100.0 * opacity),
                    FontId::proportional(14.0),
                    Color32::WHITE.gamma_multiply(opacity),
                );
                painter.text(
                    rect.center_top() + vec2(80.0, y),
                    Align2::LEFT_TOP,
                    format!("{:.0}% gray", 100.0 * opacity),
                    FontId::proportional(14.0),
                    Color32::GRAY.gamma_multiply(opacity),
                );
                painter.text(
                    rect.center_top() + vec2(160.0, y),
                    Align2::LEFT_TOP,
                    format!("{:.0}% black", 100.0 * opacity),
                    FontId::proportional(14.0),
                    Color32::BLACK.gamma_multiply(opacity),
                );
                y += 20.0;
            }

            for font_size in [6.0, 7.0, 8.0, 9.0, 10.0, 12.0, 14.0] {
                painter.text(
                    rect.center_top() + vec2(0.0, y),
                    Align2::LEFT_TOP,
                    format!(
                    "{font_size}px - The quick brown fox jumps over the lazy dog and runs away."
                ),
                    FontId::proportional(font_size),
                    color,
                );
                y += font_size + 1.0;
            }
        }

        rect.max.x = rect.center().x;

        rect = rect.shrink(16.0);
        for width in [0.05, 0.1, 0.25, 0.5, 1.0, 2.0, 4.0] {
            painter.text(
                rect.left_top(),
                Align2::CENTER_CENTER,
                width.to_string(),
                FontId::monospace(12.0),
                color,
            );

            painter.add(egui::epaint::CubicBezierShape::from_points_stroke(
                [
                    rect.left_top() + vec2(16.0, 0.0),
                    rect.right_top(),
                    rect.right_center(),
                    rect.right_bottom(),
                ],
                false,
                Color32::TRANSPARENT,
                Stroke::new(width, color),
            ));

            rect.min.y += 24.0;
            rect.max.x -= 24.0;
        }

        rect.min.y += 16.0;
        painter.text(
            rect.left_top(),
            Align2::LEFT_CENTER,
            "transparent --> opaque",
            FontId::monospace(10.0),
            color,
        );
        rect.min.y += 12.0;
        let mut mesh = Mesh::default();
        mesh.colored_vertex(rect.left_bottom(), Color32::TRANSPARENT);
        mesh.colored_vertex(rect.left_top(), Color32::TRANSPARENT);
        mesh.colored_vertex(rect.right_bottom(), color);
        mesh.colored_vertex(rect.right_top(), color);
        mesh.add_triangle(0, 1, 2);
        mesh.add_triangle(1, 2, 3);
        painter.add(mesh);
    }

    fn mul_color_gamma(left: Color32, right: Color32) -> Color32 {
        Color32::from_rgba_premultiplied(
            (left.r() as f32 * right.r() as f32 / 255.0).round() as u8,
            (left.g() as f32 * right.g() as f32 / 255.0).round() as u8,
            (left.b() as f32 * right.b() as f32 / 255.0).round() as u8,
            (left.a() as f32 * right.a() as f32 / 255.0).round() as u8,
        )
    }

    fn lerp_color_gamma(left: Color32, right: Color32, t: f32) -> Color32 {
        Color32::from_rgba_premultiplied(
            lerp((left[0] as f32)..=(right[0] as f32), t).round() as u8,
            lerp((left[1] as f32)..=(right[1] as f32), t).round() as u8,
            lerp((left[2] as f32)..=(right[2] as f32), t).round() as u8,
            lerp((left[3] as f32)..=(right[3] as f32), t).round() as u8,
        )
    }
}


// When compiling natively:
fn main() -> Result<(), eframe::Error> {
    {
        // Silence wgpu log spam (https://github.com/gfx-rs/wgpu/issues/3206)
        let mut rust_log = std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_owned());
        for loud_crate in ["naga", "wgpu_core", "wgpu_hal"] {
            if !rust_log.contains(&format!("{loud_crate}=")) {
                rust_log += &format!(",{loud_crate}=warn");
            }
        }
        std::env::set_var("RUST_LOG", rust_log);
    }

    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let options = eframe::NativeOptions {
        drag_and_drop_support: true,

        initial_window_size: Some([1280.0, 1024.0].into()),

        #[cfg(feature = "wgpu")]
        renderer: eframe::Renderer::Wgpu,

        ..Default::default()
    };
    eframe::run_native(
        "egui demo app",
        options,
        Box::new(|cc| Box::new(egui_demo_app::WrapApp::new(cc))),
    )
}
