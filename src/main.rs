mod pricing;
mod ui;

use ui::UI;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut ui = UI::new()?;
    ui.run()?;
    Ok(())
}
