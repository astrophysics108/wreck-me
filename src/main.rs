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

fn draw_bg(bg_texture1: &Texture2D, bg_texture2: &Texture2D) {
    let t = get_time();
    let bg_texture = if ((t / 0.8).floor() as i32) % 2 == 0 {
            bg_texture1
        } else {
            bg_texture2
    };
    draw_texture_ex(
        bg_texture, 
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
    if x <= mx && (x + w) >= mx && y <= my && (y + h) >= my {
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
        draw_bg(&bg_texture1, &bg_texture2);

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

async fn tutorial_loading(){
    let path1 = r"images\tutorial_1.png";
    let bg_texture1 = load_texture(&path1).await.unwrap();
    let path2 = r"images\tutorial_2.png";
    let bg_texture2 = load_texture(&path2).await.unwrap();
    let t_start = get_time();

    loop {
        clear_background(LIGHTGRAY);
        let t = get_time();
        if t - t_start >= 4.0 {
            return
        };

        draw_bg(&bg_texture1, &bg_texture2);
        next_frame().await;
    }
}

async fn text_slides_tutorial() {
    let bg_paths = vec![
        vec![r"images\tutorial_slides\1_1.png",
        r"images\tutorial_slides\1_2.png"],

        vec![r"images\tutorial_slides\2_1.png",
        r"images\tutorial_slides\2_2.png"]
    ];
    let mut bg_textures = vec![];
    let mut paths_collection_to_push = vec![];
    for paths_collection in bg_paths.iter() {
        paths_collection_to_push = vec![];
        for path in paths_collection.iter() {
            paths_collection_to_push.push(load_texture(&path).await.unwrap());
        };
     bg_textures.push(paths_collection_to_push);
    };

    for slides in bg_textures.iter() {
        let bg_texture1 = &slides[0];
        let bg_texture2 = &slides[1];
        'inner: loop {
            if is_mouse_button_pressed(MouseButton::Left) {
                next_frame().await;
                break 'inner;
            };
            clear_background(LIGHTGRAY);
            draw_bg(&bg_texture1, &bg_texture2);
            next_frame().await;
        };
    }
}

async fn pick_projectile() -> String {
    let path1 = r"images\choose_screen.png";
    let bg_texture1 = load_texture(&path1).await.unwrap();
    let path2 = r"images\choose_screen_two.png";
    let bg_texture2 = load_texture(&path2).await.unwrap();

    let orange_path = r"images\projectiles\orange_intro.png";
    let orange =  load_texture(&orange_path).await.unwrap();
    let cat_path = r"images\projectiles\cat_intro.png";
    let cat =  load_texture(&cat_path).await.unwrap();
    let proton_path = r"images\projectiles\proton_intro.png";
    let proton =  load_texture(&proton_path).await.unwrap();

    loop {
        clear_background(LIGHTGRAY);
        draw_bg(&bg_texture1, &bg_texture2);
        
        

        if is_hovering(150.0, HEIGHT - 50.0, 100.0, 100.0) {
            if is_mouse_button_pressed(MouseButton::Left){
                return orange_path.to_string()
            }
            draw_texture_ex(
                &orange, 
                200.0 - 100.0, HEIGHT - 200.0,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(200.0, 200.0)),
                    ..Default::default()
                },
            );
            draw_texture_ex(
                &cat, 
                350.0 - 50.0, HEIGHT - 50.0,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(100.0, 100.0)),
                    ..Default::default()
                },
            );
            draw_texture_ex(
                &proton,
                500.0 - 50.0, HEIGHT - 50.0,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(100.0, 100.0)),
                    ..Default::default()
                },
            );

        }
        else if is_hovering(300.0, HEIGHT - 50.0, 100.0, 100.0) {
            if is_mouse_button_pressed(MouseButton::Left){
                return cat_path.to_string()
            }
            draw_texture_ex(
                &orange, 
                200.0 - 50.0, HEIGHT - 50.0,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(100.0, 100.0)),
                    ..Default::default()
                },
            );
            draw_texture_ex(
                &cat, 
                350.0 - 100.0, HEIGHT - 200.0,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(200.0, 200.0)),
                    ..Default::default()
                },
            );
            draw_texture_ex(
                &proton,
                500.0 - 50.0, HEIGHT - 50.0,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(100.0, 100.0)),
                    ..Default::default()
                },
            );

        }
        else if is_hovering(450.0, HEIGHT - 50.0, 100.0, 100.0) {
            if is_mouse_button_pressed(MouseButton::Left){
                return proton_path.to_string()
            }
            draw_texture_ex(
                &orange, 
                200.0 - 50.0, HEIGHT - 50.0,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(100.0, 100.0)),
                    ..Default::default()
                },
            );
            draw_texture_ex(
                &cat, 
                350.0 - 50.0, HEIGHT - 50.0,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(100.0, 100.0)),
                    ..Default::default()
                },
            );
            draw_texture_ex(
                &proton,
                500.0 - 100.0, HEIGHT - 200.0,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(200.0, 200.0)),
                    ..Default::default()
                },
            );

        }
        else {
            draw_texture_ex(
                &orange, 
                200.0 - 50.0, HEIGHT - 50.0,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(100.0, 100.0)),
                    ..Default::default()
                },
            );
            draw_texture_ex(
                &cat, 
                350.0 - 50.0, HEIGHT - 50.0,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(100.0, 100.0)),
                    ..Default::default()
                },
            );
            draw_texture_ex(
                &proton,
                500.0 - 50.0, HEIGHT - 50.0,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(100.0, 100.0)),
                    ..Default::default()
                },
            );

        }

        next_frame().await;
     }
}

async fn tutorial() {
        tutorial_loading().await;
        text_slides_tutorial().await;
        let projectile_path = pick_projectile().await;
}

#[macroquad::main(conf())]
async fn main() {
    welcome_screen().await;
    tutorial().await;
}
