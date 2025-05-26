mod pricing;
mod ui;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    ui::run_ui()
}
