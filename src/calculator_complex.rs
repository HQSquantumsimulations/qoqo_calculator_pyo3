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

use crate::{CalculatorFloatWrapper, convert_into_calculator_float};
use num_complex::Complex;
use pyo3::class::basic::CompareOp;
use pyo3::exceptions::{PyNotImplementedError, PyTypeError, PyValueError, PyZeroDivisionError};
use pyo3::prelude::*;
use pyo3::ToPyObject;
use pyo3::{PyNumberProtocol, PyObjectProtocol};
use qoqo_calculator::{CalculatorError, CalculatorFloat, CalculatorComplex};
use std::collections::HashMap;
use std::convert::TryInto;
use std::panic::catch_unwind;

pub fn convert_into_calculator_complex(input: &PyAny) -> Result<CalculatorComplex, CalculatorError> {
    let try_real_part = input.getattr("real");
    match try_real_part {
        Ok(x) => {
            let real_part_converted = convert_into_calculator_float(x)?;
            let try_imag_part = input.getattr("imag");
            match try_imag_part {
                Ok(y) => {
                    let imag_part_converted = convert_into_calculator_float(y)?;
                    Ok(CalculatorComplex::new(real_part_converted, imag_part_converted))
                }
                _ => Err(CalculatorError::NotConvertable),
            }
        }
        _ => {
            let str_converted = convert_into_calculator_float(input)?;
            Ok(CalculatorComplex::new(str_converted, 0.0))
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
    fn new(input: &PyAny) -> PyResult<Self> {
        let converted = convert_into_calculator_complex(input).map_err(|_| {
            PyTypeError::new_err("Input can not be converted to Calculator Complex")
        })?;
        Ok(CalculatorComplexWrapper {
                cc_internal: converted,
        })
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

    fn __setstate__(&mut self, state: (Py<PyAny>, Py<PyAny>)) -> PyResult<()> {
        *self = CalculatorComplexWrapper::from_pair(state.0, state.1)?;
        Ok(())
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
    fn from_pair(re: Py<PyAny>, im: Py<PyAny>) -> PyResult<CalculatorComplexWrapper> {
        let gil = pyo3::Python::acquire_gil();
        let py = gil.python();
        let re_ref = re.as_ref(py);
        let imag_ref = im.as_ref(py);
        let re_cf = convert_into_calculator_float(re_ref).map_err(|_| {
            PyTypeError::new_err("Real input can not be converted to Calculator Complex")
        })?;
        let im_cf = convert_into_calculator_float(imag_ref).map_err(|_| {
            PyTypeError::new_err("Imag input can not be converted to Calculator Complex")
        })?;
        Ok(CalculatorComplexWrapper {
            cc_internal: CalculatorComplex::new(re_cf, im_cf),
        })
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

    fn isclose(&self, other: Py<PyAny>) -> PyResult<bool> {
        let gil = pyo3::Python::acquire_gil();
        let py = gil.python();
        let other_ref = other.as_ref(py);
        let other_cc = convert_into_calculator_complex(other_ref).map_err(|_| {
            PyTypeError::new_err("Right hand side can not be converted to Calculator Complex")
        })?;
        Ok(self.cc_internal.isclose(other_cc))
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

    fn __richcmp__(&self, other: Py<PyAny>, op: CompareOp) -> PyResult<bool> {
        let gil = pyo3::Python::acquire_gil();
        let py = gil.python();
        let other_ref = other.as_ref(py);
        let other_cc = convert_into_calculator_complex(other_ref).map_err(|_| {
            PyTypeError::new_err("Right hand side can not be converted to Calculator Complex")
        })?;
        match op {
            CompareOp::Eq => Ok(self.cc_internal == other_cc),
            CompareOp::Ne => Ok(self.cc_internal != other_cc),
            _ => Err(PyNotImplementedError::new_err(
                "Other comparison not implemented.",
            )),
        }
    }
}

#[pyproto]
impl PyNumberProtocol for CalculatorComplexWrapper {
    fn __add__(
        lhs: Py<PyAny>,
        rhs: Py<PyAny>,
    ) -> PyResult<CalculatorComplexWrapper> {
        let gil = pyo3::Python::acquire_gil();
        let py = gil.python();
        let lhs_ref = lhs.as_ref(py);
        let rhs_ref = rhs.as_ref(py);
        let self_cc = convert_into_calculator_complex(lhs_ref).map_err(|_| {
            PyTypeError::new_err("Left hand side can not be converted to Calculator Complex")
        })?;
        let other_cc = convert_into_calculator_complex(rhs_ref).map_err(|_| {
            PyTypeError::new_err("Right hand side can not be converted to Calculator Complex")
        })?;
        Ok(CalculatorComplexWrapper {
            cc_internal: (self_cc + other_cc),
        })
    }
    fn __iadd__(&'p mut self, other: Py<PyAny>) -> PyResult<()> {
        let gil = pyo3::Python::acquire_gil();
        let py = gil.python();
        let other_ref = other.as_ref(py);
        let other_cc = convert_into_calculator_complex(other_ref).map_err(|_| {
            PyTypeError::new_err("Right hand side can not be converted to Calculator Complex")
        })?;
        self.cc_internal += other_cc;
        Ok(())
    }

    fn __sub__(
        lhs: Py<PyAny>,
        rhs: Py<PyAny>,
    ) -> PyResult<CalculatorComplexWrapper> {
        let gil = pyo3::Python::acquire_gil();
        let py = gil.python();
        let lhs_ref = lhs.as_ref(py);
        let rhs_ref = rhs.as_ref(py);
        let self_cc = convert_into_calculator_complex(lhs_ref).map_err(|_| {
            PyTypeError::new_err("Left hand side can not be converted to Calculator Complex")
        })?;
        let other_cc = convert_into_calculator_complex(rhs_ref).map_err(|_| {
            PyTypeError::new_err("Right hand side can not be converted to Calculator Complex")
        })?;
        Ok(CalculatorComplexWrapper {
            cc_internal: (self_cc - other_cc),
        })
    }
    fn __isub__(&'p mut self, other: Py<PyAny>) -> PyResult<()> {
        let gil = pyo3::Python::acquire_gil();
        let py = gil.python();
        let other_ref = other.as_ref(py);
        let other_cc = convert_into_calculator_complex(other_ref).map_err(|_| {
            PyTypeError::new_err("Right hand side can not be converted to Calculator Complex")
        })?;
        self.cc_internal -= other_cc;
        Ok(())
    }

    fn __mul__(
        lhs: Py<PyAny>,
        rhs: Py<PyAny>,
    ) -> PyResult<CalculatorComplexWrapper> {
        let gil = pyo3::Python::acquire_gil();
        let py = gil.python();
        let lhs_ref = lhs.as_ref(py);
        let rhs_ref = rhs.as_ref(py);
        let self_cc = convert_into_calculator_complex(lhs_ref).map_err(|_| {
            PyTypeError::new_err("Left hand side can not be converted to Calculator Complex")
        })?;
        let other_cc = convert_into_calculator_complex(rhs_ref).map_err(|_| {
            PyTypeError::new_err("Right hand side can not be converted to Calculator Complex")
        })?;
        Ok(CalculatorComplexWrapper {
            cc_internal: (self_cc * other_cc),
        })
    }
    fn __imul__(&'p mut self, other: Py<PyAny>,) -> PyResult<()> {
        let gil = pyo3::Python::acquire_gil();
        let py = gil.python();
        let other_ref = other.as_ref(py);
        let other_cc = convert_into_calculator_complex(other_ref).map_err(|_| {
            PyTypeError::new_err("Right hand side can not be converted to Calculator Complex")
        })?;
        self.cc_internal *= other_cc;
        Ok(())
    }

    fn __truediv__(
        lhs: Py<PyAny>,
        rhs: Py<PyAny>,
    ) -> PyResult<CalculatorComplexWrapper> {
        let gil = pyo3::Python::acquire_gil();
        let py = gil.python();
        let lhs_ref = lhs.as_ref(py);
        let rhs_ref = rhs.as_ref(py);
        let self_cc = convert_into_calculator_complex(lhs_ref).map_err(|_| {
            PyTypeError::new_err("Left hand side can not be converted to Calculator Complex")
        })?;
        let other_cc = convert_into_calculator_complex(rhs_ref).map_err(|_| {
            PyTypeError::new_err("Right hand side can not be converted to Calculator Complex")
        })?;
        let res = catch_unwind(|| self_cc / other_cc);
        match res {
            Ok(x) => Ok(CalculatorComplexWrapper { cc_internal: x }),
            Err(_) => Err(PyZeroDivisionError::new_err("Division by zero!")),
        }
    }
    fn __itruediv__(&'p mut self, other: Py<PyAny>) -> PyResult<()> {
        let gil = pyo3::Python::acquire_gil();
        let py = gil.python();
        let other_ref = other.as_ref(py);
        let other_cc = convert_into_calculator_complex(other_ref).map_err(|_| {
            PyTypeError::new_err("Right hand side can not be converted to Calculator Complex")
        })?;
        if let CalculatorFloat::Float(x) = other_cc.norm() {
            if x == 0.0 {
                return Err(PyZeroDivisionError::new_err("Division by zero!"));
            }
        }
        self.cc_internal /= other_cc;
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
