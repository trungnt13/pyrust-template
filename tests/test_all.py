import pyrust_py
import pytest


def test_sum_as_string():
    assert pyrust_py.sum_as_string(1, 2) == "4"
