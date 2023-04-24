#[derive(clap::ValueEnum, Clone, Debug, strum_macros::Display)]
#[clap(rename_all = "verbatim")]
pub enum Location {
    Sollentuna,
    Uppsala,
}
impl Location {
    pub fn coordinates(&self) -> (f32, f32) {
        // (lon, lat)
        match self {
            Self::Sollentuna => (17.950055, 59.429316),
            Self::Uppsala => (17.638927, 59.858562),
        }
    }
}
