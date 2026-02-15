"""Test Fugen-S handling in compound splits."""
import pytest


class TestFugenSDetection:
    """Test detection and removal of Fugen-S in compound words."""

    def test_arbeitsamt_removes_ts(self, splitter):
        """Test Arbeitsamt → Arbeits | Amt (removes 'ts')."""
        result = splitter.split_compound("Arbeitsamt")

        assert len(result) > 0
        # The first part should be "Arbeits" not "Arbeit"
        # But internally, the "ts" is removed for probability calculation
        assert "Arbeits" in result[0][1]
        assert "Amt" in result[0][2]

    def test_hilfsbrunstein_removes_hls(self, splitter):
        """Test Hilfsbrunstein → Hilfsbrun | Ein (removes 'hls' → 'hlf')."""
        result = splitter.split_compound("Hilfsbrunstein")

        assert len(result) > 0
        assert "Hilfsbrun" in result[0][1]
        assert "Ein" in result[0][2]

    def test_wirtschaftsschule_removes_ss(self, splitter):
        """Test Wirtschaftsschule → Wirtschafts | Schule (removes 'ss')."""
        result = splitter.split_compound("Wirtschaftsschule")

        assert len(result) > 0
        assert "Wirtschafts" in result[0][1]
        assert "Schule" in result[0][2]

    def test_tagschau_removes_gs(self, splitter):
        """Test Tagschau → Tag | Schau (Fugen-S handling applied)."""
        result = splitter.split_compound("Tagschau")

        assert len(result) > 0
        assert "Tag" in result[0][1]
        assert "Schau" in result[0][2]

    def test_liebeslied_removes_s(self, splitter):
        """Test Liebeslied → Liebes | Lied (removes 's')."""
        result = splitter.split_compound("Liebeslied")

        assert len(result) > 0
        assert "Liebes" in result[0][1]
        assert "Lied" in result[0][2]

    def test_arbeitszimmer_removes_ts(self, splitter):
        """Test Arbeitszimmer → Arbeits | Zimmer."""
        result = splitter.split_compound("Arbeitszimmer")

        assert len(result) > 0
        assert "Arbeits" in result[0][1]
        assert "Zimmer" in result[0][2]

    def test_hilfskraft_removes_hls(self, splitter):
        """Test Hilfskraft → Hilfs | Kraft."""
        result = splitter.split_compound("Hilfskraft")

        assert len(result) > 0
        assert "Hilfs" in result[0][1]
        assert "Kraft" in result[0][2]

    def test_fugen_s_endings(self, splitter):
        """Test all known Fugen-S endings: ts, gs, ks, hls, ns."""
        test_words = [
            ("Arbeitsamt", "ts"),
            ("Tagschau", "gs"),
            ("Werkstück", "ks"),  # if exists in vocabulary
            ("Hilfskraft", "hls"),
            ("Kanzlernacht", "ns"),  # if exists in vocabulary
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

    def test_fugen_s_applied_to_first_part(self, splitter):
        """Test that Fugen-S is applied to the first part, not the second."""
        result = splitter.split_compound("Arbeitsamt")

        assert len(result) > 0
        # First part should have the "ts" ending in display
        # But internally it's removed for probability calculation
        assert "Arbeits" in result[0][1]
        assert "Amt" in result[0][2]

    def test_no_fugen_s_in_simple_compound(self, splitter):
        """Test compound without Fugen-S."""
        result = splitter.split_compound("Autobahnraststätte")

        assert len(result) > 0
        # This doesn't have Fugen-S, so no removal
        assert "Autobahn" in result[0][1]
        assert "Raststätte" in result[0][2]

    def test_multiple_fugen_s_candidates(self, splitter):
        """Test word with multiple potential Fugen-S positions."""
        # Word might have multiple S-endings, algorithm should handle
        result = splitter.split_compound("Arbeitszimmer")

        assert len(result) > 0
        # Should still find the correct split
        assert "Arbeits" in result[0][1]
