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

use num_complex::Complex;
use pyo3::class::basic::CompareOp;
use pyo3::exceptions::{PyNotImplementedError, PyTypeError, PyValueError, PyZeroDivisionError};
use pyo3::prelude::*;
use pyo3::{PyNumberProtocol, PyObjectProtocol};
use qoqo_calculator::{CalculatorError, CalculatorFloat};
use std::collections::HashMap;
use std::convert::From;
use std::panic::catch_unwind;
#[derive(FromPyObject, Clone, Debug)]
pub enum IntoCalculatorFloat {
    CF(CalculatorFloatWrapper),
    F(f64),
    S(String),
    I(i32),
}

impl IntoCalculatorFloat {
    pub fn cast_to_calculator_float(&self) -> CalculatorFloat {
        match self {
            IntoCalculatorFloat::CF(cfx) => cfx.cf_internal.clone(),
            IntoCalculatorFloat::F(x) => CalculatorFloat::from(x),
            IntoCalculatorFloat::S(y) => CalculatorFloat::from(y),
            IntoCalculatorFloat::I(y) => CalculatorFloat::from(y),
        }
    }
}

pub fn convert_into_calculator_float(input: &PyAny) -> Result<CalculatorFloat, CalculatorError> {
    let try_f64_conversion = input.call_method0("__float__");
    match try_f64_conversion {
        Ok(x) => Ok(CalculatorFloat::from(
            f64::extract(x).map_err(|_| CalculatorError::NotConvertable)?,
        )),
        _ => {
            let try_str_conversion = input.call_method0("__str__");
            match try_str_conversion {
                Ok(x) => Ok(CalculatorFloat::from(
                    String::extract(x).map_err(|_| CalculatorError::NotConvertable)?,
                )),
                _ => Err(CalculatorError::NotConvertable),
            }
        }
    }
}

#[pyclass(name = "CalculatorFloat", module = "qoqo_calculator_pyo3")]
#[derive(Clone, Debug)]
pub struct CalculatorFloatWrapper {
    pub cf_internal: CalculatorFloat,
}

#[pymethods]
impl CalculatorFloatWrapper {
    #[new]
    fn new(input: IntoCalculatorFloat) -> Self {
        match input {
            IntoCalculatorFloat::CF(cfx) => cfx,
            IntoCalculatorFloat::F(x) => CalculatorFloatWrapper {
                cf_internal: CalculatorFloat::from(x),
            },
            IntoCalculatorFloat::S(y) => CalculatorFloatWrapper {
                cf_internal: CalculatorFloat::from(y),
            },
            IntoCalculatorFloat::I(y) => CalculatorFloatWrapper {
                cf_internal: CalculatorFloat::from(y),
            },
        }
    }

    fn __copy__(&self) -> PyResult<CalculatorFloatWrapper> {
        Ok(self.clone())
    }

    fn __deepcopy__(&self, _memodict: Py<PyAny>) -> PyResult<CalculatorFloatWrapper> {
        Ok(self.clone())
    }

    fn __getnewargs_ex__(&self) -> PyResult<((PyObject,), HashMap<String, String>)> {
        let gil = pyo3::Python::acquire_gil();
        let py = gil.python();
        let object = match self.cf_internal {
            CalculatorFloat::Float(ref x) => x.to_object(py),
            CalculatorFloat::Str(ref x) => x.to_object(py),
        };
        Ok(((object,), HashMap::new()))
    }

    #[getter]
    fn is_float(&self) -> PyResult<bool> {
        Ok(self.cf_internal.is_float())
    }

    fn sqrt(&self) -> PyResult<CalculatorFloatWrapper> {
        Ok(CalculatorFloatWrapper {
            cf_internal: self.cf_internal.sqrt(),
        })
    }

    fn atan2(&self, other: IntoCalculatorFloat) -> PyResult<CalculatorFloatWrapper> {
        let other_cf = other.cast_to_calculator_float();
        Ok(CalculatorFloatWrapper {
            cf_internal: self.cf_internal.atan2(other_cf),
        })
    }

