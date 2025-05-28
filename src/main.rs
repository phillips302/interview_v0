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
    pub betDescription: String,
    #[serde(flatten)]
    pub other: serde_json::Value,
    pub outcomes: Vec<Outcome>,
}

#[derive(Debug, Deserialize)]
pub struct Outcome {
    #[serde(rename = "type")]
    pub typ: String,
    pub line: Option<f32>,
    pub odds: Option<f32>,
    pub label: Option<String>,
    pub oddsAmerican: Option<String>,
}


#[tokio::main]
async fn main() -> Result<(), Error> {
    let fixture_id = "1022035816";
    let lines = get_betrivers_lines(fixture_id).await?;
    for line in lines {
        println!("bet_key: {} - result: {} - line: {} - odds: {}",
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
    let game: Game = resp.json().await?;

    let mut t0 = "";
    let mut t1 = "";
    for p in &game.participants {
        if p.home { t0 = &p.name; }
        else      { t1 = &p.name; }
    }

    for grp in game.offeringGroups {
        for crit in grp.criterionGroups {
            let name = crit.criterionName.as_str();

            /*** MONEYLINE  ***/
            if name.starts_with("Moneyline") {
                if let Some(offer) = crit.betOffers.get(0) {
                    let period = offer.betDescription.strip_prefix("Moneyline - ")
                        .map(|s| normalize_period(s))
                        .unwrap_or_else(|| "full_time".into());
                    
                    let key = format!("moneyline | {}", period);
                    for oc in &offer.outcomes {
                        let odds: f32 = oc.odds.unwrap_or(0.0);
                        let oddsA = oc.oddsAmerican.clone().unwrap_or_default();
                        let label = oc.label.clone().unwrap_or_default();
                        let result = format!("{} | {}", label, oddsA);
                        lines.push(BetLine {
                            bet_key: key.clone(),
                            line:    0.0,
                            result,
                            odds,
                        });
                    }
                }
            }

            /*** Spread ***/
            else if name.starts_with("Spread") {
                if let Some(offer) = crit.betOffers.get(0) {
                    let period = offer.betDescription.strip_prefix("Spread - ")
                        .map(|s| normalize_period(s))
                        .unwrap_or_else(|| "full_time".into());
            
                    let key = format!("spread | {}", period);
                    for oc in &offer.outcomes {
                        let raw_line: f32 = oc.line.unwrap_or(0.0);
                        let label = oc.label.clone().unwrap_or_default();
                        let signed_line = if label == "t0" { raw_line} else { -raw_line};
                        let odds: f32 = oc.odds.unwrap_or(0.0);            
                        let result = oc.label.clone().unwrap_or_default();
                        lines.push(BetLine {
                            bet_key: key.clone(),
                            line:    signed_line,
                            result,
                            odds,
                        });
                    }
                }
            }

            /*** TOTAL RUNS ***/
            else if name == "Total Runs" {
                for offer in &crit.betOffers {
                    let period = offer
                        .betDescription
                        .strip_prefix("Total Runs - ")
                        .map(|s| normalize_period(s))
                        .unwrap_or_else(|| "full_time".into());
                    for oc in &offer.outcomes {
                        let l = oc.line.unwrap_or(0.0);
                        let result = match oc.typ.as_str() {
                            "OVER"  => "over",
                            "UNDER" => "under",
                            _       => continue,
                        };
                        let result = oc.label.clone().unwrap_or_default();
                        let key = format!("total_runs | {} | {}", result, period);
                        let odds: f32 = oc.odds.unwrap_or(0.0);
                        lines.push(BetLine {
                            bet_key: key.clone(),
                            line:    l,
                            result,
                            odds,
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