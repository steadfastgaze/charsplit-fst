# SPDX-License-Identifier: MIT OR Apache-2.0
"""Tests for Python bindings."""
import pytest


def test_import():
    """Test that the module can be imported."""
    try:
        from charsplit_fst import Splitter
    except ImportError:
        pytest.skip("charsplit_fst module not built yet - run 'maturin develop'")


def test_basic_split():
    """Test basic compound splitting."""
    from charsplit_fst import Splitter

    splitter = Splitter()
    results = splitter.split_compound("Autobahnraststätte")

    assert len(results) > 0
    assert results[0][0] > 0.79  # Score
    assert results[0][1] == "Autobahn"  # Part 1
    assert results[0][2] == "Raststätte"  # Part 2


def test_return_type():
    """Test that return type matches Python version."""
    from charsplit_fst import Splitter

    splitter = Splitter()
    results = splitter.split_compound("Haus")

    assert isinstance(results, list)
    if len(results) > 0:
        assert isinstance(results[0], tuple)
        assert len(results[0]) == 3
        assert isinstance(results[0][0], float)
        assert isinstance(results[0][1], str)
        assert isinstance(results[0][2], str)


def test_empty_word():
    """Test handling of very short words."""
    from charsplit_fst import Splitter

    splitter = Splitter()
    results = splitter.split_compound("Ab")  # Too short

    # Should return empty or fallback result
    assert isinstance(results, list)


def test_unicode():
    """Test Unicode character handling."""
    from charsplit_fst import Splitter

    splitter = Splitter()
    results = splitter.split_compound("Bäckerhandel")

    assert len(results) > 0
    # First result should contain valid Unicode parts
    assert "ä" in results[0][1] or "ä" in results[0][2] or len(results) > 1


def test_hyphen_handling():
    """Test hyphenated words."""
    from charsplit_fst import Splitter

    splitter = Splitter()
    results = splitter.split_compound("Bundes-Autobahn")

    assert len(results) == 1
    assert results[0][0] == 1.0  # Hyphen splits get score 1.0
    assert results[0][1] == "Bundes"
    assert results[0][2] == "Autobahn"


def test_multiple_results():
    """Test that multiple results are returned when available."""
    from charsplit_fst import Splitter

    splitter = Splitter()
    results = splitter.split_compound("Autobahnraststätte")

    # Should return multiple possible splits
    assert len(results) >= 2
    # Results should be sorted by score descending
    for i in range(len(results) - 1):
        assert results[i][0] >= results[i + 1][0]


def test_case_insensitive():
    """Test that input casing doesn't affect output."""
    from charsplit_fst import Splitter

    splitter = Splitter()

    results_lower = splitter.split_compound("autobahnraststätte")
    results_upper = splitter.split_compound("AUTOBAHNRASTSTÄTTE")
    results_mixed = splitter.split_compound("Autobahnraststätte")

    # All should return the same best split
    assert results_lower[0][1] == results_upper[0][1] == results_mixed[0][1]
    assert results_lower[0][2] == results_upper[0][2] == results_mixed[0][2]
