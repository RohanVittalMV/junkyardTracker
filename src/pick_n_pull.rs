pub struct PicknPullSearch {
    base_url: String,
}

impl PicknPullSearch {
    pub fn new() -> Self {
        Self {
            base_url: "https://www.picknpull.com/check-inventory/vehicle-search".to_string(),
        }
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
}