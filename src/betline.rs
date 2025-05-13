pub struct BetLine {
    pub bet_key: String,            // unique key for each bet see readme for more details
    pub result: String,             // result of the bet either (under/over or t0/t1 where t0 = away team and t1 = home team)
    pub line: f32,                  // line value i.e -1.5, 2.5
    pub odds: f32,                  // american odds i.e -110, 150
}