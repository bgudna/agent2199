use tcod::colors::*;
use tcod::console::*;


// actual size of the window
const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;

const LIMIT_FPS: i32 = 20; // 20 frames-per-second maximum

struct Tcod {
    root: Root,
    con: Offscreen,
}

struct Object {
    x: i32,
    y: i32,
    char: char,
    color: Color,
}

impl Object {
    pub fn new(x: i32, y: i32, char: char, color: Color) -> Self {
        Object { x, y, char, color }
    }

    /// move by the given amount
    pub fn move_by(&mut self, dx: i32, dy: i32) {
        self.x += dx;
        self.y += dy;
    }

    /// set the color and then draw the character that represents this object at its position
    pub fn draw(&self, con: &mut dyn Console) {
        con.set_default_foreground(self.color);
        con.put_char(self.x, self.y, self.char, BackgroundFlag::None);
    }
}


fn handle_keys(tcod: &mut Tcod, player: &mut Object) -> bool {
    // todo: handle keys

    use tcod::input::Key;
    use tcod::input::KeyCode::*;

    let key = tcod.root.wait_for_keypress(true);

    match key {
    // movement keys
    Key { code: Up, .. } => player.move_by(0, -1),
    Key { code: Down, .. } => player.move_by(0, 1),
    Key { code: Left, .. } => player.move_by(-1, 0),
    Key { code: Right, .. } => player.move_by(1, 0),

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

fn main() {
    tcod::system::set_fps(LIMIT_FPS);

    let con = Offscreen::new(SCREEN_WIDTH, SCREEN_HEIGHT);

    let root = Root::initializer()
        .font("arial10x10.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Agent2199 - an absolute dummy name")
        .init();
    
    let mut tcod = Tcod { root, con };

    // create object representing the player
    let player = Object::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2, '@', GREEN);

    // create an NPC
    let npc = Object::new(SCREEN_WIDTH / 2 - 5, SCREEN_HEIGHT / 2, 'Y', YELLOW);

    // the list of objects with those two
    let mut objects = [player, npc];

    //game loop is here
    while !tcod.root.window_closed() {
    
        tcod.con.clear(); //clear previous frame
        
        //draw stuff
        for object in &objects {
            object.draw(&mut tcod.con);
        }

        blit(
            &tcod.con,
            (0, 0),
            (SCREEN_WIDTH, SCREEN_HEIGHT),
            &mut tcod.root,
            (0, 0),
            1.0,
            1.0,
        );
        
        tcod.root.flush();
        tcod.root.wait_for_keypress(true);
        // handle keys and exit game if needed
        let player = &mut objects[0];
        let exit = handle_keys(&mut tcod, player);
        if exit {
            break;
        }

    }

}