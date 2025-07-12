use crate::models::JunkyardItem;
use regex::Regex;
use chrono::{DateTime, Utc};

pub fn parse_junkyard_page(markdown: &str, source_url: &str) -> Vec<JunkyardItem> {
    let mut items = Vec::new();
    
    // Check if no vehicles were found
    if markdown.contains("### No Vehicles Found") {
        println!("No vehicles found in inventory for this search");
        return items;
    }
    
    // Find the "Matching Vehicles" section
    if let Some(matching_section_start) = markdown.find("## Matching Vehicles") {
        let matching_section = &markdown[matching_section_start..];
        
        // Look for the table with vehicle data
        // The table has columns: Photo | Year | Make | Model | Row | Set Date
        let table_regex = Regex::new(r"\|\s*([^|]+)\s*\|\s*(\d{4})\s*\|\s*([^|]+)\s*\|\s*([^|]+)\s*\|\s*([^|]+)\s*\|\s*([^|]+)\s*\|").unwrap();
        
        for cap in table_regex.captures_iter(matching_section) {
            // Skip the header row
            if cap.get(2).map_or(false, |m| m.as_str() == "Year") {
                continue;
            }
            
            let year = cap.get(2).map_or("", |m| m.as_str().trim());
            let make = cap.get(3).map_or("", |m| m.as_str().trim());
            let model = cap.get(4).map_or("", |m| m.as_str().trim());
            let row = cap.get(5).map_or("", |m| m.as_str().trim());
            let set_date = cap.get(6).map_or("", |m| m.as_str().trim());
            
            // Skip if essential fields are empty
            if year.is_empty() || make.is_empty() || model.is_empty() {
                continue;
            }
            
            // Extract location from the markdown (look for store name and address)
            let location = extract_location_from_markdown(matching_section);
            
            // Create unique ID
            let id = format!("{}_{}_{}_{}", 
                year, 
                make.to_lowercase().replace(' ', "_"), 
                model.to_lowercase().replace(' ', "_"),
                row.replace(' ', "_")
            );
            
            items.push(JunkyardItem {
                id,
                make: make.to_string(),
                model: model.to_string(),
                year: year.parse().ok(),
                location: Some(format!("Row {}, {}", row, location.unwrap_or("Unknown Location".to_string()))),
                availability: true,
                added_date: parse_set_date(set_date).unwrap_or_else(|| Utc::now()),
            });
        }
    }
    
    // If no table found, try alternative parsing method
    if items.is_empty() {
        items = parse_alternative_format(markdown, source_url);
    }
    
    items
}

fn extract_location_from_markdown(markdown: &str) -> Option<String> {
    // Look for store name pattern like "Pick-n-Pull - Newark"
    let store_regex = Regex::new(r"Pick-n-Pull - ([^]]+)").unwrap();
    if let Some(cap) = store_regex.captures(markdown) {
        return Some(cap.get(1)?.as_str().to_string());
    }
    
    // Look for address pattern
    let address_regex = Regex::new(r"(\d+\s+[^•]+)•\s*([^[]+)").unwrap();
    if let Some(cap) = address_regex.captures(markdown) {
        let address = cap.get(1)?.as_str().trim();
        let city_state = cap.get(2)?.as_str().trim();
        return Some(format!("{}, {}", address, city_state));
    }
    
    None
}

fn parse_set_date(date_str: &str) -> Option<DateTime<Utc>> {
    // Try to parse date in MM/DD/YYYY format
    if let Ok(date) = chrono::NaiveDate::parse_from_str(date_str, "%m/%d/%Y") {
        return Some(date.and_hms_opt(0, 0, 0)?.and_utc());
    }
    
    // Try other common formats
    if let Ok(date) = chrono::NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
        return Some(date.and_hms_opt(0, 0, 0)?.and_utc());
    }
    
    None
}

fn parse_alternative_format(markdown: &str, _source_url: &str) -> Vec<JunkyardItem> {
    let mut items = Vec::new();
    
    // Look for vehicle information in other patterns
    // Example: "2005 Subaru Impreza Wagon Row 132 Set: 04/02/2025"
    let vehicle_regex = Regex::new(r"(\d{4})\s+([A-Za-z]+)\s+([A-Za-z\s]+)\s+Row\s+(\d+)\s+Set:\s*([0-9/]+)").unwrap();
    
    for cap in vehicle_regex.captures_iter(markdown) {
        let year = cap.get(1).map_or("", |m| m.as_str());
        let make = cap.get(2).map_or("", |m| m.as_str());
        let model = cap.get(3).map_or("", |m| m.as_str().trim());
        let row = cap.get(4).map_or("", |m| m.as_str());
        let set_date = cap.get(5).map_or("", |m| m.as_str());
        
        let location = extract_location_from_markdown(markdown);
        
        let id = format!("{}_{}_{}_{}", 
            year, 
            make.to_lowercase(), 
            model.to_lowercase().replace(' ', "_"),
            row
        );
        
        items.push(JunkyardItem {
            id,
            make: make.to_string(),
            model: model.to_string(),
            year: year.parse().ok(),
            location: Some(format!("Row {}, {}", row, location.unwrap_or("Unknown Location".to_string()))),
            availability: true,
            added_date: parse_set_date(set_date).unwrap_or_else(|| Utc::now()),
        });
    }
    
    items
}