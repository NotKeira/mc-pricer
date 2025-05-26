# MC Cost Estimator

A simple, modular TUI application for estimating Minecraft PaperMC hosting costs.

## Features

- Written in Rust
- TUI powered by `ratatui` and `crossterm`
- Modular pricing logic
- Simple flat + per-GB pricing model

## Usage

1. Clone or create the project:

   ```sh
   cargo run
   ```

2. Displays RAM allocation and calculated hosting price.

## Pricing Model

The pricing model is designed to be simple and modular, allowing easy adjustments to reflect different hosting scenarios. It includes:

- **Flat Rate**: Base cost for hosting
- **Per GB Rate**: Cost per GB of RAM allocated
- **Per Player Rate**: Cost per player slot
- **Per World Rate**: Cost per world hosted
- **Per Plugin Rate**: Cost per plugin installed
- **Per Mod Rate**: Cost per mod installed
- **Per Server Rate**: Cost per server instance

## Pricing Formula

The total cost is calculated using this formula:

```
total_cost = flat_rate + (per_gb_rate * ram_allocation) + (per_player_rate * player_slots) + (per_world_rate * worlds) + (per_plugin_rate * plugins) + (per_mod_rate * mods) + (per_server_rate * servers)
```

Where each variable corresponds to:

- `flat_rate`: Base cost for hosting — £2.00
- `per_gb_rate`: Cost per GB of RAM allocated — £0.50
- `player_slots`: Number of player slots — £0.10 per slot
- `worlds`: Number of worlds hosted — £0.20 per world
- `plugins`: Number of plugins installed — £0.30 per plugin
- `mods`: Number of mods installed — £0.40 per mod
- `servers`: Number of server instances — £1.50 per server

## Contributing

Contributions are welcome! Please submit a pull request or open an issue for any changes you'd like to propose.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
