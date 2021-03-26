![Sms77.io Logo](https://www.sms77.io/wp-content/uploads/2019/07/sms77-Logo-400x79.png "Sms77.io Logo")

# Official API Client for Rust

## Installation

Add to `Cargo.toml`

```toml
[dependencies]
sms77-client = "0.1.0"
```

### Example

```rust
let client = Client::new("MySecretApiKeyFromSms77.io!".to_string());
println!("Balance: {}", client.get().is_ok());
```

#### Support

Need help? Feel free to [contact us](https://www.sms77.io/en/company/contact/).

##### License

[![MIT](https://img.shields.io/badge/License-MIT-teal.svg)](./LICENSE).