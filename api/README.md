# Rust Lambda API Starter Project

This project is intended as a convenient starting point for creating a Rust API that is hosted in AWS Lambda. The content of this project, as well as the sibling projects in this repository will result in a "Fat Lambda" that will accept all requests intended for the API. Routing will be handled in Rust, and all endpoints will be hosted on the same Lambda.

# Install the required tools

1. [Install Rust](https://www.rust-lang.org/tools/install)
1. [Install Cargo Lambda](https://www.cargo-lambda.info/guide/installation.html)

# Building for release

1. Build the release zip:  
`cargo lambda build --release --output-format zip`
