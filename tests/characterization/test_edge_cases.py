"""Test edge cases: short words, unicode, case variations."""

import pytest


class TestShortWords:
    """Test handling of very short words."""

    def test_three_char_word(self, splitter):
        """Test word with 3 characters (below minimum split length)."""
        result = splitter.split_compound("Bad")

        # Too short to split, should return word as both parts
        assert len(result) >= 1
        assert result[0][1] == "Bad"
        assert result[0][2] == "Bad"

    def test_four_char_word(self, splitter):
        """Test word with 4 characters."""
        result = splitter.split_compound("Haus")

        # Barely minimum, might not split well
        assert len(result) >= 1

    def test_four_char_word_baum(self, splitter):
        """Test word with 4 characters (Baum)."""
        result = splitter.split_compound("Baum")

        assert len(result) >= 1

    def test_five_char_word(self, splitter):
        """Test word with 5 characters (Baume)."""
        result = splitter.split_compound("Baume")

        assert len(result) >= 1

    def test_single_char_word(self, splitter):
        """Test word with 1 character."""
        result = splitter.split_compound("A")

        assert len(result) >= 1
        assert result[0][1] == "A"
        assert result[0][2] == "A"

    def test_two_char_word(self, splitter):
        """Test word with 2 characters."""
        result = splitter.split_compound("AB")

        assert len(result) >= 1
        assert result[0][1] == "Ab"
        assert result[0][2] == "Ab"


class TestCaseVariations:
    """Test handling of different input cases."""

    def test_all_caps_input(self, splitter):
        """Test word in all caps."""
        result = splitter.split_compound("AUTOBAHN")

        assert len(result) >= 1
        # Output should still be title-cased
        assert result[0][1][0].isupper()
        assert result[0][2][0].isupper()

    def test_mixed_case_input(self, splitter):
        """Test word with mixed case."""
        result = splitter.split_compound("AuToBaHnRaStStÄTtE")

        assert len(result) >= 1
        # Output should be title-cased
        assert result[0][1][0].isupper()
        assert result[0][2][0].isupper()

    def test_lowercase_input(self, splitter):
        """Test word in lowercase."""
        result = splitter.split_compound("autobahnraststätte")

        assert len(result) >= 1
        # Output should be title-cased
        assert result[0][1] == "Autobahn" or result[0][1][0].isupper()
        assert result[0][2][0].isupper()


class TestUnicodeCharacters:
    """Test handling of German unicode characters."""

    def test_umlaut_a(self, splitter):
        """Test word with ä (a-umlaut)."""
        result = splitter.split_compound("Bäckerhandel")

        assert len(result) >= 1
        # Should handle umlaut correctly
        assert "Bäck" in result[0][1] or "Bä" in result[0][1]

    def test_umlaut_o(self, splitter):
        """Test word with ö (o-umlaut)."""
        result = splitter.split_compound("Behördenangaben")

        assert len(result) >= 1
        # Should handle ö correctly
        assert "Beh" in result[0][1]

    def test_umlaut_u(self, splitter):
        """Test word with ü (u-umlaut)."""
        result = splitter.split_compound("Übersee")

        assert len(result) >= 1
        assert "Über" in result[0][1] or "Üb" in result[0][1]

    def test_eszett(self, splitter):
        """Test word with ß (sharp s)."""
        result = splitter.split_compound("Fußball")

        assert len(result) >= 1
        assert "Fuß" in result[0][1] or "ß" in result[0][1] + result[0][2]

    def test_strasse_with_eszett(self, splitter):
        """Test Straße (street) with ß."""
        result = splitter.split_compound("Straße")

        assert len(result) >= 1

    def test_raststätte_with_umlaut(self, splitter):
        """Test Raststätte with ä."""
        result = splitter.split_compound("Raststätte")

        assert len(result) >= 1


class TestSpecialCases:
    """Test other special cases."""

    def test_empty_string(self, splitter):
        """Test empty string input."""
        result = splitter.split_compound("")

        # Should handle gracefully
        assert len(result) >= 1

    def test_repeated_characters(self, splitter):
        """Test word with repeated characters."""
        result = splitter.split_compound("Schiffahrt")  # double 'f'

        assert len(result) >= 1

    def test_word_with_numbers(self, splitter):
        """Test word containing numbers."""
        result = splitter.split_compound("A123")

        # Should handle without crashing
        assert len(result) >= 1

    def test_very_long_word(self, splitter):
        """Test very long compound word."""
        # Donaudampfschifffahrt is already quite long
        result = splitter.split_compound("Donaudampfschifffahrt")

        assert len(result) >= 1
        assert "Donau" in result[0][1]

    def test_no_valid_split(self, splitter):
        """Test word where no valid split is found."""
        # Very short word or unknown vocabulary
        result = splitter.split_compound("Xyz")

        assert len(result) >= 1
        # Should return word as both parts
        assert "Xyz" in result[0][1] or result[0][1] == "Xyz"
        assert result[0][2] == result[0][1] or result[0][0] == 0
