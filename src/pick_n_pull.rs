use std::collections::HashMap;

pub struct PicknPullSearch {
    base_url: String,
    make_map: HashMap<String, u32>,
    model_map: HashMap<(String, String), u32>, // (make, model) -> model_id
}

impl PicknPullSearch {
    pub fn new() -> Self {
        let mut make_map = HashMap::new();
        let mut model_map = HashMap::new();
        
        // Common makes (you can expand this as needed)
        make_map.insert("subaru".to_lowercase(), 226);
        make_map.insert("honda".to_lowercase(), 120);
        make_map.insert("toyota".to_lowercase(), 251);
        make_map.insert("nissan".to_lowercase(), 185);
        make_map.insert("ford".to_lowercase(), 95);
        make_map.insert("chevrolet".to_lowercase(), 58);
        make_map.insert("bmw".to_lowercase(), 34);
        make_map.insert("mercedes".to_lowercase(), 159);
        make_map.insert("audi".to_lowercase(), 20);
        make_map.insert("volkswagen".to_lowercase(), 273);
        
        // Common Subaru models
        model_map.insert(("subaru".to_string(), "impreza".to_string()), 4153);
        model_map.insert(("subaru".to_string(), "impreza wagon".to_string()), 4154);
        model_map.insert(("subaru".to_string(), "outback".to_string()), 4164);
        model_map.insert(("subaru".to_string(), "forester".to_string()), 4157);
        model_map.insert(("subaru".to_string(), "legacy".to_string()), 4160);
        model_map.insert(("subaru".to_string(), "wrx".to_string()), 4170);
        
        // Common Honda models
        model_map.insert(("honda".to_string(), "civic".to_string()), 2969);
        model_map.insert(("honda".to_string(), "accord".to_string()), 2960);
        model_map.insert(("honda".to_string(), "cr-v".to_string()), 2967);
        model_map.insert(("honda".to_string(), "pilot".to_string()), 2985);
        
        // Common Toyota models
        model_map.insert(("toyota".to_string(), "camry".to_string()), 6178);
        model_map.insert(("toyota".to_string(), "corolla".to_string()), 6182);
        model_map.insert(("toyota".to_string(), "prius".to_string()), 6209);
        model_map.insert(("toyota".to_string(), "rav4".to_string()), 6212);
        model_map.insert(("toyota".to_string(), "4runner".to_string()), 6165);
        
        Self {
            base_url: "https://www.picknpull.com/check-inventory/vehicle-search".to_string(),
            make_map,
            model_map,
        }
    }

    pub fn generate_search_url(&self, make: &str, model: &str, zip: &str, distance: u32, years: (u32, u32))
        -> Result<String, String> {
        let make_lower = make.to_lowercase();
        let model_lower = model.to_lowercase();
        
        let make_id = self.make_map.get(&make_lower)
            .ok_or_else(|| format!("Unsupported make: {}", make))?;
        
        let model_id = self.model_map.get(&(make_lower.clone(), model_lower))
            .ok_or_else(|| format!("Unsupported model: {} {}", make, model))?;
        
        Ok(format!(
            "{}?make={}&model={}&distance={}&zip={}&year={}-{}",
            self.base_url, make_id, model_id, distance, zip, years.0, years.1
        ))
    }

    pub fn generate_url(&self, make_id: u32, model_id: u32, zip: &str, distance: u32, years: (u32, u32))
        -> String {
        format!(
            "{}?make={}&model={}&distance={}&zip={}&year={}-{}",
            self.base_url, make_id, model_id, distance, zip, years.0, years.1
        )
    }

    // Subaru Impreza Wagon specific search TODO just for testing
    pub fn subaru_impreza_wagon_search(&self, zip: &str, distance: u32) -> String {
        // Make ID 226 = Subaru, Model ID 4154 = Impreza Wagon
        self.generate_url(226, 4154, zip, distance, (2000, 2006))
    }
    
    pub fn get_supported_makes(&self) -> Vec<String> {
        self.make_map.keys().cloned().collect()
    }
    
    pub fn get_supported_models_for_make(&self, make: &str) -> Vec<String> {
        let make_lower = make.to_lowercase();
        self.model_map.keys()
            .filter(|(m, _)| m == &make_lower)
            .map(|(_, model)| model.clone())
            .collect()
    }
}