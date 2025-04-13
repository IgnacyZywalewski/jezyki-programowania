use flo_canvas::*;
use flo_draw::*;
use futures::{executor, StreamExt};
use rand::Rng;
use std::sync::{Arc, Mutex};

#[derive(Clone, Copy)]
struct Circle {
    x: f32,
    y: f32,
    r: f32,
}

trait Paint {
    fn paint(&self, gc: &mut dyn GraphicsContext);
}

impl Paint for Circle {
    fn paint(&self, gc: &mut dyn GraphicsContext) {
        gc.circle(self.x, self.y, self.r);
        gc.fill_color(Color::Rgba(0.3, 0.6, 0.8, 1.0));
        gc.fill();
        gc.stroke_color(Color::Rgba(0.0, 0.0, 0.0, 1.0));
        gc.line_width(2.0);
        gc.stroke();
    }
}

fn main() {
    let window_width = 800.0;
    let window_height = 600.0;
    let radius = 50.0;

    let mut rng = rand::thread_rng();
    let random_x = rng.gen_range(radius..window_width - radius);
    let random_y = rng.gen_range(radius..window_height - radius);

    let circle = Arc::new(Mutex::new(Circle {
        x: random_x,
        y: random_y,
        r: radius,
    }));

    with_2d_graphics({
        let circle = Arc::clone(&circle);
        move || {
            let window_properties = WindowProperties::from(&"Projekt");
            let (canvas, events) = create_drawing_window_with_events(window_properties);

             {
                let circle_data = {
                    let circle = circle.lock().unwrap();
                    *circle
                };

                canvas.draw(move |gc| {
                    gc.clear_canvas(Color::Rgba(1.0, 1.0, 1.0, 1.0));
                    gc.canvas_height(1000.0);
                    gc.center_region(0.0, 0.0, 1000.0, 1000.0);
                    circle_data.paint(gc);
                });
            }


            executor::block_on(async move {
                let mut events = events;
                let mut rng = rand::thread_rng();

                while let Some(evt) = events.next().await {
                    match evt {
                        DrawEvent::Pointer(PointerAction::ButtonDown, _, state) => {
                            let (mouse_x, mouse_y) = state.location_in_canvas.unwrap_or((0.0, 0.0));
                            let mouse_x = mouse_x as f32;
                            let mouse_y = mouse_y as f32;

                            let mut circle_guard = circle.lock().unwrap();

                            let distance = ((mouse_x - circle_guard.x).powi(2)
                                + (mouse_y - circle_guard.y).powi(2))
                                .sqrt();

                            if distance <= circle_guard.r {
                                println!("Kliknięto w koło!");

                                circle_guard.x = rng.gen_range(radius..window_width - radius);
                                circle_guard.y = rng.gen_range(radius..window_height - radius);

                                let circle_copy = *circle_guard;

                                canvas.draw(move |gc| {
                                    gc.clear_canvas(Color::Rgba(1.0, 1.0, 1.0, 1.0));
                                    gc.canvas_height(1000.0);
                                    gc.center_region(0.0, 0.0, 1000.0, 1000.0);
                                    circle_copy.paint(gc);
                                });
                            }

                        }
                        _ => {}
                    }
                }
            });
        }
    });
}
