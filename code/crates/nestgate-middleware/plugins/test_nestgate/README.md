# NestGate TrueNAS ZFS Integration Unit Tests

This directory contains unit tests for the NestGate ZFS Tier Management middleware plugins for TrueNAS.

## Test Structure

The test suite is organized by plugin module:

- `test_zfs_tier_manager.py`: Tests for the ZFS Tier Manager plugin
- `test_ai_workload_detector.py`: Tests for the AI Workload Detector plugin
- `test_telemetry_collector.py`: Tests for the Telemetry Collector plugin
- `run_tests.py`: Script to run all tests

## Running Tests

To run all tests:

```bash
python run_tests.py
```

### Command Line Options

The `run_tests.py` script supports several command-line options:

- `-f, --file`: Specify one or more test files to run
- `-t, --test`: Specify one or more test functions to run
- `-v, --verbose`: Enable verbose output
- `-c, --coverage`: Generate code coverage report

### Examples

Run all tests:
```bash
python run_tests.py
```

Run a specific test file:
```bash
python run_tests.py -f test_zfs_tier_manager.py
```

Run multiple test files:
```bash
python run_tests.py -f test_zfs_tier_manager.py test_ai_workload_detector.py
```

Run a specific test function:
```bash
python run_tests.py -t test_do_create
```

Generate coverage report:
```bash
python run_tests.py -c
```

Run tests with verbose output:
```bash
python run_tests.py -v
```

## Adding New Tests

To add new tests for existing plugins:

1. Find the appropriate test file (e.g., `test_zfs_tier_manager.py`)
2. Add a new test method to the existing test class
3. Use the `@pytest.mark.asyncio` decorator for async test methods
4. Follow the existing pattern for mocking and assertions

To add tests for a new plugin:

1. Create a new test file named `test_<plugin_name>.py`
2. Follow the pattern of existing test files
3. Add the new test file to the `TEST_FILES` list in `run_tests.py`

## Test Fixtures

Each test class includes a fixture that creates a mocked instance of the plugin service:

```python
@pytest.fixture
def tier_manager(self):
    """Create a ZFS Tier Manager instance with mocked middleware."""
    middleware = AsyncMock()
    middleware.call.side_effect = self._mock_middleware_call
    middleware.call_sync.side_effect = self._mock_middleware_call_sync
    
    manager = ZFSTierManagerService(middleware)
    manager._config = Mock()
    manager._config.datastore = 'nestgate.tiering'
    manager._config.datastore_prefix = 'tier_'
    
    # Mock the private methods
    manager._get_tier_properties = AsyncMock()
    manager._get_tier_properties.side_effect = self._mock_get_tier_properties
    
    return manager
```

## Mocking Framework

The tests use the Python `unittest.mock` framework to mock:

- The TrueNAS middleware
- External calls (like ZFS commands)
- Private methods
- Database queries and operations

Each test class includes helper methods to mock middleware calls:

```python
def _mock_middleware_call(self, service, *args, **kwargs):
    """Mock for middleware.call method."""
    if service == 'zfs.dataset.query':
        # Mock dataset query response
        return [{'id': 'test_dataset', 'properties': {}}]
    elif service == 'datastore.insert':
        # Mock database insert
        return 1
    return AsyncMock()
```

## Best Practices

When writing tests:

1. Test each public method separately
2. Mock any external dependencies
3. Verify both successful paths and error conditions
4. Check that middleware methods are called with correct arguments
5. Verify the structure and content of returned results
6. Use descriptive test names and docstrings
7. Follow the existing patterns for consistency

## Coverage Analysis

To analyze test coverage:

```bash
python run_tests.py -c
```

This will generate an HTML coverage report in the `htmlcov` directory. Open `htmlcov/index.html` in a browser to view the report. 