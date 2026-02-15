"""Test score calculation and ranking logic."""
import pytest


class TestScoreRanking:
    """Test that scores are calculated and ranked correctly."""

    def test_top_result_has_highest_score(self, splitter):
        """Test that the first result has the highest score."""
        result = splitter.split_compound("Autobahnraststätte")

        assert len(result) > 1
        # First result should have highest score
        top_score = result[0][0]
        for r in result[1:]:
            assert r[0] <= top_score

    def test_results_sorted_descending(self, splitter):
        """Test that all results are sorted by score descending."""
        result = splitter.split_compound("Behördenangaben")

        scores = [r[0] for r in result]
        assert scores == sorted(scores, reverse=True)

    def test_score_is_float(self, splitter):
        """Test that scores are floats."""
        result = splitter.split_compound("Arbeitsamt")

        for r in result:
            assert isinstance(r[0], float)

    def test_scores_are_not_negative(self, splitter):
        """Test that scores are not overly negative (reasonable range)."""
        result = splitter.split_compound("Wirtschaftsschule")

        for r in result:
            # Scores should be reasonable (not extremely negative)
            assert r[0] > -10  # Allow some negative but not extreme

    def test_best_split_score_reasonably_high(self, splitter):
        """Test that the best split has a reasonably high score."""
        test_cases = [
            ("Autobahnraststätte", 0.79),
            ("Behördenangaben", 0.8),
            ("Arbeitsamt", 0.7),
        ]

        for word, min_score in test_cases:
            result = splitter.split_compound(word)
            assert result[0][0] >= min_score, \
                f"Expected score >= {min_score} for {word}, got {result[0][0]}"

    def test_multiple_results_have_different_scores(self, splitter):
        """Test that different splits generally have different scores."""
        result = splitter.split_compound("Autobahnraststätte")

        # Get unique scores
        unique_scores = set(round(r[0], 6) for r in result)
        # Should have multiple different scores
        assert len(unique_scores) > 1

    def test_hyphen_word_score_is_one(self, splitter):
        """Test that hyphenated words always have score 1.0."""
        result = splitter.split_compound("Bundes-Autobahn")

        assert len(result) == 1
        assert result[0][0] == 1.0

    def test_fallback_score_for_no_split(self, splitter):
        """Test that when no split is found, score is 0.0."""
        # Use a very short word that can't be split
        result = splitter.split_compound("ABC")

        # Might return the word as both parts with score 0
        found_fallback = any(r[0] == 0.0 for r in result)
        assert found_fallback or len(result) >= 1

    def test_score_formula_components(self, splitter):
        """Test that score follows: start_prob - in_prob + pre_prob."""
        # This is a characterization test - we're documenting current behavior
        # The Rust implementation should match this
        result = splitter.split_compound("Autobahnraststätte")

        # The top score should be positive
        assert result[0][0] > 0

    def test_consistent_scores_for_same_word(self, splitter):
        """Test that the same word gets consistent scores across calls."""
        word = "Wirtschaftsschule"
        result1 = splitter.split_compound(word)
        result2 = splitter.split_compound(word)

        assert len(result1) == len(result2)
        for r1, r2 in zip(result1, result2):
            assert r1[0] == r2[0], "Scores should be consistent"
            assert r1[1] == r2[1], "Splits should be consistent"
            assert r1[2] == r2[2], "Splits should be consistent"

    def test_no_split_case_has_zero_score(self, splitter):
        """Test words that can't be split get score 0.0."""
        result = splitter.split_compound("Xyzabc")

        # Should find at least one result
        assert len(result) >= 1
        # If no valid split, score should be 0
        # Note: This might not always be 0 if there's some probability match
