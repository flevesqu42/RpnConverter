mod error;
mod business;

use std::collections::{HashMap, VecDeque};
use std::io::{Error, ErrorKind};
use business::InnerProcessVariables;

#[cfg(test)]
mod tests;

/* --- TYPES --- */

type ProcessingLambda<T> = fn(& RpnConverter<T>, & mut InnerProcessVariables<T>, T, & mut VecDeque<T>) -> Result<bool, Error>;

/* --- PUBLIC --- */

/**
    Modular Reverse Polish Notation converter.
*/
pub struct RpnConverter<T> {
    symbols_map: HashMap<T, ProcessingLambda<T>>
}

impl <T: std::cmp::Eq + std::hash::Hash> RpnConverter<T> {

    /**
        Construct a Reverse Polish Notation converter with given entry as open parenthesis, closing parenthesis, unary operands and binary operands respectively.
    */
    pub fn new(open_parenthesis: Vec<T>
               , closing_parenthesis: Vec<T>
               , unary_operands: Vec<T>
               , binary_operands: Vec<T>) -> RpnConverter<T>
    {
        let capacity = open_parenthesis.len() + closing_parenthesis.len() + unary_operands.len() + binary_operands.len();
        let mut symbols_map: HashMap<T, ProcessingLambda<T>> = HashMap::with_capacity(capacity);

        for open in open_parenthesis {
            assert!(!symbols_map.contains_key(& open));
            symbols_map.insert(open, RpnConverter::process_open_parenthesis);
        }
        for closing in closing_parenthesis {
            assert!(!symbols_map.contains_key(& closing));
            symbols_map.insert(closing, RpnConverter::process_closing_parenthesis);
        }
        for unary in unary_operands {
            assert!(!symbols_map.contains_key(& unary));
            symbols_map.insert(unary, RpnConverter::process_unary_operand);
        }
        for binary in binary_operands {
            assert!(!symbols_map.contains_key( & binary));
            symbols_map.insert(binary, RpnConverter::process_binary_operand);
        }

        RpnConverter { symbols_map }
    }

    /**
        Consume given standard notation and return populated reverse polish notation with parenthesis removal.
    */
    pub fn remove_parenthesis(& self, mut standard_notation : VecDeque<T>) -> Result<Vec<T>, Error> {
        self.process(& mut standard_notation, false)
    }
}

/* --- PRIVATE --- */

impl <T: std::cmp::Eq + std::hash::Hash> RpnConverter<T> {

    /**
        In case of success, return reverse polish notation.
    */
    fn process(& self, standard_notation: & mut VecDeque<T>, closing_parenthesis_expected: bool) -> Result<Vec<T>, Error> {
        let mut inner_process_variables: InnerProcessVariables<T> = InnerProcessVariables::new(closing_parenthesis_expected);

        self.loop_over_keywords(& mut inner_process_variables, standard_notation)?;

        RpnConverter::check_integrity_and_return_result(inner_process_variables)
    }

    /**
        Loop over keywords in standard_notation pointer, process matching function and populate final operator and operands for each keywords.
    */
    fn loop_over_keywords(& self, inner_process_variables : & mut InnerProcessVariables<T>, standard_notation: & mut VecDeque<T>) -> Result<(), Error> {

        // Loop over all keywords in standard notation
        loop {

            let operand = match standard_notation.pop_front() {
                // Pop standard notation head
                Some(op)    => op,
                // Break if there is no more keywords to read
                None            => return Ok(())
            };

            match self.symbols_map.get(& operand) {

                // If operand processing found
                Some(operand_process_function) => {

                    // If processing return false value, return to previous stack level
                    if !(operand_process_function(self, inner_process_variables, operand, standard_notation)?) {
                        return Ok(());
                    }
                },

                // Else treat operand as value
                None => RpnConverter::process_value(inner_process_variables, operand)?
            };

            RpnConverter::populate_operands(inner_process_variables)?;
        }
    }

