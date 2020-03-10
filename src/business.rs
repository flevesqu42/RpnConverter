
/* --- BUSINESS --- */

pub struct InnerProcessVariables<T> {
    result: Vec<T>,
    processed_value: Vec<T>,
    processed_binary_operand: Option<T>,
    processed_unary_operands: Vec<T>,
    closing_parenthesis_expected: bool
}

impl <T> InnerProcessVariables<T> {

    pub fn new(closing_parenthesis_expected : bool) -> InnerProcessVariables<T> {
        let operands : Vec<T> = Default::default();
        let processed_value: Vec<T> = Default::default();
        let processed_binary_operand: Option<T> = None;
        let processed_unary_operands: Vec<T> = Default::default();

        InnerProcessVariables { result: operands, processed_value, processed_binary_operand, processed_unary_operands, closing_parenthesis_expected }
    }

    pub fn into_result(self) -> Vec<T> {
        self.result
    }
}

/**
    Setters
*/
impl <T> InnerProcessVariables<T> {

    pub fn set_processed_values(& mut self, processed_values: Vec<T>) {
        self.processed_value = processed_values;
    }

    pub fn add_processed_value(& mut self, processed_value: T) {
        self.processed_value.push(processed_value);
    }

    pub fn add_processed_unary_operands(& mut self, processed_unary_operand: T) {
        self.processed_unary_operands.push(processed_unary_operand);
    }

    pub fn set_processed_binary_operand(& mut self, processed_binary_operand: Option<T>) {
        self.processed_binary_operand = processed_binary_operand;
    }

    pub fn set_closing_parenthesis_expected(& mut self, closing_parenthesis_expected : bool) {
        self.closing_parenthesis_expected = closing_parenthesis_expected;
    }
}

/**
    Immutable getters
*/
impl <T> InnerProcessVariables<T> {
    pub fn result(& self) -> & Vec<T> {
        & self.result
    }

    pub fn processed_value(& self) -> & Vec<T> {
        & self.processed_value
    }

    pub fn processed_binary_operand(& self) -> & Option<T> {
        & self.processed_binary_operand
    }

    pub fn processed_unary_operands(& self) -> & Vec<T> {
        & self.processed_unary_operands
    }

    pub fn closing_parenthesis_expected(& self) -> bool {
        self.closing_parenthesis_expected
    }
}

/**
    Inner movers
*/
impl <T> InnerProcessVariables<T> {
    pub fn inner_move_processed_value_to_result(& mut self) {
        self.result.append(& mut self.processed_value);
    }

    pub fn inner_move_processed_binary_operand_to_result(& mut self) {
        self.result.push(self.processed_binary_operand.take().unwrap());
    }

    pub fn inner_move_processed_unary_operands_to_result(& mut self) {
        self.result.append(& mut self.processed_unary_operands);
    }
}

