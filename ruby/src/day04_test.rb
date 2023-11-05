# File: test_elf_analysis.rb
require 'minitest/autorun'
require_relative 'day04'

class TestDay04 < Minitest::Test
  def worker_pairs
    [
      [[2, 4], [6, 8]],
      [[2, 3], [4, 5]],
      [[5, 7], [7, 9]],
      [[2, 8], [3, 7]],
      [[6, 6], [4, 6]],
      [[2, 6], [4, 8]],
    ]
  end

  def test_parse
    input = File.read('../input/day04_test.txt')
    result = parse(input)
    expected = worker_pairs

    assert_equal(expected, result)
  end

  def test_contained_count
    wp = worker_pairs
    result = contained_count(wp)

    assert_equal(2, result)
  end

  def test_overlap_count
    wp = worker_pairs
    result = overlap_count(wp)

    assert_equal(4, result)
  end
end
