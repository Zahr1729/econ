#[derive(Debug)]
pub struct Economy {
    pub inflation: f64,
    pub interest: f64,
    pub money_supply: u64,
    pub debt: u64,
    pub taxes: u64,
    pub borrowing: u64,
    pub spending: u64,
    pub printing: u64,
}

impl Economy {
    pub fn compute_printing(&mut self) {
        self.printing = self.spending - self.borrowing - self.taxes;
    }

    pub fn progress_year(&mut self) {
        self.money_supply += self.printing;
        self.debt = (self.interest * self.debt as f64) as u64;
        self.debt += self.borrowing;
    }
}
