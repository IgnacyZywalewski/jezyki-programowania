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
use std::io::{self, Write};
use std::fs;
use serde::{Deserialize, Serialize};


//Kolo
#[derive(Clone, Copy)]
struct Circle {
    x: f32,
    y: f32,
    r: f32,
    dx: f32,
    dy: f32,
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

    //Wynik
    let padding = 50.0;
    gc.begin_line_layout(width - 130.0 - padding, height - padding, TextAlignment::Left);
    gc.layout_text(FontId(1), format!("Wynik = {}", score));
    gc.draw_text_layout();

    //Tytul i timer
    gc.begin_line_layout(width / 2.0, height - 50.0, TextAlignment::Center);
    gc.layout_text(FontId(1), message.to_string());
    gc.draw_text_layout();
}


//Poziom trudnosci
enum Difficulty {
    Easy,
    Medium,
    Hard,
}

impl Difficulty {
    fn settings(&self) -> (f32, f32) {
        match self {
            Difficulty::Easy => (50.0, 1.0),
            Difficulty::Medium => (30.0, 2.0),
            Difficulty::Hard => (20.0, 3.0),
        }
    }
}


fn select_difficulty() -> Difficulty {
    println!("Wybierz poziom trudności:");
    println!("1 - Easy\n2 - Medium\n3 - Hard");
    print!("Twój wybór: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    match input.trim() {
        "1" => Difficulty::Easy,
        "2" => Difficulty::Medium,
        "3" => Difficulty::Hard,
        _ => {
            println!("Nieprawidłowy wybór, ustawiam Medium domyślnie.");
            Difficulty::Medium
        }
    }
}


//Wyniki
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
struct HighScores {
    easy: u32,
    medium: u32,
    hard: u32,
}

fn load_high_scores() -> HighScores {
    if let Ok(data) = fs::read_to_string("highscores.json") {
        serde_json::from_str(&data).unwrap_or_default()
    } 
    else {
        HighScores::default()
    }
}

fn save_high_score(difficulty: &Difficulty, score: u32, mut scores: HighScores) {
    let updated = match difficulty {
        Difficulty::Easy if score > scores.easy => {
            scores.easy = score;
            true
        }
        Difficulty::Medium if score > scores.medium => {
            scores.medium = score;
            true
        }
        Difficulty::Hard if score > scores.hard => {
            scores.hard = score;
            true
        }
        _ => false,
    };

    if updated {
        if let Ok(json) = serde_json::to_string_pretty(&scores) {
            let _ = fs::write("highscores.json", json);
        }
    }
}



fn main() {
    let window_width = 1366.0;
    let window_height = 768.0;
    let lato = CanvasFontFace::from_slice(include_bytes!("Lato-Regular.ttf"));

    let difficulty = select_difficulty();
    let (radius, max_speed) = difficulty.settings();
    let high_scores = load_high_scores();

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
            dx: 0.0,
            dy: 0.0,
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
            let (mut game_timer_tx, game_timer_rx) = mpsc::channel::<()>(10);
            let (mut anim_timer_tx, anim_timer_rx) = mpsc::channel::<()>(10);

            let mut game_timer_rx = game_timer_rx.fuse();
            let mut anim_timer_rx = anim_timer_rx.fuse();
            let mut events = events.fuse();

            std::thread::spawn(move || {
                let mut runtime = executor::LocalPool::new();
                runtime.run_until(async move {
                    loop {
                        Delay::new(Duration::from_secs(1)).await;
                        let _ = game_timer_tx.send(()).await;
                    }
                });
            });

            std::thread::spawn(move || {
                let mut runtime = executor::LocalPool::new();
                runtime.run_until(async move {
                    loop {
                        Delay::new(Duration::from_millis(100)).await;
                        let _ = anim_timer_tx.send(()).await;
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
                                        let new_dx = rng.gen_range(-max_speed..=max_speed);
                                        let new_dy = rng.gen_range(-max_speed..=max_speed);

                                        current_circle = Circle {
                                            x: rng.gen_range(radius..window_width - radius),
                                            y: rng.gen_range(radius..window_height - radius),
                                            r: radius,
                                            dx: new_dx,
                                            dy: new_dy,
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

                    _ = game_timer_rx.next() => {
                        if game_started && !game_over {
                            let remaining = end_time.saturating_duration_since(Instant::now());

                            if remaining == Duration::ZERO {
                                game_over = true;
                                message = "Koniec gry".to_string();
                                save_high_score(&difficulty, score, high_scores.clone());
                            } 
                            else {
                                message = format!("Pozostalo: {}:{:02}", remaining.as_secs() / 60, remaining.as_secs() % 60);
                            }   

                            let circle = current_circle;
                            let score = score;
                            let lato = Arc::clone(&lato);
                            let message = message.clone();
                            canvas.draw(move |gc| draw_scene(gc, circle, score, lato, window_width, window_height, &message));
                        }
                    }

                    _ = anim_timer_rx.next() => {
                        if (current_circle.x + current_circle.r) < window_width && (current_circle.x - current_circle.r) > 0.0 
                            && (current_circle.y + current_circle.r) < window_height && (current_circle.y - current_circle.r) > 0.0 {

                            current_circle.x += current_circle.dx as f32;
                            current_circle.y += current_circle.dy as f32;
                        } 

                        let circle = current_circle;
                        let score = score;
                        let lato = Arc::clone(&lato);
                        let message = message.clone();
                        canvas.draw(move |gc| draw_scene(gc, circle, score, lato, window_width, window_height, &message));
                    }

                }
            }
        });
    });
}