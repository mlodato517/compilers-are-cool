STEPS =  [3, 3, 2, 0, 0].freeze

def valid_walks(walk = [])
  position = walk.sum
  if position == STEPS.length - 1
    [walk.dup]
  elsif (
    position >= STEPS.length ||
    STEPS[position] == 0
  )
    []
  else
    walks = []

    max_step = STEPS[position]
    (1..max_step).each do |step|
      walk.push(step)
      walks += valid_walks(walk)
      walk.pop
    end

    walks
  end
end

pp valid_walks
