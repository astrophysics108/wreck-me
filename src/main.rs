use macroquad::prelude::*;

const WIDTH:f32 = 700.0;
const HEIGHT:f32 = 500.0;

fn conf() -> Conf {
    Conf {
        window_title: "Wreck me.".to_owned(),
        window_width: WIDTH as i32,
        window_height: HEIGHT as i32,
        high_dpi: true,
        fullscreen: false,
        window_resizable: false,
        icon: None,
        platform: Default::default(),
        sample_count: 1,
    }
}

fn draw_bg(bg_path: &Texture2D) {
    draw_texture_ex(
        &bg_path, 
        0.0, 0.0,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(WIDTH, HEIGHT)),
            ..Default::default()
        },
    );
}

async fn welcome_screen() {
    loop {
        clear_background(LIGHTGRAY);
        let path = r"images\wreck_me_one.png";
        let bg_texture = load_texture(&path).await.unwrap();
        draw_bg(&bg_texture);
        next_frame().await;
    }
}

#[macroquad::main(conf())]
async fn main() {
    welcome_screen().await;
}
