# Gas Standard Output using Blocknative API

This Rust code demonstrates how to retrieve gas prices using the Blocknative API and displays the standard output.

## Prerequisites

Before running the code, make sure you have the following prerequisites:

- Rust installed on your machine
- Cargo, the Rust package manager
- API key for the Blocknative API

## Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/taimurey/Gas-Prediction/.git
   ```

2. Navigate to the project directory:

   ```bash
   cd your-repository
   ```

3. Open the `Cargo.toml` file and add the following dependencies:

   ```toml
   [dependencies]
   reqwest = "0.11"
   serde_json = "1.0"
   tokio = { version = "1", features = ["full"] }
   ```

4. Save the `Cargo.toml` file and run the following command to install the dependencies:

   ```bash
   cargo build
   ```

## Configuration

1. Open the `main.rs` file.

2. Locate the `auth_header` variable and replace `"put your API key here"` with your Blocknative API key.

   ```rust
   let auth_header = "your-api-key";
   ```

## Usage

To run the code and view the gas prices standard output, execute the following command:

```bash
cargo run
```

The code will continuously fetch gas prices from the Blocknative API and display the entire JSON response to the console. The output will look similar to the following:

```plaintext
{
    "data": {
        "average": {
            "block_number": 13163616,
            "explanation": "Aggregated prices from multiple oracle sources",
            "block_time": 14,
            "fast": "110.0",
            "fastest": "300.0",
            "safe_low": "15.0",
            "standard": "40.0"
        },
        ...
    }
}
```

## License

This project is licensed under the [MIT License](LICENSE).
