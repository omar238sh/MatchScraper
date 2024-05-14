use reqwest::{Client, Response};
use scraper::{Html, Selector};
use std::{fmt::format, io};
use csv::WriterBuilder;
use std::error::Error;
use std::fs::OpenOptions;

#[tokio::main]
async fn main() {

    let mut date = String::new();
    println!("please enter date : ");
    println!("format ==> MM/DD/YYYY ");
    io::stdin().read_line(&mut date).unwrap();
    let link = format(format_args!("https://www.yallakora.com/match-center/%D9%85%D8%B1%D9%83%D8%B2-%D8%A7%D9%84%D9%85%D8%A8%D8%A7%D8%B1%D9%8A%D8%A7%D8%AA?date={}#" , date));
    let client = Client::new();
    let page = client.get(link).send().await.unwrap();
    if page.status() == reqwest::StatusCode::OK {
        println!("We get the site");
        scrap(page).await;
    } else {
        println!("cannot reach the site")
    }
    
}

async fn scrap(page : Response) {
    let src = page.text().await.unwrap();
    let html = Html::parse_fragment(&src);
    // Selectors
    let match_div = Selector::parse("div.teamsData").unwrap();
    let teamA = Selector::parse("div.teamA p").unwrap();
    let teamB = Selector::parse("div.teamB p").unwrap();
    let matchResult = Selector::parse("div.MResult").unwrap();
    let score_span = Selector::parse("span.score").unwrap();
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open("data.csv").unwrap();

        
        let mut csv_writer = WriterBuilder::new().from_writer(file);


    let matches = html.select(&match_div);

    for matche in matches {
        let teamA_val = matche.select(&teamA).next().unwrap().inner_html();
        let teamB_val = matche.select(&teamB).next().unwrap().inner_html();

        let scores: Vec<String> = matche
            .select(&matchResult)
            .flat_map(|result| result.select(&score_span))
            .map(|span| span.inner_html())
            .collect();

        


        

        csv_writer.write_record(&[teamA_val, scores.join(" - ") , teamB_val]).unwrap();
        
    }
    csv_writer.flush().unwrap();

}