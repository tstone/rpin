use fast::{ExpansionBoard, System, SystemConfig};

mod fast;

fn main() {
    colog::basic_builder()
        .filter_level(log::LevelFilter::Trace)
        .init();

    System::start(SystemConfig {
        system: fast::FastPlatform::Neuron,
        switch_reporting: fast::SwitchReporting::Read,
        io_port_path: "COM5",
        exp_port_path: "COM7",
        expansion_boards: vec![ExpansionBoard {
            id: "48",
            leds: vec![8],
        }],
    });
}
