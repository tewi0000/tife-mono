pub mod screen;
pub mod state;

use screen::test::TestScreen;
use state::State;
use wcore::{App, AppConfig, graphics::{context::GraphicsContext, screen::Screen}};

fn main() {
    let app = App::default();

    let config = AppConfig {
        title : String::from("tife"),
        
        .. Default::default()
    };

    let state_lambda = |graphics: &mut GraphicsContext| {
        return State::new();
    };

    let screens_lambda = |graphics: &mut GraphicsContext, screens: &mut Vec<Box<dyn Screen<State>>>| {
        screens.push(Box::new(TestScreen::new(graphics)));
    };

    app.run(config, state_lambda, screens_lambda);
}