    fn populate_operands(variables : & mut InnerProcessVariables<T>) -> Result<(), Error> {
        match (!variables.result().is_empty(), variables.processed_binary_operand().is_some(), !variables.processed_value().is_empty()) {

            // left side, binary operand and right side, process binary operand
            (true, true, true) => {
                // append value
                variables.inner_move_processed_value_to_result();
                // then append chained unary operands
                variables.inner_move_processed_unary_operands_to_result();
                // then append binary operand
                variables.inner_move_processed_binary_operand_to_result();
            },

            // No binary operand, right side, process optional unary operands
            (false, false, true) => {
                // append value
                variables.inner_move_processed_value_to_result();
                // then append chained unary operands
                variables.inner_move_processed_unary_operands_to_result();
            },

            // No binary operands between two values, return error
            (true, false, true) => {
                return Err(Error::new(ErrorKind::InvalidData, error::SUCCESSIVE_TWO_VALUES));
            },

            // Binary operand but no left side, return error
            (false, true, _) => {
                return Err(Error::new(ErrorKind::InvalidData, error::MISSING_LEFT_SIDE_VALUE_BINARY));
            },


            // operands but no right side yet, do nothing
            _ => ()
        }

        Ok(())
    }

    fn check_integrity_and_return_result(inner_process_variables : InnerProcessVariables<T>) -> Result<Vec<T>, Error> {

        // check operands integrity
        if inner_process_variables.result().is_empty() {
            return Err(Error::new(ErrorKind::InvalidData, error::EMPTY_RESULT));
        }

        // check parenthesis integrity
        if inner_process_variables.closing_parenthesis_expected() {
            return Err(Error::new(ErrorKind::InvalidData, error::MISSING_CLOSING_PARENTHESIS));
        }

        // check unary operands integrity
        if !inner_process_variables.processed_unary_operands().is_empty() {
            return Err(Error::new(ErrorKind::InvalidData, error::MISSING_RIGHT_SIDE_VALUE_UNARY));
        }

        // check binary operand integrity
        match inner_process_variables.processed_binary_operand() {
            Some(_binary_operand) => {
                return Err(Error::new(ErrorKind::InvalidData, error::MISSING_RIGHT_SIDE_VALUE_BINARY));
            },
            None => ()
        }

        // return result
        Ok(inner_process_variables.into_result())
    }
}

/**
    PROCESS'S LAMBDAS
*/
impl <T: std::cmp::Eq + std::hash::Hash> RpnConverter<T> {

    /**
        Function to process if open parenthesis found, setting next inner standard notation after closing parenthesis.
    */
    pub fn process_open_parenthesis(& self, variables : & mut InnerProcessVariables<T>, _head : T, standard_notation : & mut VecDeque<T>) -> Result<bool, Error> {
        let operands = self.process(standard_notation, true)?;

        variables.set_processed_values(operands);
        Ok(true)
    }

    /**
        Function to process if closing parenthesis found
    */
    pub fn process_closing_parenthesis(& self, variables : & mut InnerProcessVariables<T>, _head : T, _tail : & mut VecDeque<T>) -> Result<bool, Error> {
        if !variables.closing_parenthesis_expected() {
            return Err(Error::new(ErrorKind::InvalidData, error::UNEXPECTED_CLOSING_PARENTHESIS))
        }

        variables.set_closing_parenthesis_expected(false);
        Ok(false)
    }

    /**
        Function to process if binary operand found
    */
    pub fn process_binary_operand(& self, variables : & mut InnerProcessVariables<T>, operand : T, _tail : & mut VecDeque<T>) -> Result<bool, Error> {
        match variables.processed_binary_operand() {
            None => (),
            Some(_value) => return Err(Error::new(ErrorKind::InvalidData, error::SUCCESSIVE_TWO_BINARY_OPERANDS))
        }

        variables.set_processed_binary_operand(Some(operand));
        Ok(true)
    }

    /**
        Function to process if unary operand found
    */
    pub fn process_unary_operand(& self, variables : & mut InnerProcessVariables<T>, operand : T, _tail : & mut VecDeque<T>) -> Result<bool, Error> {
        variables.add_processed_unary_operands(operand);
        Ok(true)
    }

    /**
        Function to process if value found
    */
    pub fn process_value(variables : & mut InnerProcessVariables<T>, value : T) -> Result<(), Error> {
        if !variables.processed_value().is_empty() {
            return Err(Error::new(ErrorKind::InvalidData, error::SUCCESSIVE_TWO_VALUES));
        }

        variables.add_processed_value(value);
        Ok(())
    }
}
