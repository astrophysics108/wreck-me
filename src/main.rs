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

fn is_hovering(x:f32, y: f32, w: f32, h: f32) -> bool {
    let (mx, my) = mouse_position();
    if (x - (w/2.0)) <= mx && (x + (w/2.0)) >= mx && (y - (h/2.0)) <= my && (y + (h/2.0)) >= my {
        return true 
   }
   else {
    return false
   }
}

async fn welcome_screen() {
    let path1 = r"images\wreck_me_one.png";
    let bg_texture1 = load_texture(&path1).await.unwrap();
    let path2 = r"images\wreck_me_two.png";
    let bg_texture2 = load_texture(&path2).await.unwrap();

    let button_path = r"images\button_full.png";
    let button_texture = load_texture(&button_path).await.unwrap();
    let button_path_broken = r"images\button_broken.png";
    let button_texture_broken = load_texture(&button_path_broken).await.unwrap();
    loop {
        clear_background(LIGHTGRAY);
        
        let t = get_time();
        let bg_texture = if ((t / 0.8).floor() as i32) % 2 == 0 {
            &bg_texture1
        } else {
            &bg_texture2
        };

        draw_bg(bg_texture);

        let button = if is_hovering(240.0, 295.0, 150.0, 150.0) {
            &button_texture_broken
        }
        else {
            &button_texture
        };

        draw_texture_ex(
            button, 
            240.0, 295.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(150.0, 150.0)),
                ..Default::default()
            },
        );

        if is_mouse_button_pressed(MouseButton::Left) && is_hovering(240.0, 295.0, 150.0, 150.0) {
            return
        }

        next_frame().await;
    }
}

async fn tutorial() {
    loop {
        clear_background(LIGHTGRAY);
        next_frame().await;
    }
}

#[macroquad::main(conf())]
async fn main() {
    welcome_screen().await;
    tutorial().await;
}
