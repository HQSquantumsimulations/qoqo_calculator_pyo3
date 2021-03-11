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

use crate::{CalculatorFloatWrapper, IntoCalculatorFloat};
use num_complex::Complex;
use pyo3::class::basic::CompareOp;
use pyo3::exceptions::{PyNotImplementedError, PyValueError, PyZeroDivisionError};
use pyo3::prelude::*;
use pyo3::ToPyObject;
use pyo3::{PyNumberProtocol, PyObjectProtocol};
use qoqo_calculator::CalculatorError;
use qoqo_calculator::{CalculatorComplex, CalculatorFloat};
use std::collections::HashMap;
use std::convert::TryInto;
use std::panic::catch_unwind;

#[derive(FromPyObject, Clone, Debug)]
pub enum IntoCalculatorComplex {
    CC(CalculatorComplexWrapper),
    CF(CalculatorFloatWrapper),
    C(Complex<f64>),
    F(f64),
    S(String),
    I(i32),
}

impl IntoCalculatorComplex {
    pub fn cast_to_calculator_complex(&self) -> CalculatorComplex {
        match self {
            IntoCalculatorComplex::CC(ccx) => ccx.cc_internal.clone(),
            IntoCalculatorComplex::CF(cfx) => CalculatorComplex::from(&cfx.cf_internal),
            IntoCalculatorComplex::C(x) => CalculatorComplex::from(*x),
            IntoCalculatorComplex::F(x) => CalculatorComplex::from(x),
            IntoCalculatorComplex::S(y) => CalculatorComplex::from(y),
            IntoCalculatorComplex::I(y) => CalculatorComplex::from(y),
        }
    }
}

#[pyclass(name = "CalculatorComplex", module = "qoqo_calculator_pyo3")]
#[derive(Clone, Debug)]
pub struct CalculatorComplexWrapper {
    pub cc_internal: CalculatorComplex,
}

#[pymethods]
impl CalculatorComplexWrapper {
    #[new]
    fn new(input: IntoCalculatorComplex) -> Self {
        match input {
            IntoCalculatorComplex::CC(ccx) => ccx,
            IntoCalculatorComplex::CF(cfx) => CalculatorComplexWrapper {
                cc_internal: CalculatorComplex::from(cfx.cf_internal),
            },
            IntoCalculatorComplex::C(x) => CalculatorComplexWrapper {
                cc_internal: CalculatorComplex::from(x),
            },
            IntoCalculatorComplex::F(x) => CalculatorComplexWrapper {
                cc_internal: CalculatorComplex::from(x),
            },
            IntoCalculatorComplex::S(y) => CalculatorComplexWrapper {
                cc_internal: CalculatorComplex::from(y),
            },
            IntoCalculatorComplex::I(y) => CalculatorComplexWrapper {
                cc_internal: CalculatorComplex::from(y),
            },
        }
    }

    fn __copy__(&self) -> CalculatorComplexWrapper {
        self.clone()
    }

    fn __deepcopy__(&self, _memodict: Py<PyAny>) -> CalculatorComplexWrapper {
        self.clone()
    }

    fn __getnewargs_ex__(&self) -> ((PyObject,), HashMap<String, String>) {
        let gil = pyo3::Python::acquire_gil();
        let py = gil.python();
        let x = 0.0;
        let object = x.to_object(py);
        ((object,), HashMap::new())
    }

    fn __getstate__(&self) -> (PyObject, PyObject) {
        let gil = pyo3::Python::acquire_gil();
        let py = gil.python();
        let object_real = match self.cc_internal.re {
            CalculatorFloat::Float(ref x) => x.to_object(py),
            CalculatorFloat::Str(ref x) => x.to_object(py),
        };
        let object_imag = match self.cc_internal.im {
            CalculatorFloat::Float(ref x) => x.to_object(py),
            CalculatorFloat::Str(ref x) => x.to_object(py),
        };
        (object_real, object_imag)
    }

    fn __setstate__(&mut self, state: (IntoCalculatorFloat, IntoCalculatorFloat)) {
        *self = CalculatorComplexWrapper::from_pair(state.0, state.1);
    }

    fn to_dict(&self) -> HashMap<String, PyObject> {
        let mut dict = HashMap::new();
        let gil = pyo3::Python::acquire_gil();
        let py = gil.python();
        dict.insert("is_calculator_complex".to_string(), true.to_object(py));
        match &self.cc_internal.re {
            CalculatorFloat::Float(x) => {
                dict.insert("real".to_string(), x.to_object(py));
            }
            CalculatorFloat::Str(x) => {
                dict.insert("real".to_string(), x.to_object(py));
            }
        }
        match &self.cc_internal.im {
            CalculatorFloat::Float(x) => {
                dict.insert("imag".to_string(), x.to_object(py));
            }
            CalculatorFloat::Str(x) => {
                dict.insert("imag".to_string(), x.to_object(py));
            }
        }
        dict
    }

    #[getter]
    fn real(&self) -> CalculatorFloatWrapper {
        CalculatorFloatWrapper {
            cf_internal: self.cc_internal.re.clone(),
        }
    }

    #[getter]
    fn imag(&self) -> CalculatorFloatWrapper {
        CalculatorFloatWrapper {
            cf_internal: self.cc_internal.im.clone(),
        }
    }

    #[staticmethod]
    fn from_pair(re: IntoCalculatorFloat, im: IntoCalculatorFloat) -> CalculatorComplexWrapper {
        let re_cf = re.cast_to_calculator_float();
        let im_cf = im.cast_to_calculator_float();
        Self {
            cc_internal: CalculatorComplex::new(re_cf, im_cf),
        }
    }

