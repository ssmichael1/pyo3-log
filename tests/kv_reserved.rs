#![cfg(feature = "kv")]
//! Key-value pairs whose keys collide with `LogRecord` attributes must not drop the message.

use std::ffi::CString;

use pyo3::prelude::*;
use pyo3::types::PyDict;

#[test]
fn kv_reserved_key() {
    Python::attach(|py| {
        let globals = PyDict::new(py);
        let code = CString::new(
            r#"
import logging

records = []

class Collect(logging.Handler):
    def emit(self, record):
        records.append(record)

logging.getLogger().addHandler(Collect())
logging.getLogger().setLevel(logging.DEBUG)
"#,
        )
        .unwrap();
        py.run(&code, Some(&globals), None).unwrap();

        pyo3_log::Logger::new(py, pyo3_log::Caching::Nothing)
            .unwrap()
            .install()
            .unwrap();

        log::info!(name = "x", message = "y", custom = 1; "with reserved keys");
        assert!(PyErr::take(py).is_none(), "logging left an exception set");

        let records = globals.get_item("records").unwrap().unwrap();
        assert_eq!(records.len().unwrap(), 1);
        let record = records.get_item(0).unwrap();
        let get = |attr: &str| -> String { record.getattr(attr).unwrap().extract().unwrap() };
        assert_eq!(get("msg"), "with reserved keys");
        assert_eq!(get("name"), "kv_reserved");
        assert_eq!(get("name_"), "x");
        assert_eq!(get("message_"), "y");
        assert_eq!(get("custom"), "1");
    });
}
