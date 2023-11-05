require 'minitest/autorun'
require_relative 'day03'
class RucksackTest < Minitest::Test
  def test_rucksacks
    [
      Rucksack.new('vJrwpWtwJgWr'.chars, 'hcsFMMfFFhFp'.chars),
      Rucksack.new('jqHRNqRjqzjGDLGL'.chars, 'rsFMfFZSrLrFZsSL'.chars),
      Rucksack.new('PmmdzqPrV'.chars, 'vPwwTWBwg'.chars),
      Rucksack.new('wMqvLMZHhHMvwLH'.chars, 'jbvcjnnSBnvTQFn'.chars),
      Rucksack.new('ttgJtRGJ'.chars, 'QctTZtZT'.chars),
      Rucksack.new('CrZsJsPPZsGz'.chars, 'wwsLwLmpwMDw'.chars)
    ]
  end

  def test_parse
    input = File.read('../input/day03_test.txt')
    result = parse(input)
    expected = test_rucksacks

    assert_equal(expected, result)
  end

  def test_common_item
    expected = %w[p L P v t s]
    result = test_rucksacks.map(&:common_item).compact

    assert_equal(expected, result)
  end

  def test_priority
    assert_equal(16, priority('p'))
    assert_equal(38, priority('L'))
    assert_equal(42, priority('P'))
    assert_equal(22, priority('v'))
    assert_equal(20, priority('t'))
    assert_equal(19, priority('s'))
  end

  def test_part_one
    rucksacks = test_rucksacks
    result = part_one(rucksacks)
    assert_equal(157, result)
  end

  def test_part_two
    rucksacks = test_rucksacks
    result = part_two(rucksacks)
    assert_equal(70, result)
  end
end