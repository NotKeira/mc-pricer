pub struct PricingConfig {
    pub flat_rate: f64,
    pub per_gb_rate: f64,
    pub per_player_rate: f64,
    pub per_world_rate: f64,
    pub per_plugin_rate: f64,
    pub per_mod_rate: f64,
    pub per_server_rate: f64,
}

impl PricingConfig {
    pub fn new() -> Self {
        Self {
            flat_rate: 2.0,
            per_gb_rate: 0.5,
            per_player_rate: 0.1,
            per_world_rate: 0.2,
            per_plugin_rate: 0.3,
            per_mod_rate: 0.4,
            per_server_rate: 1.5,
        }
    }

    pub fn calculate_cost(
        &self,
        ram_gb: u32,
        player_slots: u32,
        worlds: u32,
        plugins: u32,
        mods: u32,
        servers: u32,
    ) -> f64 {
        self.flat_rate
            + self.per_gb_rate * (ram_gb as f64)
            + self.per_player_rate * (player_slots as f64)
            + self.per_world_rate * (worlds as f64)
            + self.per_plugin_rate * (plugins as f64)
            + self.per_mod_rate * (mods as f64)
            + self.per_server_rate * (servers as f64)
    }
}
