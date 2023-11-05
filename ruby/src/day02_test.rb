require 'minitest/autorun'
require_relative 'day02'

class Day02Test < Minitest::Test
  def test_parse_chars
    input = File.read("../input/day02_test.txt")
    result = Day02.parse(input)
    assert_equal([['A','Y'],['B','X'],['C','Z']], result)
  end

  def test_part_one_round_compositions
    result = Day02.part_one_rounds([['A','Y'],['B','X'],['C','Z']])


    expected = [Round.new(Shape::PAPER, Shape::ROCK),
                Round.new(Shape::ROCK, Shape::PAPER),
                Round.new(Shape::SCISSORS, Shape::SCISSORS)]

    result.zip(expected).each do |result_round, expected_round|
      assert_equal(expected_round.player_shape, result_round.player_shape)
      assert_equal(expected_round.opponent_shape, result_round.opponent_shape)
    end
  end

  def test_part_one_player_score_calculates_total
    rounds = [
      Round.new(Shape::PAPER, Shape::ROCK),
      Round.new(Shape::ROCK, Shape::PAPER),
      Round.new(Shape::SCISSORS, Shape::SCISSORS)
    ]

    result = Day02.player_score(rounds)
    assert_equal(15, result)
  end

  def test_part_two_round_compositions
    result = Day02.part_two_rounds([['A','Y'],['B','X'],['C','Z']])
    expected = [
      Round.new(Shape::ROCK, Shape::ROCK),
      Round.new(Shape::ROCK, Shape::PAPER),
      Round.new(Shape::ROCK, Shape::SCISSORS)
    ]

    result.zip(expected).each do |result_round, expected_round|
      assert_equal(expected_round.player_shape, result_round.player_shape)
      assert_equal(expected_round.opponent_shape, result_round.opponent_shape)
    end
  end

  def test_part_two_player_score_calculates_total
    rounds = [
      Round.new(Shape::ROCK, Shape::ROCK),
      Round.new(Shape::ROCK, Shape::PAPER),
      Round.new(Shape::ROCK, Shape::SCISSORS)
    ]

    result = Day02.player_score(rounds)
    assert_equal(12, result)
  end
end