#!/bin/bash

# TinyWrite Test Runner
# Runs all test files in the examples directory

set -e

echo "=========================================="
echo "   TinyWrite Test Suite Runner"
echo "=========================================="
echo ""

# Color codes
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Count tests
total_tests=0
passed_tests=0

# List of tests to run
tests=(
    "char_byte"
    "arithmetic"
    "same_operation_support"
    "comparisons"
    "logic"
    "variables"
    "expressions"
    "vectors"
    "edge_cases"
    "advanced"
    "integration"
)

# Run each test
for test in "${tests[@]}"; do
    total_tests=$((total_tests + 1))
    test_file="${test}.txt"

    echo -e "${BLUE}▶ Running: ${test_file}${NC}"

    if cargo run --quiet "$test_file" 2>/dev/null; then
        echo -e "${GREEN}✓ PASSED${NC}"
        passed_tests=$((passed_tests + 1))
    else
        echo "✗ FAILED"
    fi
    echo ""
done

# Summary
echo "=========================================="
echo "   Test Summary"
echo "=========================================="
echo "Total tests: $total_tests"
echo -e "${GREEN}Passed: $passed_tests${NC}"
echo "Failed: $((total_tests - passed_tests))"
echo ""

if [ $passed_tests -eq $total_tests ]; then
    echo -e "${GREEN}All tests passed! ✓${NC}"
    exit 0
else
    echo "Some tests failed."
    exit 1
fi
