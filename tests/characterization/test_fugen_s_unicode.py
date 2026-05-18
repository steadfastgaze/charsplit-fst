"""Test Fugen-S handling with Unicode (umlaut) characters.

Regression tests for GitHub issue #1: Fugen-S byte-vs-char length bug
caused incorrect score for words like Önsbach where the pre_slice "öns"
(3 chars, 4 bytes) was incorrectly truncated because s.len() used byte
length instead of character count.
"""

import pytest


class TestFugenSUnicode:
    """Test Fugen-S behavior with multi-byte UTF-8 characters."""

    def test_önsbach_score_positive(self, splitter):
        """Önsbach should get a positive top-1 score (~0.5447), not negative."""
        result = splitter.split_compound("Önsbach")

        assert len(result) > 0
        assert result[0][1] == "Öns"
        assert result[0][2] == "Bach"
        assert result[0][0] > 0.4, f"Expected score ~0.5447, got {result[0][0]}"

    def test_önsbach_matches_reference(self, splitter):
        """Verify the split and score match the original Python CharSplit."""
        result = splitter.split_compound("Önsbach")

        # Reference: (+0.5447, 'Öns', 'Bach') from original CharSplit
        assert len(result) > 0
        assert result[0][1] == "Öns"
        assert result[0][2] == "Bach"
        assert abs(result[0][0] - 0.5447) < 0.01, (
            f"Score diverges from reference: got {result[0][0]:.4f}, expected ~0.5447"
        )

    def test_bäckerhandel_umlaut_split(self, splitter):
        """Bäckerhandel should split at Bäcker | Handel."""
        result = splitter.split_compound("Bäckerhandel")

        assert len(result) > 0
        assert result[0][1].startswith("Bäck")

    def test_fugen_s_with_umlaut_pre_slice(self, splitter):
        """Words where Fugen-S patterns appear after umlaut characters."""
        # "gülns" would test Fugen-S 'ns' after ü — but we test via real compounds
        result = splitter.split_compound("Önsbach")
        assert result[0][0] > 0, "Fugen-S should not truncate umlaut pre_slices incorrectly"

    def test_multi_hyphen_title_case(self, splitter):
        """Multi-hyphen words should have title case after each hyphen."""
        result = splitter.split_compound("Bundes-Autobahn-Kapitän")

        assert len(result) == 1
        assert result[0][1] == "Bundes-Autobahn"
        assert result[0][2] == "Kapitän"
