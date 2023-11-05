module Day01
  def self.parse(input)
    input.split("\n\n").map do |s|
      s.split("\n").map(&:to_i).reduce(:+)
    end
  end

  def self.top_elf(elves)
    elves.max
  end

  def self.top_elves(elves, count)
    elves.sort.reverse.take(count).reduce(:+)
  end

  def self.part1(input)
    top_elf(parse(input))
  end

  def self.part2(input)
    top_elves(parse(input), 3)
  end

  def self.run
    input = File.read("../input/day01.txt")
    puts "Part 1: #{part1(input)}"
    puts "Part 2: #{part2(input)}"
  end
end

if $PROGRAM_NAME == __FILE__
  Day01.run
end