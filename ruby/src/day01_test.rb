require 'minitest/autorun'
require_relative 'day01'

class Day01Test < Minitest::Test
  def test_parse_elf_sums
    result = Day01.parse(File.read("input/day01_test.txt"))
    assert_equal [6000, 4000, 11000, 24000, 10000], result
  end
  def test_part1
    assert_equal 24000, Day01.part1(File.read("input/day01_test.txt"))
  end

  def test_part2
    assert_equal 45000, Day01.part2(File.read("input/day01_test.txt"))
  end
end