    fn conj(&self) -> CalculatorComplexWrapper {
        Self {
            cc_internal: self.cc_internal.conj(),
        }
    }

    fn arg(&self) -> CalculatorFloatWrapper {
        CalculatorFloatWrapper {
            cf_internal: self.cc_internal.arg(),
        }
    }

    fn isclose(&self, other: IntoCalculatorComplex) -> bool {
        let other_cc = other.cast_to_calculator_complex();
        self.cc_internal.isclose(other_cc)
    }

    fn abs(&self) -> CalculatorFloatWrapper {
        CalculatorFloatWrapper {
            cf_internal: self.cc_internal.norm(),
        }
    }
}

#[pyproto]
impl PyObjectProtocol for CalculatorComplexWrapper {
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{}", self.cc_internal))
    }

    fn __format__(&self, _format_spec: &str) -> PyResult<String> {
        Ok(format!("{}", self.cc_internal))
    }

    fn __richcmp__(&self, other: IntoCalculatorComplex, op: CompareOp) -> PyResult<bool> {
        let other_cf = other.cast_to_calculator_complex();
        match op {
            CompareOp::Eq => Ok(self.cc_internal == other_cf),
            CompareOp::Ne => Ok(self.cc_internal != other_cf),
            _ => Err(PyNotImplementedError::new_err(
                "Other comparison not implemented.",
            )),
        }
    }
}

#[pyproto]
impl PyNumberProtocol for CalculatorComplexWrapper {
    fn __add__(
        lhs: IntoCalculatorComplex,
        rhs: IntoCalculatorComplex,
    ) -> PyResult<CalculatorComplexWrapper> {
        let other_cf = rhs.cast_to_calculator_complex();
        let self_cf = lhs.cast_to_calculator_complex();
        Ok(CalculatorComplexWrapper {
            cc_internal: (self_cf + other_cf),
        })
    }
    fn __iadd__(&'p mut self, other: IntoCalculatorComplex) -> PyResult<()> {
        let other_cf = other.cast_to_calculator_complex();
        self.cc_internal += other_cf;
        Ok(())
    }

    fn __sub__(
        lhs: IntoCalculatorComplex,
        rhs: IntoCalculatorComplex,
    ) -> PyResult<CalculatorComplexWrapper> {
        let other_cf = rhs.cast_to_calculator_complex();
        let self_cf = lhs.cast_to_calculator_complex();
        Ok(CalculatorComplexWrapper {
            cc_internal: (self_cf - other_cf),
        })
    }
    fn __isub__(&'p mut self, other: IntoCalculatorComplex) -> PyResult<()> {
        let other_cf = other.cast_to_calculator_complex();
        self.cc_internal -= other_cf;
        Ok(())
    }

    fn __mul__(
        lhs: IntoCalculatorComplex,
        rhs: IntoCalculatorComplex,
    ) -> PyResult<CalculatorComplexWrapper> {
        let other_cf = rhs.cast_to_calculator_complex();
        let self_cf = lhs.cast_to_calculator_complex();
        Ok(CalculatorComplexWrapper {
            cc_internal: (self_cf * other_cf),
        })
    }
    fn __imul__(&'p mut self, other: IntoCalculatorComplex) -> PyResult<()> {
        let other_cf = other.cast_to_calculator_complex();
        self.cc_internal *= other_cf;
        Ok(())
    }

    fn __truediv__(
        lhs: IntoCalculatorComplex,
        rhs: IntoCalculatorComplex,
    ) -> PyResult<CalculatorComplexWrapper> {
        let other_cf = rhs.cast_to_calculator_complex();
        let self_cf = lhs.cast_to_calculator_complex();
        let res = catch_unwind(|| self_cf / other_cf);
        match res {
            Ok(x) => Ok(CalculatorComplexWrapper { cc_internal: x }),
            Err(_) => Err(PyZeroDivisionError::new_err("Division by zero!")),
        }
    }
    fn __itruediv__(&'p mut self, other: IntoCalculatorComplex) -> PyResult<()> {
        let other_cf = other.cast_to_calculator_complex();
        if let CalculatorFloat::Float(x) = other_cf.norm() {
            if x == 0.0 {
                return Err(PyZeroDivisionError::new_err("Division by zero!"));
            }
        }
        self.cc_internal /= other_cf;
        Ok(())
    }

    fn __neg__(&'p self) -> PyResult<CalculatorComplexWrapper> {
        Ok(CalculatorComplexWrapper {
            cc_internal: -self.cc_internal.clone(),
        })
    }
    fn __abs__(&'p self) -> PyResult<CalculatorFloatWrapper> {
        Ok(CalculatorFloatWrapper {
            cf_internal: self.cc_internal.norm(),
        })
    }
    fn __invert__(&'p self) -> PyResult<CalculatorComplexWrapper> {
        Ok(CalculatorComplexWrapper {
            cc_internal: self.cc_internal.recip(),
        })
    }

    fn __float__(&'p self) -> PyResult<f64> {
        let fl: Result<f64, CalculatorError> =
            CalculatorComplex::try_into(self.cc_internal.clone());
        match fl {
            Ok(x) => Ok(x),
            Err(x) => Err(PyValueError::new_err(format!("{:?}", x))),
        }
    }

    fn __complex__(&'p self) -> PyResult<Complex<f64>> {
        let com: Result<Complex<f64>, CalculatorError> =
            CalculatorComplex::try_into(self.cc_internal.clone());
        match com {
            Ok(x) => Ok(x),
            Err(x) => Err(PyValueError::new_err(format!("{:?}", x))),
        }
    }
}
