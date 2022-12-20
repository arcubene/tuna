# Tuna

Tuna is a general purpose Discord bot written in Rust and hosted on [Shuttle.rs](https://www.shuttle.rs/). It aims to provide a wide range of features and functionality, while still being lightweight and efficient. With Tuna, you can take advantage of the modular design to easily add new features. 
## Features
- Lightweight
- Modular design allowing for easy addition of new features
- Hosted using [Shuttle.rs](https://www.shuttle.rs/)

## Installation

Tuna is designed to be easy to install and set up. Simply follow these steps:

- Clone the Tuna repository: `git clone https://github.com/arcHexagon/tuna.git`.
- Enter the Tuna directory: `cd tuna`.
- Create a `Secrets.toml` file in the root directory and add your Discord bot token as follows: `DISCORD_TOKEN = "your_token_here"`, along with your owner ID `OWNER_ID = "your_discord_id_here"`.
- Install `cargo-shuttle` with `cargo install cargo-shuttle`.
- Run the bot with `cargo shuttle run`.

## Usage

Once Tuna is up and running, you can use the following slash commands:
- `/roll`: Roll a random number.
- `/xkcd`: Grab a xkcd comic.
- `/anime waifu`: Find a waifu.
- `/anime quote`: Grab a quote from a random anime.

## Contributing

We welcome contributions to Tuna! If you would like to report a bug or request a new feature, please open an issue on the GitHub repository.

If you would like to contribute code, please follow these steps:

- Fork the repository.
- Create a new branch for your changes.
- Make your changes.
- Test your changes thoroughly.
- Submit a pull request.

## License

Tuna is licensed under the MIT License. See [LICENCE](LICENCE.md) for more information.
