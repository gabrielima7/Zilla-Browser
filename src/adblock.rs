use adblock::{lists::ParseOptions, request::Request, Engine, FilterSet};
use lazy_static::lazy_static;
use reqwest::blocking::Client;
use std::sync::Mutex;

// A simple adblock engine that will be initialized once and used globally.
pub struct AdblockEngine {
    pub engine: Engine,
    pub client: Client,
}

impl AdblockEngine {
    fn new() -> Self {
        println!("Initializing Adblock engine...");

        // For now, use a small set of hardcoded rules.
        // In the future, we will fetch this from a URL like EasyList.
        let rules = vec![
            // A simple rule to block a common ad server pattern
            "||doubleclick.net^".to_string(),
            // A rule to block a specific image
            "/ad.png".to_string(),
        ];

        let mut filter_set = FilterSet::new(true);
        filter_set.add_filters(&rules, ParseOptions::default());

        let engine = Engine::from_filter_set(filter_set, true);
        let client = Client::new();

        println!("Adblock engine initialized.");

        AdblockEngine { engine, client }
    }

    // A method to check a request.
    // We will call this from our custom protocol handler.
    pub fn check_should_block(&self, url: &str, source_url: &str, request_type: &str) -> bool {
        let request = match Request::new(url, source_url, request_type) {
            Ok(req) => req,
            Err(e) => {
                eprintln!("Failed to create adblock request: {}", e);
                return false; // Don't block if we can't even parse the request
            }
        };

        let result = self.engine.check_network_request(&request);

        // The `matched` field is true if any rule matched and the request should be blocked.
        result.matched
    }
}

// Use lazy_static to create a single, thread-safe, globally accessible instance.
// We wrap it in a Mutex to solve the Sync issue with the adblock engine.
lazy_static! {
    pub static ref ADBLOCK_ENGINE: Mutex<AdblockEngine> = Mutex::new(AdblockEngine::new());
}
