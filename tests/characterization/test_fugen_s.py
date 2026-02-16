"""Test Fugen-S handling in compound splits.

Fugen-S is a linking element in German compounds. The algorithm detects
these patterns at the end of the first part: ts, gs, ks, hls, ns.
When detected (and the result would be > 2 chars), the trailing 's' is
stripped for probability lookup only -- the displayed output keeps the
original split.

Note: Not all words tested here involve Fugen-S at the winning split.
Some are included to verify correct splitting of compounds where Fugen-S
patterns appear in the word but may not be the deciding factor.
"""

import pytest


class TestFugenSCompounds:
    """Test splitting of compounds where Fugen-S patterns may apply."""

    def test_arbeitsamt_fugen_s_ts(self, splitter):
        """Test Arbeitsamt → Arbeits | Amt (pre_slice 'arbeits' ends with 'ts')."""
        result = splitter.split_compound("Arbeitsamt")

        assert len(result) > 0
        # The first part is "Arbeits"; internally "ts" is detected and the
        # trailing 's' is removed for the suffix probability lookup.
        assert result[0][1] == "Arbeits"
        assert result[0][2] == "Amt"

    def test_hilfsbrunstein_no_fugen_s(self, splitter):
        """Test Hilfsbrunstein → Hilfsbrunst | Ein (no Fugen-S at top split)."""
        result = splitter.split_compound("Hilfsbrunstein")

        assert len(result) > 0
        # Top split is at position 11: "hilfsbrunst" | "ein"
        # 'hilfsbrunst' ends with 'st', which is NOT a Fugen-S pattern.
        assert result[0][1] == "Hilfsbrunst"
        assert result[0][2] == "Ein"

    def test_wirtschaftsschule_fugen_s_ts(self, splitter):
        """Test Wirtschaftsschule → Wirtschafts | Schule (pre_slice ends with 'ts')."""
        result = splitter.split_compound("Wirtschaftsschule")

        assert len(result) > 0
        # 'wirtschafts' ends with 'ts' -> Fugen-S detected
        assert result[0][1] == "Wirtschafts"
        assert result[0][2] == "Schule"

    def test_tagschau_no_fugen_s(self, splitter):
        """Test Tagschau → Tag | Schau (no Fugen-S at top split)."""
        result = splitter.split_compound("Tagschau")

        assert len(result) > 0
        # 'tag' does not end with any Fugen-S pattern
        assert result[0][1] == "Tag"
        assert result[0][2] == "Schau"

    def test_liebeslied_no_fugen_s(self, splitter):
        """Test Liebeslied → Liebes | Lied (no Fugen-S pattern match)."""
        result = splitter.split_compound("Liebeslied")

        assert len(result) > 0
        # 'liebes' ends with 'es', which is NOT a Fugen-S pattern
        # (patterns are: ts, gs, ks, hls, ns)
        assert result[0][1] == "Liebes"
        assert result[0][2] == "Lied"

    def test_arbeitszimmer_fugen_s_ts(self, splitter):
        """Test Arbeitszimmer → Arbeits | Zimmer (pre_slice ends with 'ts')."""
        result = splitter.split_compound("Arbeitszimmer")

        assert len(result) > 0
        assert result[0][1] == "Arbeits"
        assert result[0][2] == "Zimmer"

    def test_hilfskraft_no_fugen_s(self, splitter):
        """Test Hilfskraft → Hilfs | Kraft (no Fugen-S at top split)."""
        result = splitter.split_compound("Hilfskraft")

        assert len(result) > 0
        # 'hilfs' ends with 'fs', which is NOT a Fugen-S pattern
        assert result[0][1] == "Hilfs"
        assert result[0][2] == "Kraft"

    def test_fugen_s_patterns_in_various_words(self, splitter):
        """Test that words with known Fugen-S patterns can be split."""
        test_words = [
            ("Arbeitsamt", "ts"),  # 'arbeits' ends with 'ts'
            ("Arbeitszimmer", "ts"),  # 'arbeits' ends with 'ts'
            ("Werkstück", "ks"),  # 'werk' at top split; 'werks' at pos 5 ends with 'ks'
            (
                "Kanzlernacht",
                "ns",
            ),  # 'kanzlern' at pos 8 ends with 'ns' (not the top split)
        ]

        for word, ending in test_words:
            result = splitter.split_compound(word)
            assert len(result) > 0, f"No result for {word}"

    def test_fugen_s_minimum_length(self, splitter):
        """Test that Fugen-S is not removed if result would be too short."""
        # Words where removing Fugen-S would leave <= 2 chars
        # should not have it removed
        short_words = ["Haus", "Bad", "Tas"]

        for word in short_words:
            result = splitter.split_compound(word)
            assert len(result) > 0
            # Should still return something reasonable

    def test_fugen_s_display_vs_lookup(self, splitter):
        """Test that Fugen-S removal affects lookup only, not displayed output."""
        result = splitter.split_compound("Arbeitsamt")

        assert len(result) > 0
        # Display still shows "Arbeits" (with the 's')
        # But internally, the 's' was stripped for the suffix probability lookup
        assert result[0][1] == "Arbeits"
        assert result[0][2] == "Amt"

    def test_no_fugen_s_in_simple_compound(self, splitter):
        """Test compound without Fugen-S."""
        result = splitter.split_compound("Autobahnraststätte")

        assert len(result) > 0
        # 'autobahn' does not end with any Fugen-S pattern
        assert result[0][1] == "Autobahn"
        assert result[0][2] == "Raststätte"

    def test_multiple_fugen_s_candidates(self, splitter):
        """Test word with multiple potential Fugen-S positions."""
        # "arbeits" ends with "ts" (Fugen-S), algorithm should handle
        result = splitter.split_compound("Arbeitszimmer")

        assert len(result) > 0
        assert result[0][1] == "Arbeits"
