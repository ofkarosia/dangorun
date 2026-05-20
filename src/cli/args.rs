use argh::FromArgs;

#[derive(Debug, FromArgs)]
/// Wuthering Waves Dangorun (Cubie Derby) Simulator
pub(super) struct Args {
    /// run simulation with debug logs from the given seed
    #[argh(option)]
    pub debug: Option<u64>,
    /// run simulation with debug logs from random seed
    #[argh(switch)]
    pub sample: bool,
}
