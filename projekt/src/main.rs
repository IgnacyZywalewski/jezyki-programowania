use flo_canvas::*;
use flo_draw::*;
use flo_draw::binding::*;

use futures::channel::mpsc;
use futures::executor;
use futures::stream::StreamExt;
use futures_timer::Delay;
use futures::sink::SinkExt;

use rand::Rng;
use std::sync::Arc;
use std::time::{Duration, Instant};


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

fn draw_scene(gc: &mut dyn GraphicsContext, circle: Circle, score: u32, lato: Arc<CanvasFontFace>, width: f32, height: f32, message: &str) {
    gc.clear_canvas(Color::Rgba(1.0, 1.0, 1.0, 1.0));
    gc.canvas_height(height);
    gc.center_region(0.0, 0.0, width, height);

    circle.paint(gc);

    gc.define_font_data(FontId(1), Arc::clone(&lato));
    gc.fill_color(Color::Rgba(0.0, 0.0, 0.0, 1.0));
    gc.set_font_size(FontId(1), 32.0);

    // Wynik w prawym górnym rogu
    let padding = 50.0;
    gc.begin_line_layout(width - 130.0 - padding, height - padding, TextAlignment::Left);
    gc.layout_text(FontId(1), format!("Wynik = {}", score));
    gc.draw_text_layout();

    // Napis lub timer na górze środka
    gc.begin_line_layout(width / 2.0, height - 50.0, TextAlignment::Center);
    gc.layout_text(FontId(1), message.to_string());
    gc.draw_text_layout();
}


fn main() {
    let window_width = 1366.0;
    let window_height = 768.0;
    let radius = 40.0;
    let lato = CanvasFontFace::from_slice(include_bytes!("Lato-Regular.ttf"));

    with_2d_graphics(move || {
        let lato = lato;

        let mut window_properties = WindowProperties::from(&"Projekt");
        window_properties.fullscreen = BindRef::from(bind(true));
        window_properties.size = BindRef::from(bind((window_width as u64, window_height as u64)));

        let (canvas, events) = create_drawing_window_with_events(window_properties);

        let mut rng = rand::thread_rng();

        let mut current_circle = Circle {
            x: window_width / 2.0,
            y: window_height / 2.0,
            r: radius,
        };

        let mut score = 0;
        let mut game_started = false;
        let mut game_over = false;
        let mut end_time = Instant::now();

        let mut message = String::from("Kliknij w kółko");

        canvas.draw({
            let circle = current_circle;
            let score = score;
            let lato = Arc::clone(&lato);
            let message = message.clone();

            move |gc| {
                draw_scene(gc, circle, score, lato, window_width, window_height, &message);
            }
        });

        executor::block_on(async move {
            let (mut timer_tx, timer_rx) = mpsc::channel::<()>(10);

            let mut timer_rx = timer_rx.fuse();
            let mut events = events.fuse();

            let _ = std::thread::spawn(move || {
                let mut runtime = executor::LocalPool::new();
                runtime.run_until(async move {
                    loop {
                        Delay::new(Duration::from_secs(1)).await;
                        let _ = timer_tx.send(()).await;
                    }
                });
            });

            loop {
                futures::select! {
                    evt = events.next() => {
                        if let Some(evt) = evt {
                            match evt {
                                DrawEvent::Pointer(PointerAction::ButtonDown, _, state) => {
                                    if game_over { continue; }

                                    let (mouse_x, mouse_y) = state.location_in_canvas.unwrap_or((0.0, 0.0));

                                    let mouse_x = mouse_x as f32;
                                    let mouse_y = mouse_y as f32;

                                    let distance = ((mouse_x - current_circle.x).powi(2) + (mouse_y - current_circle.y).powi(2)).sqrt();

                                    if distance <= current_circle.r {
                                        if !game_started {
                                            game_started = true;
                                            end_time = Instant::now() + Duration::from_secs(60);
                                        }

                                        score += 1;
                                        current_circle = Circle {
                                            x: rng.gen_range(radius..window_width - radius),
                                            y: rng.gen_range(radius..window_height - radius),
                                            r: radius,
                                        };
                                    }

                                    let circle = current_circle;
                                    let score = score;
                                    let lato = Arc::clone(&lato);
                                    let message = message.clone();
                                    canvas.draw(move |gc| draw_scene(gc, circle, score, lato, window_width, window_height, &message));
                                }

                                DrawEvent::KeyDown(_, Some(Key::KeyEscape)) => {
                                    println!("Zamykam aplikację...");
                                    std::process::exit(0);
                                }

                                _ => {}
                            }
                        }
                    },

                    _ = timer_rx.next() => {
                        if game_started && !game_over {
                            let remaining = end_time.saturating_duration_since(Instant::now());

                            if remaining == Duration::ZERO {
                                game_over = true;
                                message = "Koniec gry".to_string();
                            } else {
                                message = format!("Pozostalo: {}:{:02}", remaining.as_secs() / 60, remaining.as_secs() % 60);
                            }

                            let circle = current_circle;
                            let score = score;
                            let lato = Arc::clone(&lato);
                            let message = message.clone();
                            canvas.draw(move |gc| draw_scene(gc, circle, score, lato, window_width, window_height, &message));
                        }
                    }
                }
            }
        });
    });
}