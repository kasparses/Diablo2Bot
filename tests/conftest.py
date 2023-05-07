import pytest

@pytest.fixture(scope='module')
def functions():
    from run_test import functions
    return functions