STEPS =  [3, 3, 2, 0, 0].freeze

def num_valid_walks(position = 0)

  if position == STEPS.length - 1
    1
  elsif (
    position >= STEPS.length ||
    STEPS[position] == 0
  )
    0
  else
    count = 0

    max_step = STEPS[position]
    (1..max_step).each do |step|
      position += step
      count += num_valid_walks(position)
      position -= step
    end

    count
  end
end

pp num_valid_walks
