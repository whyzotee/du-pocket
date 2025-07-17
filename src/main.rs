mod app;

use crate::app::App;

use std::io::Result;

fn main() -> Result<()> {
    let app = App::new();
    app.run()
}
