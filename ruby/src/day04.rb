def parse(input)
  input.lines.map do |line|
    line.split(',').map do |part|
      part.split('-').map(&:to_i)
    end
  end
end

def fully_contained(wp)
  ((a, b), (x, y)) = wp
  (a <= x && b >= y) || (x <= a && y >= b)
end

def overlap(wp)
  ((a, b), (x, y)) = wp
  (a <= y && a >= x) || (b <= y && b >= x) || (x <= b && x >= a) || (y <= b && y >= a)
end

def contained_count(worker_pairs)
  worker_pairs.count { |wp| fully_contained(wp) }
end

def overlap_count(worker_pairs)
  worker_pairs.count { |wp| overlap(wp) }
end

if $PROGRAM_NAME == __FILE__
  input = File.read('../input/day04.txt')

  elf_pairs = parse(input)

  part_one = contained_count(elf_pairs)
  puts "Contained count (part 1): #{part_one}"

  part_two = overlap_count(elf_pairs)
  puts "Overlap Count (part 2): #{part_two}"
end
