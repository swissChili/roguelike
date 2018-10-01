extern crate tcod;
use tcod::console::*;
use tcod::colors::{self, Color};
use tcod::input::Key;
use tcod::input::KeyCode::*;

const MAP_WIDTH: i32 = 80;
const MAP_HEIGHT: i32 = 45;

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;

const COLOR_DARK_WALL: Color = Color { r: 0, g: 0, b: 100 };
const COLOR_DARK_GROUND: Color = Color { r: 50, g: 50, b: 150 };

struct Object {
    x: i32,
    y: i32,
    char: char,
    color: Color,
}

impl Object {
    pub fn new(x: i32, y: i32, char: char, color: Color) -> Self {
        Object {
            x: x,
            y: y,
            char: char,
            color: color,
        }
    }

    pub fn move_by(&mut self , dx: i32, dy: i32) {
        // Pretty self explanatory
        self.y += dy;
        self.x += dx;
    }

    pub fn draw(&self, con: &mut Console) {
        con.set_default_foreground(self.color);
        con.put_char(self.x, self.y, self.char, BackgroundFlag::None);
    }

    pub fn clear(&self, con: &mut Console) {
        con.put_char(self.x, self.y, ' ', BackgroundFlag::None);
    }
}
type Map = Vec<Vec<Tile>>;
fn make_map() -> Map {
    let mut map = vec![vec![Tile::empty(); MAP_HEIGHT as usize]; MAP_WIDTH as usize];

    map
}

#[derive(Clone, Copy, Debug)]
struct Tile {
    blocked: bool,
    block_sight: bool,
}

impl Tile {
    pub fn empty() -> Self {
        Tile{blocked: false, block_sight: false}
    }

    pub fn wall() -> Self {
        Tile{blocked: true, block_sight: true}
    }
}

fn handle_keys(root: &mut Root, player: &mut Object) -> bool {
    let key = root.wait_for_keypress(true);
    match key {
        Key { code: Enter, alt: true, .. } => {
            let fullscreen = root.is_fullscreen();
            root.set_fullscreen(!fullscreen);
        },
        Key { code: Escape, .. } => return true,
        Key { code: Up, .. }     => player.move_by(0, -1),
        Key { code: Down, ..}    => player.move_by(0, 1),
        Key { code: Left, .. }   => player.move_by(-1, 0),
        Key { code: Right, .. }  => player.move_by(1, 0),
        _ => {},
    }
    false
}

fn render_all(root: &mut Root, con: &mut Offscreen, objects: &[Object], map: &Map) {
    for object in objects {
        object.draw(con);
    }
    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            let wall = map[x as usize][y as usize].block_sight;
            if wall {
                con.set_char_background(x, y, COLOR_DARK_WALL, BackgroundFlag::Set);
            } else {
                con.set_char_background(x, y, COLOR_DARK_GROUND, BackgroundFlag::Set);
            }
        }
    }

    blit(con, (0, 0), (SCREEN_WIDTH, SCREEN_HEIGHT), root, (0, 0), 1.0, 1.0);
}

fn main() {

    let mut map = make_map();
    map[30][22] = Tile::wall();
    map[50][22] = Tile::wall();

    const LIMIT_FPS: i32 = 60;
    let mut root = Root::initializer()
        .font("arial10x10.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("my EPIC game")
        .init();
    let mut con = Offscreen::new(SCREEN_WIDTH, SCREEN_HEIGHT);
    tcod::system::set_fps(LIMIT_FPS);
    let player = Object::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT /2, '@', colors::WHITE);
    let npc = Object::new(SCREEN_WIDTH / 2 - 5, SCREEN_HEIGHT / 2, '@', colors::YELLOW);
    let mut objects = [player, npc];

    while !root.window_closed() {
        con.set_default_foreground(colors::WHITE);
        for object in &objects {
            object.draw(&mut con);
        }
        render_all(&mut root, &mut con, &objects, &map);
        root.flush();

        for object in &objects {
            object.clear(&mut con);
        }

        let player = &mut objects[0];
        let exit = handle_keys(&mut root, player);
        if exit {
            break
        }
    }
}
