"""Pytest fixtures for characterization tests."""

import sys
from pathlib import Path

import pytest

# Add parent directory to path to import split_words
sys.path.insert(0, str(Path(__file__).parent.parent.parent))

from split_words import Splitter


@pytest.fixture
def splitter():
    """Provide a Splitter instance for testing."""
    return Splitter()


@pytest.fixture
def common_compounds():
    """Common German compound words for testing."""
    return [
        "Autobahnraststätte",
        "Behördenangaben",
        "Donaudampfschifffahrt",
        "Bundesregierung",
        "Arbeitsamt",
        "Hilfsbrunstein",
        "Wirtschaftsschule",
        "Tagschau",
    ]


@pytest.fixture
def fugen_s_words():
    """Words where Fugen-S patterns may be involved.

    Format: (word, expected_part1, expected_part2)
    Note: Not all of these have Fugen-S at the winning split position.
    """
    return [
        ("Arbeitsamt", "Arbeits", "Amt"),  # 'arbeits' ends with 'ts' (Fugen-S)
        (
            "Hilfsbrunstein",
            "Hilfsbrunst",
            "Ein",
        ),  # top split; no Fugen-S at this position
        ("Wirtschaftsschule", "Wirtschafts", "Schule"),  # 'wirtschafts' ends with 'ts'
        ("Tagschau", "Tag", "Schau"),  # 'tag' does not match any Fugen-S pattern
        (
            "Liebeslied",
            "Liebes",
            "Lied",
        ),  # 'liebes' ends with 'es', not a Fugen-S pattern
        ("Arbeitszimmer", "Arbeits", "Zimmer"),  # 'arbeits' ends with 'ts' (Fugen-S)
        (
            "Hilfskraft",
            "Hilfs",
            "Kraft",
        ),  # 'hilfs' ends with 'fs', not a Fugen-S pattern
    ]


@pytest.fixture
def hyphen_words():
    """Hyphenated compound words."""
    return [
        ("Donaudampfschifffahrt-Kapitän", "Donaudampfschifffahrt", "Kapitän"),
        ("Bundes-Autobahn", "Bundes", "Autobahn"),
        ("Arbeitsamt-Gebäude", "Arbeitsamt", "Gebäude"),
        ("Einkaufs-zentrum", "Einkaufs", "Zentrum"),
    ]


@pytest.fixture
def edge_case_words():
    """Edge cases: short words, unicode, case variations."""
    return [
        "Haus",  # 4 chars
        "Bad",  # 3 chars
        "AUTOBAHN",  # all caps
        "AuToBaHn",  # mixed case
        "Bäckerhandel",  # umlaut
        "Fußball",  # ß
        "Straße",  # ß
        "Übersee",  # umlaut
    ]


@pytest.fixture
def test_words_for_scores():
    """Words for score calculation verification."""
    return [
        "Autobahnraststätte",
        "Behördenangaben",
        "Arbeitsamt",
        "Wirtschaftsschule",
    ]