    fn isclose(&self, other: IntoCalculatorFloat) -> PyResult<bool> {
        let other_cf = other.cast_to_calculator_float();
        Ok(self.cf_internal.isclose(other_cf))
    }

    fn exp(&self) -> PyResult<CalculatorFloatWrapper> {
        Ok(CalculatorFloatWrapper {
            cf_internal: self.cf_internal.exp(),
        })
    }

    fn sin(&self) -> PyResult<CalculatorFloatWrapper> {
        Ok(CalculatorFloatWrapper {
            cf_internal: self.cf_internal.sin(),
        })
    }

    fn cos(&self) -> PyResult<CalculatorFloatWrapper> {
        Ok(CalculatorFloatWrapper {
            cf_internal: self.cf_internal.cos(),
        })
    }

    fn acos(&self) -> PyResult<CalculatorFloatWrapper> {
        Ok(CalculatorFloatWrapper {
            cf_internal: self.cf_internal.acos(),
        })
    }

    fn abs(&self) -> PyResult<CalculatorFloatWrapper> {
        Ok(CalculatorFloatWrapper {
            cf_internal: self.cf_internal.abs(),
        })
    }

    fn signum(&self) -> PyResult<CalculatorFloatWrapper> {
        Ok(CalculatorFloatWrapper {
            cf_internal: self.cf_internal.signum(),
        })
    }

    fn sign(&self) -> PyResult<CalculatorFloatWrapper> {
        Ok(CalculatorFloatWrapper {
            cf_internal: self.cf_internal.signum(),
        })
    }
    #[getter]
    fn value(&self) -> PyResult<PyObject> {
        let gil = pyo3::Python::acquire_gil();
        let py = gil.python();
        match self.cf_internal {
            CalculatorFloat::Float(ref x) => Ok(x.to_object(py)),
            CalculatorFloat::Str(ref x) => Ok(x.to_object(py)),
        }
    }
}

#[pyproto]
impl PyObjectProtocol for CalculatorFloatWrapper {
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{}", self.cf_internal))
    }

    fn __format__(&self, _format_spec: &str) -> PyResult<String> {
        Ok(format!("{}", self.cf_internal))
    }

    fn __richcmp__(&self, other: IntoCalculatorFloat, op: CompareOp) -> PyResult<bool> {
        let other_cf = other.cast_to_calculator_float();
        match op {
            CompareOp::Eq => Ok(self.cf_internal == other_cf),
            CompareOp::Ne => Ok(self.cf_internal != other_cf),
            _ => Err(PyNotImplementedError::new_err(
                "Other comparison not implemented.",
            )),
        }
    }
}

#[pyproto]
impl PyNumberProtocol for CalculatorFloatWrapper {
    fn __add__(
        lhs: IntoCalculatorFloat,
        rhs: IntoCalculatorFloat,
    ) -> PyResult<CalculatorFloatWrapper> {
        let other_cf = rhs.cast_to_calculator_float();
        let self_cf = lhs.cast_to_calculator_float();
        Ok(CalculatorFloatWrapper {
            cf_internal: (self_cf + other_cf),
        })
    }
    fn __iadd__(&'p mut self, other: IntoCalculatorFloat) -> PyResult<()> {
        let other_cf = other.cast_to_calculator_float();
        self.cf_internal += other_cf;
        Ok(())
    }

    fn __sub__(
        lhs: IntoCalculatorFloat,
        rhs: IntoCalculatorFloat,
    ) -> PyResult<CalculatorFloatWrapper> {
        let other_cf = rhs.cast_to_calculator_float();
        let self_cf = lhs.cast_to_calculator_float();
        Ok(CalculatorFloatWrapper {
            cf_internal: (self_cf - other_cf),
        })
    }
    fn __isub__(&'p mut self, other: IntoCalculatorFloat) -> PyResult<()> {
        let other_cf = other.cast_to_calculator_float();
        self.cf_internal -= other_cf;
        Ok(())
    }

