mod fast;

fn main() {
    colog::basic_builder()
        .filter_level(log::LevelFilter::Debug)
        .init();

    fast::System::start("COM5");
}
