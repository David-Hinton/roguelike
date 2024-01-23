// pulling in types and the namespaces from rltk
use rltk::{GameState, Rltk, RGB};
use specs::prelude::*;

mod components;
pub use components::*;

mod player;
use player::*;



pub fn xy_idx(x: i32, y: i32) -> usize {
    (y as usize * 80) + x as usize
}

fn new_map() -> Vec<TileType> {
    let mut map = vec![TileType::Floor; 80 *50 ];

    // Make the boundaries walls
    for x in 0..80 {
        map[xy_idx(x, 0)] = TileType::Wall;
        map[xy_idx(x, 49)] = TileType::Wall;

    }

    for y in 0..50 {
        map[xy_idx(0, y)] = TileType::Wall;
        map[xy_idx(79, y)] = TileType::Wall;
    }

    // Now we'll randomly splat a bunch of walls. It won't be pretty, but it's a decent illistration.
    // First, obtain the thread-local RNG;
    let mut rng = rltk::RandomNumberGenerator::new();

    for _i in 0..400 {
        let x = rng.roll_dice(1, 79);
        let y = rng.roll_dice(1, 49);
        let idx = xy_idx(x, y);

        if idx != xy_idx(40, 25){
            map[idx] = TileType::Wall
        }
    }

    map
}


fn draw_map(map: &[TileType], ctx : &mut Rltk){
    let mut y = 0;
    let mut x = 0;
    for tile in map.iter(){
        // render a tile depending upon the tile type
        match tile{
            TileType::Floor => {
                ctx.set(x,y, RGB::from_f32(0.5,0.5,0.5), RGB::from_f32(0.,0.,0.), rltk::to_cp437('.'));
            }
            TileType::Wall => {
                ctx.set(x, y, RGB::from_f32(0.0, 1.0, 0.0), RGB::from_f32(0., 0., 0.), rltk::to_cp437('#'));
            }
        }

        // move the coordinates
        x += 1;
        if x > 79{
            x = 0;
            y += 1;
        }
    }
}

struct State {
    ecs: World
}

/*
The state structure implements the trait GameState
Traits are like interfaces
*/
impl GameState for State {
    /*
    &mut self:
    &mut self means "this function requires access to the parent structure,
    and may change it" (the mut is short for "mutable" - 
    meaning it can change variables inside the structure - "state"). 
     */

    /*
    ctx : &mut Rltk:
    & means "pass a reference" - which is a pointer to an existing copy of the variable.
    The variable isn't copied, you are working on the version that was passed in; 
    if you make a change, you are changing the original.
    
    mut:
    once again indicates that this is a "mutable" 
    reference: you are allowed to make changes to the context.
    
    Rltk:
    Finally Rltk is the type of the variable you are receiving.
    In this case, it's a struct defined inside the RLTK library
    that provides various things you can do to the screen.
     */
    fn tick(&mut self, ctx : &mut Rltk) {
        ctx.cls();

        player_input(self, ctx);

        let map = self.ecs.fetch::<Vec<TileType>>();
        draw_map(&map, ctx);

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}


fn main() -> rltk::BError {
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build()?;
    let mut gs = State {
        ecs: World::new()
    };
    
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();

    gs.ecs.insert(new_map());

    gs.ecs
        .create_entity()
        .with(Position { x: 40, y: 25 })
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Player{})
        .build();

    rltk::main_loop(context, gs)
}
