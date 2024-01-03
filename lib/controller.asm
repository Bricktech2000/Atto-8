primary_up! x01 @const
primary_down! x02 @const
primary_left! x04 @const
primary_right! x08 @const
secondary_up! x10 @const
secondary_down! x20 @const
secondary_left! x40 @const
secondary_right! x80 @const

controller_null! x00 @const

# converts controller states to "deltas"
# `u8.add`ing a delta to a `u4u4` treated as `(x, y)` coordinates
# will "move" the `u4u4` in the direction the controller dictates

primary_to_delta! # delta = primary_to_delta(state, default)
  shr xF0 if2 # primary_up
  shr x10 if2 # primary_down
  shr xFF if2 # primary_left
  shr x01 if2 # primary_right
pop

secondary_to_delta! # delta = secondary_to_delta(state, default)
  shl x01 if2 # secondary_right
  shl xFF if2 # secondary_left
  shl x10 if2 # secondary_down
  shl xF0 if2 # secondary_up
pop
