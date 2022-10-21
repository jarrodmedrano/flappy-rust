use bracket_lib::prelude::*;

// START: use
use bracket_lib::prelude::*;
// END: use

// START: state
struct State {}
// END: state

// START: gamestate
impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print(1, 1, "Hello, Bracket Terminal!");
    }
}
// END: gamestate

// START: main

// START: mainfn
fn main() -> BError {
// END: mainfn

// START: builder
    let context = BTermBuilder::simple80x50()
        .with_title("Flappy Dragon")
        .build()?;
// END: builder

// START: mainloop
    main_loop(context, State{})
// END: mainloop
}
// END: main