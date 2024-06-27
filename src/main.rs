#![allow(non_snake_case)]

use reqwest::Client;
use scraper::{Html, Selector};
#[macro_use] extern crate rocket;
use rocket::serde::json::Json;
use serde::Serialize;


#[derive(Serialize)]
struct MatchResponse {
    message : String,
    team1 : String,
    scores : String,
    team2 : String
}

async fn scrap(date : String) -> Vec<[String; 3]> {
   
    let mut finalResult: Vec<[String; 3]> = Vec::new();

    let link = format_args!("https://www.yallakora.com/match-center/%D9%85%D8%B1%D9%83%D8%B2-%D8%A7%D9%84%D9%85%D8%A8%D8%A7%D8%B1%D9%8A%D8%A7%D8%AA?date={}#" , date).to_string();
    let client = Client::new();
    let page = client.get(link).send().await.unwrap();

    let src = page.text().await.unwrap();
    let html = Html::parse_fragment(&src);
    // Selectors
    let match_div = Selector::parse("div.teamsData").unwrap();
    let teamA = Selector::parse("div.teamA p").unwrap();
    let teamB = Selector::parse("div.teamB p").unwrap();
    let matchResult = Selector::parse("div.MResult").unwrap();
    let score_span = Selector::parse("span.score").unwrap();


    let matches = html.select(&match_div);

    for matche in matches {
        let teamA_val = matche.select(&teamA).next().unwrap().inner_html();
        let teamB_val = matche.select(&teamB).next().unwrap().inner_html();

        let scores: Vec<String> = matche
            .select(&matchResult)
            .flat_map(|result| result.select(&score_span))
            .map(|span| span.inner_html())
            .collect();
        
        finalResult.push([teamA_val, scores.join("_") ,teamB_val]);
    }
    finalResult
}

#[get("/<DD>/<MM>/<YYYY>")]
async fn matches(DD : String , MM : String , YYYY : String) -> Json<Vec<MatchResponse>> {

    let date = format!("{}/{}/{}", MM , DD , YYYY);
    let a = scrap(date).await;

    let modResponse: Vec<MatchResponse> = a.iter().map(|x| { MatchResponse {
        message : "connected".to_string(),
        team1 : x[0].clone(),
        team2 : x[2].clone(),
        scores : x[1].clone()
    } }).collect();
    Json(modResponse)
}

#[get("/")]
fn start() -> String {
    String::from("Hello! You are using my API to get match data.\n\n\
                  To fetch match data, use the following format:\n\
                  /DD/MM/YYYY\n\n\
                  Example: /16/06/2024")
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![matches , start])
}
