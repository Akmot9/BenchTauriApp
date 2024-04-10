pub mod statistic;
use statistic::Statistic;
pub struct BenchState {
    pub statisics: Vec<Statistic>
}

impl Default for BenchState {
    fn default() -> Self {
        Self::new()
    }
}

impl BenchState {
    // Constructeur pour initialiser `SonarState`
    pub fn new() -> BenchState {
        BenchState {
            statisics: Vec::new(),
            
        }
    }
}