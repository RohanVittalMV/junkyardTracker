# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Common Development Commands

### Build and Run
- `cargo build` - Build the project
- `cargo run` - Run the main application
- `cargo test` - Run all tests
- `cargo check` - Check compilation without building

### Testing
- `cargo test` - Run all tests
- `cargo test it_works` - Run specific test by name

## Project Architecture

This is a Rust junkyard vehicle tracker that scrapes Pick-n-Pull inventory data using the Firecrawl API.

### Core Components

**Main Application Flow (src/main.rs:10-46)**
- Loads FIRECRAWL_API_KEY from environment
- Creates FirecrawlClient instance
- Generates Pick-n-Pull search URLs
- Crawls webpages and parses results

**Web Scraping Layer**
- `FirecrawlClient` (src/firecrawl_client.rs) - HTTP client wrapper for Firecrawl API
- `PicknPullSearch` (src/pick_n_pull.rs) - URL generation for Pick-n-Pull searches
- Uses reqwest for HTTP requests with JSON serialization

**Data Models**
- `JunkyardItem` (src/models.rs:4-11) - Core data structure for vehicle inventory
- Uses serde for JSON serialization/deserialization
- Includes chrono for date handling

**Parser Module**
- `parse_junkyard_page` (src/parser.rs:3-5) - Currently incomplete, intended for HTML parsing

### Key Dependencies
- `reqwest` - HTTP client with JSON support
- `tokio` - Async runtime
- `serde` / `serde_json` - Serialization
- `chrono` - Date/time handling

### Environment Setup
Requires `FIRECRAWL_API_KEY` environment variable for the Firecrawl API.

### Current State
The parser module is incomplete (empty function body in src/parser.rs:3-5). The application can generate search URLs and make API calls but cannot yet parse the returned HTML data.