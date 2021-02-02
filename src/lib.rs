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

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
mod calculator_float;
pub use calculator_float::convert_into_calculator_float;
pub use calculator_float::CalculatorFloatWrapper;
pub use calculator_float::IntoCalculatorFloat;
mod calculator_complex;
pub use calculator_complex::CalculatorComplexWrapper;
pub use calculator_complex::IntoCalculatorComplex;
mod calculator;
pub use calculator::parse_str;
pub use calculator::CalculatorWrapper;

#[pyfunction]
fn parse_string(expression: &str) -> PyResult<f64> {
    parse_str(expression)
}

/// qoqo_calculator_py03 module bringing the qoqo_calculator rust library to python
///
/// rcalcultor is a rust library implementing CalculatorFloat, a type that can
/// contain a float or a symbolic math expression in string form.
///
/// Uses the pyo3 rust crate to create the python bindings.
///
#[pymodule]
fn qoqo_calculator_py03(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<CalculatorWrapper>()?;
    m.add_class::<CalculatorFloatWrapper>()?;
    m.add_class::<CalculatorComplexWrapper>()?;
    m.add_function(wrap_pyfunction!(parse_string, m)?).unwrap();
    Ok(())
}
