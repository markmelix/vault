//! Python bindings.

use crate::vault::Vault as _Vault;
use cpython::{py_class, py_module_initializer, PyNone, PyResult};
use std::cell::RefCell;

py_module_initializer!(vault, |py, m| {
    m.add(
        py,
        "__doc__",
        "backend of the vault project written in pure Rust.",
    )?;
    m.add_class::<Vault>(py)?;
    Ok(())
});

py_class!(class Vault |py| {
    data instance: RefCell<_Vault>;
    def __new__(_cls, password: String) -> PyResult<Self> {
        let instance = RefCell::new(
            match _Vault::new(password) {
                Ok(vault) => vault,
                Err(e) => panic!("{}", e),
            }
            );
        Self::create_instance(py, instance)
    }
    @classmethod def open(_cls, path: String) -> PyResult<Self> {
        Self::create_instance(py, RefCell::new(
                match _Vault::open(path) {
                    Ok(vault) => vault,
                    Err(e) => panic!("{}", e),
                }
                ))
    }
    def decrypt(&self, password: String) -> PyResult<String> {
        Ok(match self.instance(py).borrow().decrypt(password) {
            Ok(data) => data,
            Err(e) => panic!("{}", e),
        })
    }
    def encrypt(&self, password: String, data: String) -> PyResult<PyNone> {
        if let Err(e) = self.instance(py).borrow_mut().encrypt(password, data.as_bytes().to_vec()) {
            panic!("{}", e);
        }
        Ok(PyNone)
    }
    def save(&self, path: String) -> PyResult<PyNone> {
        if let Err(e) = self.instance(py).borrow().save(path) {
            panic!("{}", e);
        }
        Ok(PyNone)
    }
});
