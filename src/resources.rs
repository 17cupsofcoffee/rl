use hecs::World;
use rand::Rng;
use tetra::graphics::Color;

use crate::entities;

pub struct Resources {
    pub input: Input,
    pub turn_state: TurnState,
    pub map: Map,
}

impl Resources {
    pub fn new(world: &mut World, map_width: usize, map_height: usize) -> Resources {
        Resources {
            input: Input::new(),
            turn_state: TurnState::new(),
            map: Map::generate(world, map_width, map_height),
        }
    }
}

pub struct Input {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
}

impl Input {
    pub fn new() -> Input {
        Input {
            up: false,
            down: false,
            left: false,
            right: false,
        }
    }
}

pub struct TurnState {
    pub waiting: bool,
}

impl TurnState {
    pub fn new() -> TurnState {
        TurnState { waiting: false }
    }
}

#[derive(Clone)]
pub struct MapTile {
    pub color: Color,
    pub solid: bool,
}

impl MapTile {
    const WALL: MapTile = MapTile {
        color: Color::rgb(0.2, 0.2, 0.2),
        solid: true,
    };

    const FLOOR: MapTile = MapTile {
        color: Color::BLACK,
        solid: false,
    };
}

pub struct Map {
    tiles: Vec<MapTile>,
    width: usize,
}

impl Map {
    pub fn generate(world: &mut World, map_width: usize, map_height: usize) -> Map {
        const MAX_ROOMS: usize = 20;
        const MAX_ATTEMPTS: usize = 200;
        const MIN_SIZE: usize = 5;
        const MAX_SIZE: usize = 15;

        let mut map = Map {
            tiles: vec![MapTile::WALL; map_width * map_height],
            width: map_width,
        };

        let mut rng = rand::thread_rng();

        let mut rooms = vec![];

        'attempt: for _ in 0..MAX_ATTEMPTS {
            let x = rng.gen_range(1..map_width - 1);
            let y = rng.gen_range(1..map_height - 1);

            let width = usize::min(rng.gen_range(MIN_SIZE..MAX_SIZE), map_width - 1 - x);
            let height = usize::min(rng.gen_range(MIN_SIZE..MAX_SIZE), map_height - 1 - y);

            if width < MIN_SIZE || height < MIN_SIZE {
                continue;
            }

            let room = Room {
                x,
                y,
                width,
                height,
            };

            for existing in &rooms {
                if room.intersects(existing) {
                    continue 'attempt;
                }
            }

            map.carve_room(&room);

            rooms.push(room);

            if rooms.len() == MAX_ROOMS {
                break;
            }
        }

        for pair in rooms.windows(2) {
            let (ax, ay) = pair[0].centre();
            let (bx, by) = pair[1].centre();

            if rng.gen_bool(0.5) {
                map.carve_h_corridor(ax, bx, ay);
                map.carve_v_corridor(bx, ay, by);
            } else {
                map.carve_v_corridor(ax, ay, by);
                map.carve_h_corridor(ax, bx, by);
            }
        }

        world.spawn(entities::player(rooms[0].centre().0, rooms[0].centre().1));
        world.spawn(entities::snake(rooms[1].centre().0, rooms[1].centre().1));
        world.spawn(entities::rat(rooms[2].centre().0, rooms[2].centre().1));

        map
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&MapTile> {
        self.tiles.get(x + y * self.width)
    }

    pub fn tile_positions(&self) -> impl Iterator<Item = (usize, usize, &MapTile)> {
        self.tiles
            .iter()
            .enumerate()
            .map(move |(i, tile)| (i % self.width, i / self.width, tile))
    }

    pub fn carve_room(&mut self, room: &Room) {
        for x in room.x + 1..room.x + room.width {
            for y in room.y + 1..room.y + room.height {
                self.tiles[x + y * self.width] = MapTile::FLOOR;
            }
        }
    }

    pub fn carve_h_corridor(&mut self, x1: usize, x2: usize, y: usize) {
        let (start, end) = if x1 < x2 { (x1, x2) } else { (x2, x1) };

        for x in start..=end {
            self.tiles[x + y * self.width] = MapTile::FLOOR;
        }
    }

    pub fn carve_v_corridor(&mut self, x: usize, y1: usize, y2: usize) {
        let (start, end) = if y1 < y2 { (y1, y2) } else { (y2, y1) };

        for y in start..=end {
            self.tiles[x + y * self.width] = MapTile::FLOOR;
        }
    }
}

pub struct Room {
    x: usize,
    y: usize,
    width: usize,
    height: usize,
}

impl Room {
    pub fn centre(&self) -> (usize, usize) {
        (self.x + self.width / 2, self.y + self.height / 2)
    }

    pub fn intersects(&self, other: &Room) -> bool {
        self.x < other.x + other.width
            && self.x + self.width > other.x
            && self.y < other.y + other.height
            && self.y + self.height > other.y
    }
}
