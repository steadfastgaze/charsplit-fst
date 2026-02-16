"""Test basic German compound splits."""

import pytest


class TestBasicSplits:
    """Test common German compound word splits."""

    def test_autobahnraststätte_split(self, splitter):
        """Test Autobahnraststätte → Autobahn | Raststätte."""
        result = splitter.split_compound("Autobahnraststätte")

        assert len(result) > 0
        assert result[0][0] > 0.79  # Top score should be high
        assert "Autobahn" in result[0][1]
        assert "Raststätte" in result[0][2]

    def test_behördenangaben_split(self, splitter):
        """Test Behördenangaben → Behörden | Angaben."""
        result = splitter.split_compound("Behördenangaben")

        assert len(result) > 0
        assert result[0][0] > 0.8
        assert "Beh" in result[0][1]
        assert "ang" not in result[0][1].lower()
        assert "Ang" in result[0][2]
        assert "beh" not in result[0][2].lower()

    def test_donaudampfschifffahrt_split(self, splitter):
        """Test Donaudampfschifffahrt → Donau | Dampfschifffahrt."""
        result = splitter.split_compound("Donaudampfschifffahrt")

        assert len(result) > 0
        assert "Donau" in result[0][1]
        assert "Dampfschifffahrt" in result[0][2]

    def test_bundesregierung_split(self, splitter):
        """Test Bundesregierung → Bundes | Regierung."""
        result = splitter.split_compound("Bundesregierung")

        assert len(result) > 0
        assert "Bundes" in result[0][1]
        assert "Regierung" in result[0][2]

    def test_arbeitsamt_split(self, splitter):
        """Test Arbeitsamt → Arbeits | Amt."""
        result = splitter.split_compound("Arbeitsamt")

        assert len(result) > 0
        assert "Arbeits" in result[0][1]
        assert "Amt" in result[0][2]

    def test_hilfsbrunstein_split(self, splitter):
        """Test Hilfsbrunstein → Hilfsbrunst | Ein."""
        result = splitter.split_compound("Hilfsbrunstein")

        assert len(result) > 0
        assert result[0][1] == "Hilfsbrunst"
        assert result[0][2] == "Ein"

    def test_wirtschaftsschule_split(self, splitter):
        """Test Wirtschaftsschule → Wirtschafts | Schule."""
        result = splitter.split_compound("Wirtschaftsschule")

        assert len(result) > 0
        assert "Wirtschafts" in result[0][1]
        assert "Schule" in result[0][2]

    def test_tagschau_split(self, splitter):
        """Test Tagschau → Tag | Schau."""
        result = splitter.split_compound("Tagschau")

        assert len(result) > 0
        assert "Tag" in result[0][1]
        assert "Schau" in result[0][2]

    def test_liebeslied_split(self, splitter):
        """Test Liebeslied → Liebes | Lied."""
        result = splitter.split_compound("Liebeslied")

        assert len(result) > 0
        assert "Liebes" in result[0][1]
        assert "Lied" in result[0][2]

    def test_arbeitszimmer_split(self, splitter):
        """Test Arbeitszimmer → Arbeits | Zimmer."""
        result = splitter.split_compound("Arbeitszimmer")

        assert len(result) > 0
        assert "Arbeits" in result[0][1]
        assert "Zimmer" in result[0][2]

    def test_hilfskraft_split(self, splitter):
        """Test Hilfskraft → Hilfs | Kraft."""
        result = splitter.split_compound("Hilfskraft")

        assert len(result) > 0
        assert "Hilfs" in result[0][1]
        assert "Kraft" in result[0][2]

    def test_result_ordering(self, splitter):
        """Test that results are ordered by score descending."""
        result = splitter.split_compound("Autobahnraststätte")

        scores = [r[0] for r in result]
        assert scores == sorted(scores, reverse=True)

    def test_multiple_results_returned(self, splitter):
        """Test that multiple split possibilities are returned."""
        result = splitter.split_compound("Autobahnraststätte")

        assert len(result) > 5  # Should return many possible splits

    def test_all_results_have_three_parts(self, splitter):
        """Test that all results have (score, part1, part2) structure."""
        result = splitter.split_compound("Behördenangaben")

        for r in result:
            assert len(r) == 3
            assert isinstance(r[0], float)  # score
            assert isinstance(r[1], str)  # part1
            assert isinstance(r[2], str)  # part2

    def test_title_case_output(self, splitter):
        """Test that output parts are title-cased."""
        result = splitter.split_compound("autobahnraststätte")  # lowercase input

        for r in result:
            assert r[1][0].isupper()  # First character of part1 is uppercase
            assert r[2][0].isupper()  # First character of part2 is uppercase
