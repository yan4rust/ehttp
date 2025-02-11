fn main() -> eframe::Result<()> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let locale = sys_locale::get_locale().or_else(||Some("en-US".to_string())).unwrap();
    println!("locale: {}",&locale);
    eframe::run_native(
        "ehttp demo",
        Default::default(),
        Box::new(|_cc| Box::<example_eframe::DemoApp>::default()),
    )
}
