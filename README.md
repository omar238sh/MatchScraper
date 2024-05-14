
# Yallakora Scraper

A simple web scraper written in Rust using the reqwest and scraper crates to scrape match data from Yallakora website and save it to a CSV file.

## Introduction

This project is a Rust application that scrapes match data from the Yallakora website based on the provided date and saves it to a CSV file. It allows users to input a date in the MM/DD/YYYY format and retrieves the match data for that date.

## Features

- Retrieves match data from the Yallakora website based on the provided date.
- Scrapes team names and match results.
- Saves the scraped data to a CSV file for further analysis.

## Installation

To install and run this project, follow these steps:

1. Clone the repository:

```
git clone https://github.com/omar238sh/MatchScraper.git
```

2. Build the project:

```
cargo build
```

3. Run the project:

```
cargo run
```

## Usage

When prompted, enter the date in the MM/DD/YYYY format to retrieve match data for that date from the Yallakora website. The scraped data will be saved to a file named `data.csv` in the project directory.




## Acknowledgements

- This project uses the reqwest and scraper crates for web scraping.
- Special thanks to the Yallakora website for providing the match data.




