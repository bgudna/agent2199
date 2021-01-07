use tcod::colors::*;
use tcod::console::*;
use std::cmp;


// initializing stuff

//screen
const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;

//map
const MAP_WIDTH: i32 = 80;
const MAP_HEIGHT: i32 = 45;

//wall + floor tiles
const COLOR_DARK_WALL: Color = Color { r: 0, g: 0, b: 100 };
const COLOR_DARK_GROUND: Color = Color {
    r: 50,
    g: 50,
    b: 150,
};

// main fps setter
const LIMIT_FPS: i32 = 20;

struct Tcod {
    root: Root,
    con: Offscreen,
}

type Map = Vec<Vec<Tile>>;

struct Game {
    map: Map,
}

#[derive(Clone, Copy, Debug)]
struct Tile {
    blocked: bool,
    block_sight: bool,
}

impl Tile {
    pub fn empty() -> Self {
        Tile {
            blocked: false,
            block_sight: false,
        }
    }

    pub fn wall() -> Self {
        Tile {
            blocked: true,
            block_sight: true,
        }
    }
}

#[derive(Debug)]
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
    pub fn move_by(&mut self, dx: i32, dy: i32, game: &Game) {
        if !game.map[(self.x + dx) as usize][(self.y + dy) as usize].blocked {  
            self.x += dx;  
            self.y += dy;
        }
    }

    /// set the color and then draw the character that represents this object at its position
    pub fn draw(&self, con: &mut dyn Console) {
        con.set_default_foreground(self.color);
        con.put_char(self.x, self.y, self.char, BackgroundFlag::None);
    }
}

//make us some rooms!
#[derive(Clone, Copy, Debug)]
struct Rect {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

impl Rect {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
        Rect {
            x1: x,
            y1: y,
            x2: x + w,
            y2: y + h,
        }
    }
}

fn make_map() -> Map {
    // fill map with "unblocked" tiles
    //let mut map = vec![vec![Tile::empty(); MAP_HEIGHT as usize]; MAP_WIDTH as usize];
    let mut map = vec![vec![Tile::wall(); MAP_HEIGHT as usize]; MAP_WIDTH as usize];


    //map[30][22] = Tile::wall();
    //map[50][22] = Tile::wall();
    
    // create two rooms
    let room1 = Rect::new(20, 15, 10, 15);
    let room2 = Rect::new(50, 15, 10, 15);
    create_room(room1, &mut map);
    create_room(room2, &mut map);

    create_h_tunnel(25, 55, 23, &mut map);
    map
}

// these functions make the tunnels between the rooms
fn create_h_tunnel(x1: i32, x2: i32, y: i32, map: &mut Map) {
    // horizontal tunnel. `min()` and `max()` are used in case `x1 > x2`
    for x in cmp::min(x1, x2)..(cmp::max(x1, x2) + 1) {
        map[x as usize][y as usize] = Tile::empty();
    }
}

fn create_v_tunnel(y1: i32, y2: i32, x: i32, map: &mut Map) {
    // vertical tunnel
    for y in cmp::min(y1, y2)..(cmp::max(y1, y2) + 1) {
        map[x as usize][y as usize] = Tile::empty();
    }
}

fn render_all(tcod: &mut Tcod, game: &Game, objects: &[Object]) {
    // draw all objects in the list
    for object in objects {
        object.draw(&mut tcod.con);
    }

    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            let wall = game.map[x as usize][y as usize].block_sight;
            if wall {
                tcod.con
                    .set_char_background(x, y, COLOR_DARK_WALL, BackgroundFlag::Set);
            } else {
                tcod.con
                    .set_char_background(x, y, COLOR_DARK_GROUND, BackgroundFlag::Set);
            }
        }
    }

    blit(
        &tcod.con,
        (0, 0),
        (MAP_WIDTH, MAP_HEIGHT),
        &mut tcod.root,
        (0, 0),
        1.0,
        1.0,
    );
}

fn create_room(room: Rect, map: &mut Map) {
    // go through the tiles in the rectangle and make them passable
    for x in (room.x1 + 1)..room.x2 {
        for y in (room.y1 + 1)..room.y2 {
            map[x as usize][y as usize] = Tile::empty();
        }
    }
}

fn handle_keys(tcod: &mut Tcod, game: &Game, player: &mut Object) -> bool {
    // todo: handle keys

    use tcod::input::Key;
    use tcod::input::KeyCode::*;

    let key = tcod.root.wait_for_keypress(true);

    match key {
    // movement keys
    Key { code: Up, .. } => player.move_by(0, -1, game),
    Key { code: Down, .. } => player.move_by(0, 1, game),
    Key { code: Left, .. } => player.move_by(-1, 0, game),
    Key { code: Right, .. } => player.move_by(1, 0, game),

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

    let con = Offscreen::new(MAP_WIDTH, MAP_HEIGHT);

    let root = Root::initializer()
        .font("arial10x10.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Agent2199 - an absolute dummy name")
        .init();
    
    let mut tcod = Tcod { root, con };

    // create object representing the player
    //let player = Object::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2, '@', GREEN);
    let player = Object::new(25, 23, '@', WHITE);


    // create an NPC
    let npc = Object::new(SCREEN_WIDTH / 2 - 5, SCREEN_HEIGHT / 2, 'Y', YELLOW);

    // the list of objects with those two
    let mut objects = [player, npc];

    let game = Game {
        // generate map (at this point it's not drawn to the screen)
        map: make_map(),
    };

    //game loop is here
    while !tcod.root.window_closed() {
    
        tcod.con.clear(); //clear previous frame
        
        //draw stuff
        for object in &objects {
            object.draw(&mut tcod.con);
        }
        
        // render the screen/everything!
        render_all(&mut tcod, &game, &objects);
        tcod.root.flush();
        tcod.root.wait_for_keypress(true);
        // handle keys and exit game if needed
        let player = &mut objects[0];
        let exit = handle_keys(&mut tcod, &game, player);
        if exit {
            break;
        }

    }

}