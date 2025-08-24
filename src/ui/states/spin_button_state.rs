pub struct SpinButtonState {
    pub label_text: String,
    pub min_value: f64,
    pub max_value: f64,
    pub current_value: f64,
    pub increment_value: f64,
    pub page_increment_value: f64,
    pub page_size: f64,
    pub climb_rate: f64,
    pub digit_count: u32,
    pub use_integral_numbers: bool
}