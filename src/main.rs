use macroquad::prelude::*;

const WIDTH:f32 = 700.0;
const HEIGHT:f32 = 500.0;

fn conf() -> Conf {
    Conf {
        window_title: "Wreck me.".to_owned(),
        window_width: (WIDTH*1.5) as i32,
        window_height: (HEIGHT*1.5) as i32,
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

fn draw_slider(mut value: f32, pos: Vec2, colour: Color, label:&str) -> f32 { 
    let size = vec2(WIDTH/4.0, HEIGHT/30.0);
    let min: f32 = 1.0;
    let max: f32 = 100.0;

    draw_rectangle(pos.x, pos.y, size.x, size.y, colour);

    // moves the recatngle to the mouse place
    if is_mouse_button_down(MouseButton::Left) {

        let mouse = mouse_position();
        let mx = mouse.0;
        if mx >= pos.x && mx <= pos.x + size.x &&
           mouse.1 >= pos.y && mouse.1 <= pos.y + size.y
        {
            value = min + (mx - pos.x) / size.x * (max - min);
        }
    }

    let handle_x = pos.x + (value - min) / (max - min) * size.x;
    draw_text(label, pos.x, pos.y - 10.0, 20.0, DARKGRAY);
    draw_rectangle(handle_x - 5.0, pos.y, 10.0, size.y + 5.0, DARKGRAY);

    return value
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

fn did_hit(
    x: f32, y: f32, w: f32, h: f32,
    mx: f32, my: f32, mw: f32, mh: f32
) -> bool {
    x < mx + mw &&
    x + w > mx &&
    y < my + mh &&
    y + h > my
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

async fn level_loading(path1: String, path2: String){
    let bg_texture1 = load_texture(&path1).await.unwrap();
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
                return r"images\projectiles\orange.png".to_string()
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
                return  r"images\projectiles\cat.png".to_string()
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
                return r"images\projectiles\proton.png".to_string()
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

async fn tutorial_launch(projectile_path: String) -> bool {
    let path1 = r"images\tutorial_ground_one.png";
    let bg_texture1 = load_texture(&path1).await.unwrap();
    let path2 = r"images\tutorial_ground_two.png";
    let bg_texture2 = load_texture(&path2).await.unwrap();

    let obstacle_path = r"images\obstacles\bricks_ok.png";
    let obstacle = load_texture(&obstacle_path).await.unwrap();

    let projectile =  load_texture(&projectile_path).await.unwrap();
    let mut projectile_x = 100.0;
    let mut projectile_y = HEIGHT - 120.0;
    let mut theta:f32 = 45.0;
    let mut u:f32 = 10.0;
    loop {
        clear_background(LIGHTGRAY);
        draw_bg(&bg_texture1, &bg_texture2);
        theta = draw_slider(theta, vec2(20.0, 50.0), Color::new(0.0, 0.0, 0.0, 1.0), "angle");
        u = draw_slider(u, vec2(20.0, 120.0), Color::new(0.0, 0.0, 0.0, 1.0), "initial velocity");
        if (theta != 45.0) || (u != 10.0) {
            next_frame().await;
            break;
        }
        draw_texture_ex(
            &projectile, 
            projectile_x - 30.0, projectile_y - 30.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(60.0, 60.0)),
                ..Default::default()
            },
        );
        draw_texture_ex(
            &obstacle, 
            600.0 - 70.0, HEIGHT-120.0 - 130.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(140.0, 140.0)),
                ..Default::default()
            },
        );
        next_frame().await;
    }

    let path1 = r"images\tutorial_ground_three.png";
    let bg_texture1 = load_texture(&path1).await.unwrap();
    let path2 = r"images\tutorial_ground_four.png";
    let bg_texture2 = load_texture(&path2).await.unwrap();

    let launch_path = r"images\launch.png";
    let launch = load_texture(&launch_path).await.unwrap();

    loop {
        clear_background(LIGHTGRAY);
        draw_bg(&bg_texture1, &bg_texture2);
        theta = draw_slider(theta, vec2(20.0, 50.0), Color::new(0.0, 0.0, 0.0, 1.0), "angle");
        u = draw_slider(u, vec2(20.0, 120.0), Color::new(0.0, 0.0, 0.0, 1.0), "initial velocity");
        draw_texture_ex(
            &projectile, 
            projectile_x - 30.0, projectile_y - 30.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(60.0, 60.0)),
                ..Default::default()
            },
        );
        draw_texture_ex(
            &launch, 
            WIDTH - 150.0, -10.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(150.0, 150.0)),
                ..Default::default()
            },
        );
        draw_texture_ex(
            &obstacle, 
            600.0 - 70.0, HEIGHT-120.0 - 130.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(140.0, 140.0)),
                ..Default::default()
            },
        );

        if is_hovering(WIDTH - 150.0, -10.0, 150.0, 150.0) && is_mouse_button_down(MouseButton::Left){
            next_frame().await;
            break;
        }

        next_frame().await;
    }

    let path1 = r"images\tutorial_launched1.png";
    let bg_texture1 = load_texture(&path1).await.unwrap();
    let path2 = r"images\tutorial_launched2.png";
    let bg_texture2 = load_texture(&path2).await.unwrap();
    theta = theta * 0.9;
    u = u * 2.0;
    let mut velocityx = theta.to_radians().cos() * u;
    let mut velocityy = -theta.to_radians().sin() * u;
    loop {
        clear_background(LIGHTGRAY);
        draw_bg(&bg_texture1, &bg_texture2);
        draw_texture_ex(
            &projectile, 
            projectile_x - 30.0, projectile_y - 30.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(60.0, 60.0)),
                ..Default::default()
            },
        );
        draw_texture_ex(
            &obstacle, 
            600.0 - 70.0, HEIGHT-120.0 - 130.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(140.0, 140.0)),
                ..Default::default()
            },
        );
        projectile_x += velocityx*0.05;
        projectile_y += velocityy*0.05;
        velocityy += 9.81 * 0.05;

        if did_hit(600.0, HEIGHT-120.0, 140.0, 140.0, projectile_x, projectile_y, 60.0, 60.0) {
            return true;
        }

        if projectile_y > HEIGHT - 120.0 {
            velocityx = 0.0;
            velocityy = 0.0;
            if did_hit(600.0, HEIGHT-120.0, 140.0, 140.0, projectile_x, projectile_y, 60.0, 60.0) {
                return true;
            }
            else {
                return false;
            }
            break false;  
        };
        next_frame().await;
    }

    
}

