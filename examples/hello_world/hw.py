#!/usr/bin/env python

import logging
import hello_world


class ExtraInfoFormatter(logging.Formatter):
    def format(self, record):
        record_dict = record.__dict__.copy()
        standard_keys = logging.makeLogRecord({}).__dict__.keys()
        extras = {k: v for k, v in record_dict.items()
                  if k not in standard_keys}
        extras_string = f" {extras}" if extras else ""
        formatted_message = super().format(record)
        return f"{formatted_message}{extras_string}"


# Configure the root logger, so both the Python side ('hello_world') and the
# Rust side (prefixed with 'test_prefix', see src/lib.rs) get output.
root = logging.getLogger()
handler = logging.StreamHandler()
handler.setFormatter(ExtraInfoFormatter("%(name)s:%(levelname)s:%(message)s"))
root.addHandler(handler)
root.setLevel(logging.INFO)

logger = logging.getLogger('hello_world')

logger.info("Hello, world!", extra={"foo": 42, "bar": "baz"})

hello_world.log_hello()
