"""Test hyphen handling in compound splits."""
import pytest


class TestHyphenHandling:
    """Test splitting of hyphenated compound words."""

    def test_single_hyphen_at_last_position(self, splitter):
        """Test Donaudampfschifffahrt-Kapitän splits at hyphen."""
        result = splitter.split_compound("Donaudampfschifffahrt-Kapitän")

        assert len(result) == 1  # Only one result for hyphenated words
        assert result[0][0] == 1.0  # Score is always 1.0
        assert result[0][1] == "Donaudampfschifffahrt"
        assert result[0][2] == "Kapitän"

    def test_single_hyphen_simple(self, splitter):
        """Test Bundes-Autobahn splits at hyphen."""
        result = splitter.split_compound("Bundes-Autobahn")

        assert len(result) == 1
        assert result[0][0] == 1.0
        assert result[0][1] == "Bundes"
        assert result[0][2] == "Autobahn"

    def test_hyphen_with_fugen_s(self, splitter):
        """Test Arbeitsamt-Gebäude splits at hyphen, preserves Fugen-S."""
        result = splitter.split_compound("Arbeitsamt-Gebäude")

        assert len(result) == 1
        assert result[0][0] == 1.0
        assert result[0][1] == "Arbeitsamt"
        assert result[0][2] == "Gebäude"

    def test_hyphen_in_middle(self, splitter):
        """Test Einkaufs-zentrum splits at hyphen."""
        result = splitter.split_compound("Einkaufs-zentrum")

        assert len(result) == 1
        assert result[0][0] == 1.0
        assert result[0][1] == "Einkaufs"
        assert result[0][2] == "Zentrum"

    def test_multiple_hyphens_splits_at_last(self, splitter):
        """Test that multiple hyphens split at the last one."""
        result = splitter.split_compound("Bundes-Autobahn-Kapitän")

        assert len(result) == 1
        # Should split at the last hyphen
        assert result[0][1] == "Bundes-Autobahn"
        assert result[0][2] == "Kapitän"

    def test_hyphen_at_start(self, splitter):
        """Test hyphen at start of word."""
        result = splitter.split_compound("-Autobahn")

        assert len(result) == 1
        # Empty first part before first hyphen
        assert result[0][1] == ""
        assert result[0][2] == "Autobahn"

    def test_hyphen_at_end(self, splitter):
        """Test hyphen at end of word."""
        result = splitter.split_compound("Autobahn-")

        assert len(result) == 1
        assert result[0][1] == "Autobahn"
        # Empty second part after last hyphen
        assert result[0][2] == ""

    def test_only_hyphen(self, splitter):
        """Test word that is only a hyphen."""
        result = splitter.split_compound("-")

        assert len(result) == 1
        assert result[0][1] == ""
        assert result[0][2] == ""

    def test_hyphen_case_preserved_in_output(self, splitter):
        """Test that output is title-cased regardless of input case."""
        result = splitter.split_compound("bundes-autobahn")  # lowercase

        assert len(result) == 1
        assert result[0][1] == "Bundes"
        assert result[0][2] == "Autobahn"

    def test_hyphen_with_unicode(self, splitter):
        """Test hyphen with unicode characters."""
        result = splitter.split_compound("Bäckerei-Konditorei")

        assert len(result) == 1
        assert result[0][1] == "Bäckerei"
        assert result[0][2] == "Konditorei"
