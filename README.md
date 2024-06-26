
# Match API

This is a simple API built with Rust using the Rocket framework. It allows users to retrieve match data based on a specified date.

## Features

- Retrieve match data by date in the format `/DD/MM/YYYY`.
- Simple and easy-to-use API endpoints.

## Setup

### Prerequisites

- Rust installed (version 1.65.0 or later)
- Cargo (Rust's package manager and build tool)
- Git

### Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/your-username/match-api.git
   cd match-api
   ```

2. Build the project:

   ```bash
   cargo build
   ```

3. Run the project:

   ```bash
   cargo run
   ```

### Running the Server

Once the server is running, you can access it at `http://localhost:8000`.

## Endpoints

- **GET** `/`

  Returns a welcome message and information about the API.

  Example response:

  ```json
  {
    "message": "Hello, you are using my API. You can get match data by adding the parameter /DD/MM/YYYY."
  }
  ```

- **GET** `/{DD}/{MM}/{YYYY}`

  Retrieve match data for the specified date.

  Example URL: `http://localhost:8000/16/06/2024`

  Example response:

  ```json
  [
    {
      "message": "connected",
      "team1": "Team A",
      "team2": "Team B",
      "scores": "2-1"
    },
    {
      "message": "connected",
      "team1": "Team C",
      "team2": "Team D",
      "scores": "1-1"
    }
  ]
  ```

## Contributing

1. Fork the repository.
2. Create a new branch (`git checkout -b feature-branch`).
3. Make your changes.
4. Commit your changes (`git commit -am 'Add new feature'`).
5. Push to the branch (`git push origin feature-branch`).
6. Create a new Pull Request.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contact

- GitHub: [your-username](https://github.com/your-username)
- Email: your-email@example.com

