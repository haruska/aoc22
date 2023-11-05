# frozen_string_literal: true

class Rucksack
  attr_accessor :first_half, :second_half

  def initialize(first_half, second_half)
    @first_half = first_half
    @second_half = second_half
  end

  def self.from_input(s)
    middle = s.size / 2
    chars = s.chars
    new(chars[...middle], chars[middle..])
  end

  def ==(other)
    other.class == self.class && other.first_half == self.first_half && other.second_half == self.second_half
  end


  def common_item
    @first_half.each do |c|
      return c if @second_half.include?(c)
    end
    nil
  end

  def unique_items
    (@first_half + @second_half).uniq
  end
end

def common_item(rucksacks)
  char_sets = rucksacks.map(&:unique_items).sort_by(&:size)
  char_sets.first.find { |c| char_sets[1..].all? { |set| set.include?(c) } }
end

def priority(c)
  if c =~ /[a-z]/
    c.ord - 'a'.ord + 1
  else
    c.ord - 'A'.ord + 27
  end
end

def part_one(rucksacks)
  rucksacks.map { |r| priority(r.common_item) }.compact.sum
end

def part_two(rucksacks)
  rucksacks.each_slice(3).map { |group| priority(common_item(group)) }.compact.sum
end

def parse(input)
  input.split("\n").map { |line| Rucksack.from_input(line) }
end

if $PROGRAM_NAME == __FILE__
  input = File.read('../input/day03.txt')
  rucksacks = parse(input)

  sum = part_one(rucksacks)
  puts "Sum of common items priority (part 1): #{sum}"

  sum = part_two(rucksacks)
  puts "Sum of badges (part 2): #{sum}"
end