# gemini-rust

A Rust client for the Gemini Exchange REST API

see https://docs.gemini.com/rest-api/

# Usage

    let url = "https://api.sandbox.gemini.com";
    let api_key = "<your api key>"
    let api_secret = "<your api secret>";
    let client = GeminiClient::new(&url, &api_key, &api_sec);
    
    let symbols = client.symbols().expect("error with request");
    assert!(symbols.len() > 0, "symbols vector should be > 0 size");
    println!("{:?}", symbols)

# Tests

Copy tests/cfg.example.toml to tests/cfg.toml, and edit the file to enter your Sandbox Gemini API key and secret.
The tests are written explicitly using the sandbox url.
If a non-sandbox API key is used, the tests will fail.

Use the script tests.sh to execute all the tests.
If executing specific test cases, do note the need to use the option "--test-threads=1" ,
otherwise failures will occur due to the test cases running in parallel.

# Features

- [x] Public
- [x] Order Place
- [x] Order Status
- [x] Fee and Volume

# TODO

- [ ] Clearing 
- [ ] Fund Management
- [ ] Approved Addresses
- [ ] Account Admin
- [ ] Session
- [ ] Websockets