    fn __mul__(lhs: Py<PyAny>, rhs: Py<PyAny>) -> PyResult<CalculatorFloatWrapper> {
        let gil = pyo3::Python::acquire_gil();
        let py = gil.python();
        let lhs_ref = lhs.as_ref(py);
        let rhs_ref = rhs.as_ref(py);
        let self_cf = convert_into_calculator_float(lhs_ref).map_err(|_| {
            PyTypeError::new_err("Left hand side can not be converted to Calculator Float")
        })?;
        let other_cf = convert_into_calculator_float(rhs_ref).map_err(|_| {
            PyTypeError::new_err("Right hand side can not be converted to Calculator Float")
        })?;
        Ok(CalculatorFloatWrapper {
            cf_internal: (self_cf * other_cf),
        })
    }
    fn __imul__(&'p mut self, other: IntoCalculatorFloat) -> PyResult<()> {
        let other_cf = other.cast_to_calculator_float();
        self.cf_internal *= other_cf;
        Ok(())
    }

    fn __pow__(
        lhs: CalculatorFloatWrapper,
        rhs: IntoCalculatorFloat,
        modulo: Option<CalculatorFloatWrapper>,
    ) -> PyResult<CalculatorFloatWrapper> {
        if let Some(_x) = modulo {
            return Err(PyNotImplementedError::new_err("Modulo is not implemented"));
        }
        let other_cf = rhs.cast_to_calculator_float();
        let self_cf = lhs.cf_internal;
        Ok(CalculatorFloatWrapper {
            cf_internal: (self_cf.powf(other_cf)),
        })
    }

    fn __truediv__(
        lhs: IntoCalculatorFloat,
        rhs: IntoCalculatorFloat,
    ) -> PyResult<CalculatorFloatWrapper> {
        let other_cf = rhs.cast_to_calculator_float();
        let self_cf = lhs.cast_to_calculator_float();
        let res = catch_unwind(|| self_cf / other_cf);
        match res {
            Ok(x) => Ok(CalculatorFloatWrapper { cf_internal: x }),
            Err(_) => Err(PyZeroDivisionError::new_err("Division by zero!")),
        }
    }
    fn __itruediv__(&'p mut self, other: IntoCalculatorFloat) -> PyResult<()> {
        let other_cf = other.cast_to_calculator_float();
        if let CalculatorFloat::Float(x) = other_cf {
            if x == 0.0 {
                return Err(PyZeroDivisionError::new_err("Division by zero!"));
            }
        }
        self.cf_internal /= other_cf;
        Ok(())
    }

    fn __neg__(&'p self) -> PyResult<CalculatorFloatWrapper> {
        Ok(CalculatorFloatWrapper {
            cf_internal: -self.cf_internal.clone(),
        })
    }
    fn __abs__(&'p self) -> PyResult<CalculatorFloatWrapper> {
        Ok(CalculatorFloatWrapper {
            cf_internal: self.cf_internal.abs(),
        })
    }
    fn __invert__(&'p self) -> PyResult<CalculatorFloatWrapper> {
        Ok(CalculatorFloatWrapper {
            cf_internal: self.cf_internal.recip(),
        })
    }

    fn __float__(&'p self) -> PyResult<f64> {
        match self.cf_internal {
            CalculatorFloat::Float(x) => Ok(x),
            CalculatorFloat::Str(_) => Err(PyValueError::new_err(
                "Symbolic Value can not be cast to float.",
            )),
        }
    }
    fn __complex__(&'p self) -> PyResult<Complex<f64>> {
        match self.cf_internal {
            CalculatorFloat::Float(x) => Ok(Complex::new(x, 0.0)),
            CalculatorFloat::Str(_) => Err(PyValueError::new_err(
                "Symbolic Value can not be cast to complex.",
            )),
        }
    }
}
