use macroquad::prelude::*;

fn conf() -> Conf {
    Conf {
        window_title: "Projectile Motion!".to_owned(),
        window_width: (WIDTH * 2.0) as i32,
        window_height: (HEIGHT * 2.0) as i32,
        high_dpi: true,
        fullscreen: false,
        window_resizable: false,
        icon: None,
        platform: Default::default(),
        sample_count: 1,
    }
}

#[macroquad::main(conf())]
async fn main() {
    loop {
        clear_background(LIGHTGRAY);
        next_frame().await;
    }
}
