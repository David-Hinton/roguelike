// pulling in types and the namespaces from rltk
use rltk::{GameState, Rltk, RGB};
use specs::prelude::*;

mod components;
use components::*;

mod map;
use map::*;

mod player;
use player::*;

pub mod rect;


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

    gs.ecs.insert(new_map_rooms_and_corridors());

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
