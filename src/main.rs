use fast::{System, SystemConfig};

mod fast;

fn main() {
    colog::basic_builder()
        .filter_level(log::LevelFilter::Trace)
        .init();

    System::start(SystemConfig {
        system: fast::FastPlatform::Neuron,
        switch_reporting: fast::SwitchReporting::Read,
        io_net_port_path: "COM5",
    });
}
