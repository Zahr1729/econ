pub struct ValueBarConverter {
    min_value: u64,
    max_value: u64,
    min_bar: u32,
    max_bar: u32,
}

impl ValueBarConverter {
    pub fn new(min_value: u64, max_value: u64, min_bar: u32, max_bar: u32) -> Self {
        Self {
            min_value,
            max_value,
            min_bar,
            max_bar,
        }
    }

    pub fn value_window(&self) -> u64 {
        self.max_value - self.min_value
    }

    pub fn bar_window(&self) -> u32 {
        self.max_bar - self.min_bar
    }

    pub fn bar_to_value_scale(&self) -> u64 {
        return self.value_window() / self.bar_window() as u64;
    }
    pub fn to_bar(&self, value: u64) -> u32 {
        let zeroed_value = value - self.min_value;
        let zeroed_bar = (zeroed_value / self.bar_to_value_scale()) as u32;
        let bar = zeroed_bar + self.min_bar;

        bar
    }

    pub fn to_value(&self, bar: u32) -> u64 {
        let zeroed_bar = bar - self.min_bar;
        let zeroed_value = zeroed_bar as u64 * self.bar_to_value_scale();
        let value = zeroed_value + self.min_value;

        value
    }
}
