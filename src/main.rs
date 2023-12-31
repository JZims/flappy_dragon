use  bracket_lib::prelude::*;

enum GameMode {
    Menu, 
    Playing, 
    End,
}

struct State {
    mode: GameMode,
}

impl State {

    fn new() -> Self {
        State {
            mode: GameMode::Menu,
        }
    }
    
    fn play (&mut self, ctx: &mut BTerm){
        //TODO: Fill in this stub later
        self.mode = GameMode::End;
    }
}

impl GameState for State {
    // Bridge between Engine and Game
     fn tick(&mut self, ctx: &mut BTerm) {
       match self.mode {
            GameMode::Menu => self.main_menu(ctx),
            GameMode::End => self.dead(ctx),
            GameMode::Playing => self.play(ctx),
       }
     }
}


fn main() -> BError {

    let context = BTermBuilder::simple80x50()
    .with_title("Flappy Dragon")
    .build()?;

    main_loop(context, State::new())

}
