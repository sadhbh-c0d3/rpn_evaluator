use rust_decimal::Decimal;

pub struct SimpleVM {
    stack: Vec<Decimal>,
}

impl SimpleVM {
    pub fn new() -> Self {
        Self { stack: Vec::new() }
    }

    pub fn push_operand(&mut self, value: Decimal) {
        self.stack.push(value);
    }

    pub fn sum(&mut self, depth: usize) -> Result<(), &'static str> {
        let offset = self.stack.len().checked_sub(depth).ok_or_else(|| "")?;
        let result = self.stack.drain(offset..).sum();
        self.stack.push(result);
        Ok(())
    }

    pub fn product(&mut self, depth: usize) -> Result<(), &'static str> {
        let offset = self.stack.len().checked_sub(depth).ok_or_else(|| "")?;
        let result = self.stack.drain(offset..).fold(Decimal::ONE, |a, x| a * x);
        self.stack.push(result);
        Ok(())
    }

    pub fn depth(&self) -> usize {
        self.stack.len()
    }

    pub fn outcome(self) -> Result<Decimal, &'static str> {
        if self.stack.len() != 1 {
            return Err("Expected stack length == 1");
        }
        Ok(self.stack[0])
    }
}
