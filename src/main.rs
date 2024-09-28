mod models;
use models::CryptoPrice;
use reqwest::blocking::Client;
use rusqlite::{params, Connection};
use plotters::prelude::*;
use std::error::Error;

fn fetch_crypto_prices() -> Result<Vec<CryptoPrice>, Box<dyn Error>> {
    let url = "https://api.coingecko.com/api/v3/coins/markets?vs_currency=usd";

    // Create a client with a User-Agent
    let client = Client::builder()
        .user_agent("Crysalis/1.0 (https://github.com/your-repo-url)") // Update with your actual repo URL
        .build()?;

    let response = client.get(url).send()?;

    // Check if the response is successful first
    if !response.status().is_success() {
        return Err(Box::from(format!(
            "Failed to fetch data: {}",
            response.status()
        )));
    }

    // Get the response text
    let response_text = response.text()?;

    // Deserialize the JSON response into Vec<CryptoPrice>
    let prices: Vec<CryptoPrice> = serde_json::from_str(&response_text)?;
    Ok(prices)
}

fn init_db() -> rusqlite::Result<Connection> {
    let conn = Connection::open("crypto_prices.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS prices (
            id TEXT PRIMARY KEY,
            symbol TEXT NOT NULL,
            name TEXT NOT NULL,
            current_price REAL NOT NULL
        )",
        [],
    )?;
    Ok(conn)
}

fn store_prices(prices: &[CryptoPrice], conn: &Connection) -> rusqlite::Result<()> {
    for price in prices {
        conn.execute(
            "INSERT OR REPLACE INTO prices (id, symbol, name, current_price) VALUES (?1, ?2, ?3, ?4)",
            params![price.id.clone(), price.symbol.clone(), price.name.clone(), price.current_price],
        )?;
    }
    Ok(())
}

fn print_prices(conn: &Connection) -> rusqlite::Result<()> {
    let mut stmt = conn.prepare("SELECT symbol, current_price FROM prices")?;
    let price_iter = stmt.query_map([], |row| {
        Ok((row.get::<_, String>(0)?, row.get::<_, f64>(1)?))
    })?;

    for price in price_iter {
        let (symbol, current_price) = price?;
        println!("{}: ${:.2}", symbol, current_price);
    }
    Ok(())
}




// fn plot_prices(conn: &Connection) -> Result<(), Box<dyn Error>> {
//     let mut stmt = conn.prepare("SELECT symbol, current_price FROM prices")?;
//     let price_iter = stmt.query_map([], |row| {
//         Ok((row.get::<_, String>(0)?, row.get::<_, f64>(1)?))
//     })?;
//
//     // Prepare data for plotting
//     let mut symbols = Vec::new();
//     let mut prices = Vec::new();
//
//     for price in price_iter {
//         let (symbol, current_price) = price?;
//         symbols.push(symbol);
//         prices.push(current_price);
//     }
//
//     // Create a chart with a "Dracula" theme
//     let root = BitMapBackend::new("crypto_prices.png", (800, 600)).into_drawing_area();
//     root.fill(&RGBColor(40, 42, 54))?; // Background color for Dracula theme
//
//     let mut chart = ChartBuilder::on(&root)
//         .caption("Cryptocurrency Prices", ("sans-serif", 50).into_font().color(&WHITE)) // White text color
//         .margin(5)
//         .x_label_area_size(50) // Increased area for labels
//         .y_label_area_size(40) // Increased area for y label to avoid overlap
//         .build_cartesian_2d(0..symbols.len() - 1, 0.0..*prices.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap())?;
//
//     // Create a vector of indices for x labels
//     let x_labels: Vec<usize> = (0..symbols.len()).step_by(2).collect(); // Show every second label
//
//     chart
//         .configure_mesh()
//         .x_desc("Cryptocurrency")
//         .y_desc("Price (USD)")
//         .x_labels(x_labels.len()) // Set the number of labels
//         .x_label_formatter(&|x| {
//             if *x < symbols.len() {
//                 symbols[*x].to_string().to_uppercase()
//             } else {
//                 "".to_string()
//             }
//         })
//         .y_label_formatter(&|y| format!("${:.2}", y))
//         .label_style(&WHITE) // Set label color to white
//         .draw()?;
//
//     // Draw the price line with emerald green color
//     chart.draw_series(LineSeries::new(
//         (0..prices.len()).zip(prices.iter()).map(|(x, &y)| (x, y)),
//         &RGBColor(80, 200, 120), // Emerald green color
//     ))?
//         .label("Price")
//         .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 1, y)], &RGBColor(80, 200, 120))); // Corrected line
//
//     chart
//         .configure_series_labels()
//         .border_style(&WHITE) // White border for the legend
//         .draw()?;
//
//     println!("Chart saved as crypto_prices.png");
//     Ok(())
// }

fn plot_prices(conn: &Connection) -> Result<(), Box<dyn Error>> {
    let mut stmt = conn.prepare("SELECT symbol, current_price FROM prices")?;
    let price_iter = stmt.query_map([], |row| {
        Ok((row.get::<_, String>(0)?, row.get::<_, f64>(1)?))
    })?;

    // Prepare data for plotting
    let mut symbols = Vec::new();
    let mut prices = Vec::new();

    for price in price_iter {
        let (symbol, current_price) = price?;
        symbols.push(symbol);
        prices.push(current_price);
    }

    // Create a chart with a "Dracula" theme
    let root = BitMapBackend::new("crypto_prices.png", (800, 600)).into_drawing_area();
    root.fill(&RGBColor(40,42,54))?; // Background color for Dracula theme

    let mut chart = ChartBuilder::on(&root)
        .caption("Cryptocurrency Prices", ("sans-serif", 50).into_font().color(&WHITE)) // White text color
        .margin(10) // Increased margin for better spacing
        .x_label_area_size(50) // Increased area for labels
        .y_label_area_size(50) // Increased area for y label to avoid overlap
        .build_cartesian_2d(0..symbols.len() - 1, 0.0..*prices.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap())?;

    // Create a vector of indices for x labels
    let x_labels: Vec<usize> = (0..symbols.len()).step_by(2).collect(); // Show every second label

    chart
        .configure_mesh()
        .x_desc("Cryptocurrency")
        .y_desc("Price (USD)")
        .x_labels(x_labels.len()) // Set the number of labels
        .x_label_formatter(&|x| {
            if *x < symbols.len() {
                symbols[*x].to_string().to_uppercase()
            } else {
                "".to_string()
            }
        })
        .y_label_formatter(&|y| format!("${:.2}", y))
        .label_style(&WHITE) // Set label color to white
        .disable_mesh()
        .draw()?;

    // Draw the price line with emerald green color
    let price_series = chart.draw_series(LineSeries::new(
        (0..prices.len()).zip(prices.iter()).map(|(x, &y)| (x, y)),
        &RGBColor(255, 184, 108), // teal
    ))?
        .label("Price")
        .legend(|(x, y)| {
            // Create a line for the legend
            PathElement::new(vec![(x, y), (x + 1, y)], &RGBColor(255, 184, 108))
        });

    chart
        .configure_series_labels()
        .border_style(&WHITE) // White border for the legend
        .draw()?;

    println!("Chart saved as crypto_prices.png");
    Ok(())
}


fn main() -> Result<(), Box<dyn Error>> {
    let conn = init_db()?;

    let prices = fetch_crypto_prices()?;
    store_prices(&prices, &conn)?;

    println!("Current Cryptocurrency Prices:");
    print_prices(&conn)?;

    // Call the visualization function
    plot_prices(&conn)?;

    Ok(())
}
