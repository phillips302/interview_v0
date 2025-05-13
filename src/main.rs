mod betline;

#[tokio::main]
async fn main() {
    let lines = get_betrivers_lines("123456").await;
    for line in lines {
        println!("{} - {} - {} - {}", line.bet_key, line.result, line.line, line.odds);
    }
}

/// fixture_id: the game id on betrivers for the desired game
async fn get_betrivers_lines(fixture_id: &str) -> Vec<betline::BetLine> {
    let mut lines = Vec::new();

    // pull lines from betrivers api and clean appropriately in BetLine struct

    lines   
}