async fn level(projectile_path: String, path1: String, path2: String, obstacle_path: String, obstacle_position: Vec2) -> bool {
    let bg_texture1 = load_texture(&path1).await.unwrap();
    let bg_texture2 = load_texture(&path2).await.unwrap();

    let obstacle = load_texture(&obstacle_path).await.unwrap();
    let obstaclex = obstacle_position[0];
    let obstacley = obstacle_position[1];

    let projectile =  load_texture(&projectile_path).await.unwrap();
    let mut projectile_x = 100.0;
    let mut projectile_y = HEIGHT - 120.0;
    let mut theta:f32 = 45.0;
    let mut u:f32 = 10.0;

    let launch_path = r"images\launch.png";
    let launch = load_texture(&launch_path).await.unwrap();

    loop {
        clear_background(LIGHTGRAY);
        draw_bg(&bg_texture1, &bg_texture2);
        theta = draw_slider(theta, vec2(20.0, 50.0), Color::new(0.0, 0.0, 0.0, 1.0), "angle");
        u = draw_slider(u, vec2(20.0, 120.0), Color::new(0.0, 0.0, 0.0, 1.0), "initial velocity");
        draw_texture_ex(
            &projectile, 
            projectile_x - 30.0, projectile_y - 30.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(60.0, 60.0)),
                ..Default::default()
            },
        );
        draw_texture_ex(
            &launch, 
            WIDTH - 150.0, -10.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(150.0, 150.0)),
                ..Default::default()
            },
        );
        draw_texture_ex(
            &obstacle, 
            obstaclex - 70.0, obstacley - 130.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(140.0, 140.0)),
                ..Default::default()
            },
        );

        if is_hovering(WIDTH - 150.0, -10.0, 150.0, 150.0) && is_mouse_button_down(MouseButton::Left){
            next_frame().await;
            break;
        }

        next_frame().await;
    }

    theta = theta * 0.9;
    u = u * 2.0;
    let mut velocityx = theta.to_radians().cos() * u;
    let mut velocityy = -theta.to_radians().sin() * u;
    loop {
        clear_background(LIGHTGRAY);
        draw_bg(&bg_texture1, &bg_texture2);
        draw_texture_ex(
            &projectile, 
            projectile_x - 30.0, projectile_y - 30.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(60.0, 60.0)),
                ..Default::default()
            },
        );
        draw_texture_ex(
            &obstacle, 
            obstaclex - 70.0, obstacley - 130.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(140.0, 140.0)),
                ..Default::default()
            },
        );
        projectile_x += velocityx*0.05;
        projectile_y += velocityy*0.05;
        velocityy += 9.81 * 0.05;

        if did_hit(obstaclex, obstacley, 140.0, 140.0, projectile_x, projectile_y, 60.0, 60.0) {
            return true;
        }

        if projectile_y > HEIGHT - 120.0 {
            velocityx = 0.0;
            velocityy = 0.0;
            if did_hit(obstaclex, obstacley, 140.0, 140.0, projectile_x, projectile_y, 60.0, 60.0) {
                return true;
            }
            else {
                return false;
            }
            break false;  
        };
        next_frame().await;
    }

    
}

