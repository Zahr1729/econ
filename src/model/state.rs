#[derive(Debug)]
pub struct State {
    pub inflation: f64,
    pub interest: f64,
    pub money_supply: u64,
    pub debt: u64,
    pub taxes: u64,
    pub borrowing: u64,
    pub spending: u64,
    pub printing: u64,
}

impl State {
    pub fn revenue(&self) -> u64 {
        self.printing + self.taxes
    }

    pub fn expenses(&self) -> u64 {
        self.spending
    }

    pub fn profit(&self) -> i64 {
        self.revenue() as i64 - self.expenses() as i64
    }

    pub fn surplus(&self) -> u64 {
        0.max(self.profit()) as u64
    }

    pub fn deficit(&self) -> u64 {
        0.max(-self.profit()) as u64
    }

    pub fn adjust_borrowing(&mut self) {
        // Borrowing has to match deficit.
        self.borrowing = self.deficit();
    }

    pub fn progress_year(&mut self) {
        self.money_supply += self.printing;
        self.debt = ((1.0 + self.interest) * self.debt as f64) as u64;
        self.debt += self.borrowing;
    }
}
