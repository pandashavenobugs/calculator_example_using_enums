pub enum CalculateType {
    Addition,
    Substraction,
    Multiplication,
    Division,
}
pub struct Calculator {
    pub field1: f32,
    pub field2: f32,
    pub calculate_type: Option<CalculateType>,
}

impl Calculator {
    pub fn new(field1: f32, field2: f32, calculate_type: &str) -> Result<Self, String> {
        let calculate: Option<CalculateType> = Calculator::find_calculate_type(calculate_type);
        match calculate {
            Some(calculate) => Ok(Calculator {
                field1,
                field2,
                calculate_type: Some(calculate),
            }),
            None => Err("calculate type is none".to_string()),
        }
    }

    fn add(&self) -> f32 {
        &self.field1 + &self.field2
    }
    fn sub(&self) -> f32 {
        &self.field1 - &self.field2
    }
    fn multiple(&self) -> f32 {
        &self.field1 * &self.field2
    }
    fn division(&self) -> f32 {
        &self.field1 / &self.field2
    }
    pub fn calculate(&self) -> f32 {
        match &self.calculate_type {
            Some(value) => match value {
                CalculateType::Addition => self.add(),
                CalculateType::Substraction => self.sub(),
                CalculateType::Multiplication => self.multiple(),
                CalculateType::Division => self.division(),
            },
            None => self.division(),
        }
    }

    fn find_calculate_type(data: &str) -> Option<CalculateType> {
        match data {
            "+" => Some(CalculateType::Addition),
            "-" => Some(CalculateType::Substraction),
            "/" => Some(CalculateType::Division),
            "*" => Some(CalculateType::Multiplication),
            _ => None,
        }
    }
    pub fn find_calculate_type_as_string(&self) -> &str {
        match self.calculate_type {
            Some(CalculateType::Addition) => "+",
            Some(CalculateType::Substraction) => "-",
            Some(CalculateType::Multiplication) => "*",
            Some(CalculateType::Division) => "/",
            None => "+",
        }
    }
}