async fn tutorial_win() {
    let obstacle_path1 = r"images\obstacles\bricks_kabam_one.png";
    let obstacle1 = load_texture(&obstacle_path1).await.unwrap();
    let obstacle_path2 = r"images\obstacles\bricks_kabam_two.png";
    let obstacle2 = load_texture(&obstacle_path2).await.unwrap();

    let path1 = r"images\tutorialwin1.png";
    let bg_texture1 = load_texture(&path1).await.unwrap();
    let path2 = r"images\tutorialwin2.png";
    let bg_texture2 = load_texture(&path2).await.unwrap();

    loop {
        clear_background(LIGHTGRAY);
        draw_bg(&bg_texture1, &bg_texture2);
        let t = get_time();
        let obstacle = if ((t / 0.8).floor() as i32) % 2 == 0 {
                &obstacle1
            } else {
                &obstacle2
        };
        draw_texture_ex(
            &obstacle, 
            600.0 - 70.0, HEIGHT - 120.0- 130.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(140.0, 140.0)),
                ..Default::default()
            },
        );
        
        if is_mouse_button_down(MouseButton::Left){
            next_frame().await;
            break;
        }
        next_frame().await;
    }

}

async fn level_win(path1: String, path2: String, obstacle_path1: String, obstacle_path2: String, obstacle_position: Vec2) {
    let obstacle1 = load_texture(&obstacle_path1).await.unwrap();
    let obstacle2 = load_texture(&obstacle_path2).await.unwrap();

    let bg_texture1 = load_texture(&path1).await.unwrap();
    let bg_texture2 = load_texture(&path2).await.unwrap();

    loop {
        clear_background(LIGHTGRAY);
        draw_bg(&bg_texture1, &bg_texture2);
        let t = get_time();
        let obstacle = if ((t / 0.8).floor() as i32) % 2 == 0 {
                &obstacle1
            } else {
                &obstacle2
        };
        draw_texture_ex(
            &obstacle, 
            obstacle_position[0] - 70.0, obstacle_position[1] - 130.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(140.0, 140.0)),
                ..Default::default()
            },
        );
        
        if is_mouse_button_down(MouseButton::Left){
            next_frame().await;
            break;
        }
        next_frame().await;
    }

}

async fn tutorial() -> String {
        tutorial_loading().await;
        text_slides_tutorial().await;
        let projectile_path = pick_projectile().await;
        let mut win = tutorial_launch(projectile_path.clone()).await;
        while win == false {
            win = tutorial_launch(projectile_path.clone()).await;
        }
        tutorial_win().await;
        return projectile_path;
}



#[macroquad::main(conf())]
async fn main() {
    welcome_screen().await;
    let projectile_path = tutorial().await;
    level_loading(r"images\levelone1.png".to_string(), r"images\levelone2.png".to_string()).await;
    let mut win = level(projectile_path.clone(), r"images\levelonebg.png".to_string(), r"images\levelonebg.png".to_string(), r"images\starok.png".to_string(), vec2(300.0, HEIGHT-200.0)).await;
    while win == false {
        win = level(projectile_path.clone(), r"images\levelonebg.png".to_string(), r"images\levelonebg.png".to_string(), r"images\starok.png".to_string(), vec2(300.0, HEIGHT-200.0)).await;
    }
    level_win(r"images\levelonewin1.png".to_string(), r"images\levelonewin2.png".to_string(), r"images\starbrokenone.png".to_string(), r"images\starbrokentwo.png".to_string(), vec2(300.0, HEIGHT-200.0)).await;

}
