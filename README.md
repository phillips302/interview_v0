# AVO Scraping Interview

The purpose of this interview is to do a test run at the job you will be asked to persue by filling out the get_betrivers_lines function
definition in the main file. You will need to set up this repository to print out betting information for any MLB game pulled from the 
BetRivers sportsbook. If you do not have access to BetRivers please try a VPN or ask us for assistance. Step 1 will be locating the backend
api call for BetRivers in order to gather the odds of a single MLB game. Then defining the get_betrivers function will require 2 components:

1. Pulling in the JSON data from the BetRivers backend api endpoint
2. Doing some minor cleaning to get it into the standardized form

The standard form for bets is defined by the BetLine struct in betline.rs. To explain further the 3 bet types we want for this interview 
are as follows:

1. Moneylines
    - bet_key = "moneyline | {period}"
    - line = 0.0
    - result = "t0/t1"

2. Spreads
    - bet_key = "spread | {period}"
    - line = 5.5/-5.5
    - result = "t0/t1"

3. Total Runs
    - bet_key = "total_runs | {t0/t1/combined} | {period}"
    - line = 2.5/3.5
    - result = "under/over"

Note: t0 = away team and t1 = home team

The valid periods are the following:
    - full_time
    - 1st_inning
    - 2nd_inning
    - 3rd_inning
    - 4th_inning
    - 5th_inning
    - 6th_inning
    - 7th_inning
    - 8th_inning
    - 9th_inning
    - 1st_3_innings
    - 1st_5_innings
    - 1st_7_innings
    - middle_3_innings
    - last_3_innings

If there are any further questions about the standard form please contact one of us. To review, we will be running this function to see
what lines are gathered and how they are cleaned, however we will also be keeping track of overall code organization.
