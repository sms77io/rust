<img src="https://www.seven.io/wp-content/uploads/Logo.svg" width="250" />

# Official API Client for Rust

## Installation

`cargo add seven-client`

### Example

```rust
use seven_client::client::Client;
use seven_client::balance::Balance;

fn main() {
    let client = Client::new("MySecretApiKeyFromSeven.io!".to_string());
    let resource = Balance::new(client);
    let result = resource.get();
    println!("Success: {}", result.is_ok());
    let response = result.unwrap();
    println!("Balance: {} {}", response.amount, response.currency);
}
```

#### Support

Need help? Feel free to [contact us](https://www.seven.io/en/company/contact).

##### License

[![MIT](https://img.shields.io/badge/License-MIT-teal.svg)](LICENSE).
