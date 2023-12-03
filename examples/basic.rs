use mage_core::{
    load_font_image, run, App, Colour, Config, Font, PresentInput, PresentResult, TickInput,
    TickResult,
};
use mage_ui::render_ui;
use tracing::info;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    color_eyre::install().unwrap();
    let filter = EnvFilter::from_default_env()
        .add_directive("wgpu=warn".parse().unwrap())
        .add_directive("mage=trace".parse().unwrap());
    tracing_subscriber::fmt::fmt()
        .without_time()
        .compact()
        .with_env_filter(filter)
        .init();

    info!("Starting...");

    let app = TestApp::new();
    let config = Config {
        font: Font::Custom(load_font_image(include_bytes!("font3.png")).unwrap()),
        ..Default::default()
    };

    let _ = run(app, config).await;
}

struct TestApp {}

impl TestApp {
    fn new() -> Self {
        Self {}
    }
}

impl App for TestApp {
    fn tick(&mut self, _tick_input: TickInput) -> TickResult {
        TickResult::Continue
    }

    fn present(&mut self, mut present_input: PresentInput) -> PresentResult {
        render_ui(&mut present_input, |ctx| {
            ctx.print("Hello, world!", Colour::Yellow.into(), Colour::Black.into());
            ctx.print(
                "Goodbye cruel world!",
                Colour::LightRed.into(),
                Colour::Black.into(),
            );
        });
        mage_core::PresentResult::Changed
    }
}
