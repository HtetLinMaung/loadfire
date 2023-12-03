# LoadFire

LoadFire is a command-line load testing tool built in Rust, designed to perform load tests on web applications. It supports dynamic data loading from CSV or Excel files, allowing users to simulate real-world scenarios by customizing request parameters.

## Features

- Perform HTTP load testing on web applications.
- Support for various HTTP methods (GET, POST, PUT, DELETE, PATCH).
- Load test configurations from YAML files.
- Dynamically generate request payloads using data from CSV or Excel files.
- Cross-platform compatibility.

## Installation

To install LoadFire, ensure you have [Rust and Cargo](https://www.rust-lang.org/tools/install) installed on your system.

Then, clone the repository and build the project:

```bash
git clone https://github.com/your-username/loadfire.git
cd loadfire
cargo build --release
```

The executable will be located in `target/release`.

## Usage

To use LoadFire, create a configuration file in YAML format with your load test parameters. An example configuration file might look like this:

```yaml
url: "http://example.com/api"
method: POST
request_count: 100
headers:
  Content-Type: "application/json"
body: '{"key": "value"}'
data_file: "data.xlsx"
```

Run the load test using:

```bash
./target/release/loadfire -c path/to/your/config.yml
```

Replace `path/to/your/config.yml` with the path to your configuration file.
