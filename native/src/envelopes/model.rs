pub struct Envelope {
    name: String,
    description: String,
    allocated_amount: f32,
    amount_spent: f32,
}

impl Envelope {
    pub fn new(name: String, description: String, allocated_amount: f32, amount_spent: f32) -> Self {
        Self {
            name,
            description,
            allocated_amount,
            amount_spent,
        }
    }
}
