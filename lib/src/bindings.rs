//! Python bindings.

use crate::vault::{self, Vault as _Vault};
use cpython::{exc, py_class, py_fn, py_module_initializer, PyErr, PyNone, PyResult, Python};
use std::cell::RefCell;

py_module_initializer!(vault, |py, m| {
    m.add(
        py,
        "__doc__",
        "backend of the vault project written in pure Rust.",
    )?;
    m.add_class::<Vault>(py)?;
    m.add(py, "bytes_to_bits", py_fn!(py, bytes_to_bits(amount: u32)))?;
    m.add(py, "mb_to_bits", py_fn!(py, mb_to_bits(amount: u32)))?;
    Ok(())
});

py_class!(class Vault |py| {
    data instance: RefCell<_Vault>;
    def __new__(_cls, password: String, size: u32) -> PyResult<Self> {
        let instance = RefCell::new(
            match _Vault::new(password, size) {
                Ok(vault) => vault,
                Err(e) => return Err(PyErr::new::<exc::OSError, _>(py, e.to_string())),
            }
            );
        Self::create_instance(py, instance)
    }
    @classmethod def open(_cls, path: String) -> PyResult<Self> {
        Self::create_instance(py, RefCell::new(
                match _Vault::open(path) {
                    Ok(vault) => vault,
                    Err(e) => return Err(PyErr::new::<exc::IOError, _>(py, e.to_string())),
                }
                ))
    }
    def decrypt(&self, password: String) -> PyResult<String> {
        Ok(match self.instance(py).borrow().decrypt(password) {
            Ok(data) => data,
            Err(e) => return Err(PyErr::new::<exc::OSError, _>(py, e.to_string())),
        })
    }
    def encrypt(&self, password: String, data: String) -> PyResult<PyNone> {
        if let Err(e) = self.instance(py).borrow_mut().encrypt(password, data) {
            return Err(PyErr::new::<exc::OSError, _>(py, e.to_string()))
        }
        Ok(PyNone)
    }
    def encrypt_append(&self, password: String, data: String) -> PyResult<PyNone> {
        if let Err(e) = self.instance(py).borrow_mut().encrypt_append(password, data) {
            return Err(PyErr::new::<exc::OSError, _>(py, e.to_string()))
        }
        Ok(PyNone)
    }
    def save(&self, path: String) -> PyResult<PyNone> {
        if let Err(e) = self.instance(py).borrow().save(path) {
            return Err(PyErr::new::<exc::IOError, _>(py, e.to_string()))
        }
        Ok(PyNone)
    }
});

fn bytes_to_bits(_: Python, amount: u32) -> PyResult<u32> {
    Ok(vault::bytes_to_bits(amount))
}

fn mb_to_bits(_: Python, amount: u32) -> PyResult<u32> {
    Ok(vault::mb_to_bits(amount))
}
