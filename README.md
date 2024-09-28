
# Crysalis

Crysalis is a Rust-based application designed to fetch and visualize cryptocurrency prices. The project utilizes various libraries to handle HTTP requests, database interactions, and data visualization, providing an elegant and modern interface for monitoring cryptocurrency trends.

## Features

- Fetches real-time cryptocurrency prices from CoinGecko API.
- Stores price data in a local SQLite database.
- Visualizes the price data with an appealing graph using Plotters.
- Supports easy extensions for additional features like historical data analysis.

## Technologies Used

- **Rust**: The primary programming language used for development.
- **Reqwest**: For making HTTP requests to fetch cryptocurrency data.
- **Rusqlite**: To interact with SQLite for data storage.
- **Plotters**: For creating visualizations of the cryptocurrency prices.

## Installation

To get started with Crysalis, follow these steps:

1. Ensure you have [Rust](https://www.rust-lang.org/tools/install) installed on your system.
2. Clone the repository:
   ```bash
   git clone https://github.com/your-username/Crysalis.git
   cd Crysalis
   ```
3. Install the required dependencies:
   ```bash
   cargo build
   ```

## Usage

To run the application and fetch cryptocurrency prices, execute:
```bash
cargo run
```

## Visualization
![Cryptocurrency Prices Visualization](https://github.com/user-attachments/assets/579d6c5d-805b-4f7c-be67-6d42ecd9ba89)

## License

This project is licensed under the MIT License.

## Author

Abhiraj Bhattashali
