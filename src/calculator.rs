// Copyright © 2020-2021 HQS Quantum Simulations GmbH. All Rights Reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License"); you may not use this file except
// in compliance with the License. You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software distributed under the
// License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either
// express or implied. See the License for the specific language governing permissions and
// limitations underthe License.

use crate::IntoCalculatorFloat;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use qoqo_calculator::Calculator;

#[pyclass(name = "Calculator", module = "qoqo_calculator_py03")]
pub struct CalculatorWrapper {
    r_calculator: Calculator,
}
#[pymethods]
impl CalculatorWrapper {
    /// Calculator class
    ///
    #[new]
    fn new() -> Self {
        let r_calculator = Calculator::new();
        CalculatorWrapper { r_calculator }
    }

    fn set(&mut self, variable_string: &str, val: f64) -> PyResult<()> {
        self.r_calculator.set_variable(variable_string, val);
        Ok(())
    }

    pub fn parse_str(&mut self, input: &str) -> PyResult<f64> {
        match self.r_calculator.parse_str(input) {
            Ok(x) => Ok(x),
            Err(x) => Err(PyValueError::new_err(format!(
                "{:?}; expression: {}",
                x, input
            ))),
        }
    }

    pub fn parse_get(&mut self, input: IntoCalculatorFloat) -> PyResult<f64> {
        let out = match input {
            IntoCalculatorFloat::CF(cfw) => self.r_calculator.parse_get(cfw.cf_internal),
            IntoCalculatorFloat::S(expression) => self.r_calculator.parse_str(&expression),
            IntoCalculatorFloat::F(fl) => Ok(fl),
            IntoCalculatorFloat::I(fl) => Ok(fl as f64),
        };
        match out {
            Ok(x) => Ok(x),
            Err(x) => Err(PyValueError::new_err(format!("{:?}", x))),
        }
    }
}

pub fn parse_str(expression: &str) -> PyResult<f64> {
    let mut calculator = Calculator::new();
    match calculator.parse_str(expression) {
        Ok(x) => Ok(x),
        Err(x) => Err(PyValueError::new_err(format!(
            "{:?}; expression {}",
            x, expression
        ))),
    }
}
