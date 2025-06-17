#!/usr/bin/env python3

import os
import sys
import pytest
import argparse
import subprocess
from pathlib import Path

# Add parent directory to sys.path to allow imports from middleware plugins
current_dir = Path(__file__).parent
parent_dir = current_dir.parent
if str(parent_dir) not in sys.path:
    sys.path.insert(0, str(parent_dir))

# Define test files
TEST_FILES = [
    "test_zfs_tier_manager.py",
    "test_ai_workload_detector.py",
    "test_telemetry_collector.py"
]


def setup_environment():
    """Set up the test environment with necessary mocks."""
    # Create an environment variables dictionary to pass to pytest
    env = os.environ.copy()
    
    # Mock the TrueNAS middleware environment
    env["MIDDLEWARE_TEST"] = "1"
    
    # Return the environment
    return env


def run_tests(test_files=None, verbose=False, coverage=False):
    """Run the specified test files or all tests if none specified."""
    # Set up arguments for pytest
    args = ["-xvs"] if verbose else ["-xs"]
    
    # Add coverage if requested
    if coverage:
        args.extend(["--cov=middlewared.plugins.nestgate", "--cov-report=term", "--cov-report=html"])
    
    # If specific test files are provided, run only those
    if test_files:
        for test_file in test_files:
            test_path = os.path.join(current_dir, test_file)
            if os.path.exists(test_path):
                args.append(test_path)
            else:
                print(f"Test file not found: {test_path}")
                continue
    else:
        # Otherwise run all test files
        for test_file in TEST_FILES:
            test_path = os.path.join(current_dir, test_file)
            if os.path.exists(test_path):
                args.append(test_path)
    
    # Get the test environment
    env = setup_environment()
    
    # Run the tests
    print(f"Running tests with args: {args}")
    return pytest.main(args)


def run_individual_tests(test_names, verbose=False, coverage=False):
    """Run specific test functions across all test files."""
    # Set up arguments for pytest
    args = ["-xvs"] if verbose else ["-xs"]
    
    # Add coverage if requested
    if coverage:
        args.extend(["--cov=middlewared.plugins.nestgate", "--cov-report=term", "--cov-report=html"])
    
    # Add test files
    for test_file in TEST_FILES:
        test_path = os.path.join(current_dir, test_file)
        if os.path.exists(test_path):
            # For each test name, add a specific test selection option
            for test_name in test_names:
                # Find the class name in the test file
                with open(test_path, 'r') as f:
                    content = f.read()
                    class_line = next((line for line in content.split('\n') if "class Test" in line), None)
                    if class_line:
                        class_name = class_line.split("class ")[1].split("(")[0].strip()
                        args.append(f"{test_path}::{class_name}::{test_name}")
    
    # Get the test environment
    env = setup_environment()
    
    # Run the tests
    print(f"Running tests with args: {args}")
    return pytest.main(args)


def check_coverage():
    """Generate and display code coverage report."""
    # Set up arguments for pytest with coverage
    args = ["--cov=middlewared.plugins.nestgate", "--cov-report=term", "--cov-report=html"]
    
    # Add test files
    for test_file in TEST_FILES:
        test_path = os.path.join(current_dir, test_file)
        if os.path.exists(test_path):
            args.append(test_path)
    
    # Get the test environment
    env = setup_environment()
    
    # Run the tests with coverage
    print(f"Generating coverage report with args: {args}")
    return pytest.main(args)


def parse_arguments():
    """Parse command line arguments."""
    parser = argparse.ArgumentParser(description="Run NestGate plugin unit tests")
    parser.add_argument("-f", "--file", nargs="+", help="Specify test files to run")
    parser.add_argument("-t", "--test", nargs="+", help="Specify test functions to run")
    parser.add_argument("-v", "--verbose", action="store_true", help="Enable verbose output")
    parser.add_argument("-c", "--coverage", action="store_true", help="Generate code coverage report")
    return parser.parse_args()


if __name__ == "__main__":
    # Parse command line arguments
    args = parse_arguments()
    
    # Set up pytest arguments
    if args.test:
        # Run individual test functions
        return_code = run_individual_tests(args.test, args.verbose, args.coverage)
    elif args.file:
        # Run specific test files
        return_code = run_tests(args.file, args.verbose, args.coverage)
    elif args.coverage:
        # Generate coverage report
        return_code = check_coverage()
    else:
        # Run all tests
        return_code = run_tests(None, args.verbose, False)
    
    # Exit with the pytest return code
    sys.exit(return_code) 