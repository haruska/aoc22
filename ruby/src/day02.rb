
class Shape

  attr_reader :score

  def initialize(score)
    @score = score
  end

  ROCK = new(1)
  PAPER =  new(2)
  SCISSORS = new(3)

  def self.from_char(c)
    case c
    when "A", "X" then ROCK
    when "B", "Y" then PAPER
    when "C", "Z" then SCISSORS
    end
  end

  def for_outcome(outcome)
    case outcome
    when Outcome::DRAW then self
    when Outcome::LOST
      case self
      when ROCK then SCISSORS
      when PAPER then ROCK
      when SCISSORS then PAPER
      end
    when Outcome::WON
      case self
      when ROCK then PAPER
      when PAPER then SCISSORS
      when SCISSORS then ROCK
      end
    end
  end
end

class Outcome

  attr_reader :score

  def initialize(score)
    @score = score
  end

  LOST = new(0)
  DRAW = new(3)
  WON = new(6)

  def self.from_char(c)
    case c
    when "X" then LOST
    when "Y" then DRAW
    when "Z" then WON
    end
  end
end

class Round
  attr_reader :player_shape, :opponent_shape

  def initialize(player_shape, opponent_shape)
    @player_shape = player_shape
    @opponent_shape = opponent_shape
  end

  def outcome
    if player_shape == opponent_shape
      Outcome::DRAW
    elsif player_shape == Shape::ROCK
      if opponent_shape == Shape::SCISSORS
        Outcome::WON
      else
        Outcome::LOST
      end
    elsif player_shape == Shape::PAPER
      if opponent_shape == Shape::ROCK
        Outcome::WON
      else
        Outcome::LOST
      end
    elsif player_shape == Shape::SCISSORS
      if opponent_shape == Shape::PAPER
        Outcome::WON
      else
        Outcome::LOST
      end
    end
  end

  def score
    player_shape.score + outcome.score
  end
end


module Day02
  def self.parse(input)
    input.split("\n").map(&:chars).map {|chars| [chars[0], chars[2]]}
  end

  def self.part_one_rounds(input)
    input.map do |opponent_c, player_c|
      opponent = Shape.from_char(opponent_c)
      player = Shape.from_char(player_c)
      Round.new(player, opponent)
    end
  end

  def self.part_two_rounds(input)
    input.map do |opponent_c, outcome_c|
      opponent = Shape.from_char(opponent_c)
      desired_outcome = Outcome.from_char(outcome_c)
      player = opponent.for_outcome(desired_outcome)
      Round.new(player, opponent)
    end
  end

  def self.player_score(rounds)
    rounds.map(&:score).sum
  end

  def self.part_one(input)
    rounds = part_one_rounds(parse(input))
    player_score(rounds)
  end

  def self.part_two(input)
    rounds = part_two_rounds(parse(input))
    player_score(rounds)
  end

  def self.run
    input = File.read("../input/day02.txt")
    puts "Part 1: #{part_one(input)}"
    puts "Part 2: #{part_two(input)}"
  end
end

if $PROGRAM_NAME == __FILE__
  Day02.run
end