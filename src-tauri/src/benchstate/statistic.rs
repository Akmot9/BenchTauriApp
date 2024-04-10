#[derive(Debug)]
pub struct Statistic {
    pub name: String, // Le nom du processus ou une description
    pub rss: u64,     // Resident Set Size en kilo-octets (KB)
    pub mem: f32,     // Pourcentage de la mémoire utilisée
    pub vsz: u64,     // Virtual Memory Size en kilo-octets (KB)
    pub cpu: f32,     // Pourcentage du CPU utilisé
    pub ni: i32,      // Nice value indiquant la priorité du processus
}
