pub struct Dataset {
    pub name: String,
    pub used: String,
    pub available: String,
    pub referred: String,
    pub mountpoint: String,
}

pub type Datasets = Vec<Dataset>;
