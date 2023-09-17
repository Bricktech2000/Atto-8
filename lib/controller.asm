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

primary_to_delta! # (delta, state) = primary_to_delta(default, state)
  !primary_up xo2 xF0 iff !primary_up xo2
  !primary_down xo2 x10 iff !primary_down xo2
  !primary_left xo2 xFF iff !primary_left xo2
  !primary_right xo2 x01 iff !primary_right xo2

secondary_to_delta! # (delta, state) = secondary_to_delta(default, state)
  !secondary_up xo2 xF0 iff !secondary_up xo2
  !secondary_down xo2 x10 iff !secondary_down xo2
  !secondary_left xo2 xFF iff !secondary_left xo2
  !secondary_right xo2 x01 iff !secondary_right xo2
