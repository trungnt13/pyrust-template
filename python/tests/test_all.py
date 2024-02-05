import pyrust_template
import pytest


def test_sum_as_string():
    assert pyrust_template.sum_as_string(1, 1) == "2"
