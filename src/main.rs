mod betline;
use reqwest::Error;
use crate::betline::BetLine;
use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize)]
pub struct Game {
    pub id: u64,
    pub name: String,
    pub participants: Vec<Participant>,
    pub offeringGroups: Vec<OfferingGroup>,
}

#[derive(Debug, Deserialize)]
pub struct Participant {
    pub id: u64,
    pub name: String,
    pub home: bool,
}

#[derive(Debug, Deserialize)]
pub struct OfferingGroup {
    pub categoryName: String,
    pub criterionGroups: Vec<CriterionGroup>,
}

#[derive(Debug, Deserialize)]
pub struct CriterionGroup {
    pub criterionName: String,
    pub betOffers: Vec<BetOffer>,
}

#[derive(Debug, Deserialize)]
pub struct BetOffer {
    pub id: u64,
    #[serde(flatten)]
    pub other: serde_json::Value,
    pub outcomes: Vec<Outcome>,
}

#[derive(Debug, Deserialize)]
pub struct Outcome {
    #[serde(rename = "type")]
    pub typ: String,
    pub line: Option<f32>,
    pub odds: f32,
    pub oddsAmerican: String,
    pub oddsFractional: String, 
}


#[tokio::main]
async fn main() -> Result<(), Error> {
    let fixture_id = "1022035826";
    let lines = get_betrivers_lines(fixture_id).await?;
    for line in lines {
        println!("bet_key: {}\nresult: {}\nline: {}\nodds: {}",
        line.bet_key, line.result, line.line, line.odds);
    }
    Ok(())
}

async fn get_betrivers_lines(fixture_id: &str) -> Result<Vec<BetLine>, Error> {
    let mut lines = Vec::new();
    let url = format!(
        "https://in.betrivers.com/api/service/sportsbook/offering/listview/details\
?eventId={}&cageCode=812",
fixture_id);

    let resp = reqwest::get(&url).await?.error_for_status()?;  

    //let json: Value = resp.json().await?;
    let game: Game = resp.json().await?;
    println!("Game: {:?}", game);

    let mut t0 = "";
    let mut t1 = "";
    for p in &game.participants {
        if p.home { t0 = p.name; }
        else      { t1 = p.name; }
    }

    for grp in game.offeringGroups {
        for crit in grp.criterionGroups {
            let name = crit.criterionName.as_str();

            /*** MONEYLINE  ***/
            if name.starts_with("Moneyline") {
                let period = name
                    .strip_prefix("Moneyline - ")
                    .map(|s| normalize_period(s))
                    .unwrap_or_else(|| "full_time".into());
                let key = format!("moneyline | {}", period);
    
                if let Some(offer) = crit.betOffers.get(0) {
                    for oc in &offer.outcomes {
                        let result = match oc.typ.as_str() {
                            "T0" => "t0",
                            "T1" => "t1",
                            _    => continue,
                        };
                        lines.push(BetLine {
                            bet_key: key.clone(),
                            line:    0.0,
                            result:  result.to_string(),
                            odds:    oc.odds,
                        });
                    }
                }
            }

            /*** Spread ***/
            else if name.starts_with("Spread") {
                // extract the period suffix just once
                let period = name
                    .strip_prefix("Spread - ")
                    .map(|s| normalize_period(s))
                    .unwrap_or_else(|| "full_time".into());
                let key = format!("spread | {}", period);
    
                for offer in &crit.betOffers {
                    for oc in &offer.outcomes {
                        // oc.line holds the signed handicap for each side
                        let raw_line = oc.line.unwrap_or(0.0);
                        // T0 is away→positive, T1 is home→negative
                        let signed_line = match oc.typ.as_str() {
                            "T0" =>  raw_line,
                            "T1" => -raw_line,
                            _    => continue,
                        };
                        let result = match oc.typ.as_str() {
                            "T0" => "t0",
                            "T1" => "t1",
                            _    => unreachable!(),
                        };
                        lines.push(BetLine {
                            bet_key: key.clone(),
                            line:    signed_line,
                            result:  result.to_string(),
                            odds:    oc.odds,
                        });
                    }
                }
            }

            /*** TOTAL RUNS ***/
            else if name == "Total Runs" {
                let period = "full_time"; // or derive from crit if you ever see suffixes
                let key = format!("total_runs | combined | {}", period);
    
                for offer in &crit.betOffers {
                    for oc in &offer.outcomes {
                        let l = oc.line.unwrap_or(0.0);
                        let result = match oc.typ.as_str() {
                            "OVER"  => "over",
                            "UNDER" => "under",
                            _       => continue,
                        };
                        lines.push(BetLine {
                            bet_key: key.clone(),
                            line:    l,
                            result:  result.to_string(),
                            odds:    oc.odds,
                        });
                    }
                }
            }
        }
    }

    Ok(lines)  
}

fn normalize_period(raw: &str) -> String {
    let p = raw.to_lowercase().replace(' ', "_");
    if let Some(tail) = p.strip_prefix("first_") {
        return format!("1st_{}", tail);
    }
    if let Some(tail) = p.strip_prefix("middle_") {
        return format!("middle_{}", tail);
    }
    if let Some(tail) = p.strip_prefix("last_") {
        return format!("last_{}", tail);
    }
    p
}