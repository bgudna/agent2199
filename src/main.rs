use tcod::colors::*;
use tcod::console::*;


// actual size of the window
const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;

const LIMIT_FPS: i32 = 20; // 20 frames-per-second maximum

struct Tcod {
    root: Root,
}

fn main() {
    tcod::system::set_fps(LIMIT_FPS);

    let root = Root::initializer()
        .font("arial10x10.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Agent2199 - an absolute dummy name")
        .init();
    
    let mut tcod = Tcod { root };

    let mut player_x = SCREEN_WIDTH / 2;
    let mut player_y = SCREEN_HEIGHT / 2;

    //game loop is here
    while !tcod.root.window_closed() {
        tcod.root.set_default_foreground(GREEN);
        tcod.root.clear();
        tcod.root.put_char(player_x, player_y, '@', BackgroundFlag::None);
        tcod.root.flush();
        tcod.root.wait_for_keypress(true);
        // handle keys and exit game if needed
        let exit = handle_keys(&mut tcod, &mut player_x, &mut player_y);
        if exit {
            break;
        }
    }
}

fn handle_keys(tcod: &mut Tcod, player_x: &mut i32, player_y: &mut i32) -> bool {
    // todo: handle keys

    use tcod::input::Key;
    use tcod::input::KeyCode::*;

    let key = tcod.root.wait_for_keypress(true);
    match key {
    // movement keys
    Key { code: Up, .. } => *player_y -= 1,
    Key { code: Down, .. } => *player_y += 1,
    Key { code: Left, .. } => *player_x -= 1,
    Key { code: Right, .. } => *player_x += 1,

    Key {
        code: Enter,
        alt: true,
        ..
    } => {
        // Alt+Enter: toggle fullscreen
        let fullscreen = tcod.root.is_fullscreen();
        tcod.root.set_fullscreen(!fullscreen);
    }
    Key { code: Escape, .. } => return true, // exit game

    _ => {}
}

    false
}