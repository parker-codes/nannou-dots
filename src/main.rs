use nannou::prelude::*;

const DOT_RADIUS: f32 = 8.0;

fn main() {
    nannou::app(model).event(event).simple_window(view).run();
}

struct Model {
    points: Vec<Vector2>,

    // colors
    cursor_color: Hsl,
    point_color: Hsl,
}

fn model(_app: &App) -> Model {
    Model {
        points: Vec::new(),

        // colors
        cursor_color: hsl(198.0 / 360.0, 93.0, 60.0), // red
        point_color: hsl(0.0, 91.0, 71.0),            // blue
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(BLACK);

    // draw ellipse that follows mouse
    draw.ellipse()
        .x_y(app.mouse.x, app.mouse.y)
        .radius(DOT_RADIUS)
        .color(model.cursor_color);

    // draw each point we've added
    for point in &model.points {
        draw.ellipse()
            .x_y(point.x, point.y)
            .radius(DOT_RADIUS)
            .color(model.point_color);
    }

    draw.to_frame(app, &frame).unwrap();
}

fn event(app: &App, model: &mut Model, event: Event) {
    match event {
        Event::WindowEvent { id: _, simple } => match simple {
            Some(MousePressed(MouseButton::Left)) => mouse_pressed(app, model),
            _ => (),
        },
        _ => (),
    };
}

fn mouse_pressed(app: &App, model: &mut Model) {
    model.points.push(app.mouse.position());
}
