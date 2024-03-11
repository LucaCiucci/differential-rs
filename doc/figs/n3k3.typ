#let unn(it, ok) = if ok > 0 { text(gray, it) } else { it }

$
(
  f,
  vec(f_x, f_y, f_x),
  mat(
    vec(unn(f_(x x), #0), unn(f_(x y), #0), unn(f_(x z), #0)),
    vec(unn(f_(y x), #1), unn(f_(y y), #0), unn(f_(y z), #0)),
    vec(unn(f_(z x), #1), unn(f_(z y), #1), unn(f_(z z), #0)),
  ),
  mat(
    cases(
      unn(vec(unn(f_(x x x), #0), unn(f_(x x y), #0), unn(f_(x x z), #0)), #0),
      unn(vec(unn(f_(x y x), #1), unn(f_(x y y), #0), unn(f_(x y z), #0)), #0),
      unn(vec(unn(f_(x z x), #1), unn(f_(x z y), #1), unn(f_(x z z), #0)), #0),
    ),
    cases(
      unn(vec(unn(f_(y x x), #0), unn(f_(y x y), #0), unn(f_(y x z), #0)), #1),
      unn(vec(unn(f_(y y x), #1), unn(f_(y y y), #0), unn(f_(y y z), #0)), #0),
      unn(vec(unn(f_(y z x), #1), unn(f_(y z y), #1), unn(f_(y z z), #0)), #0),
    ),
    cases(
      unn(vec(unn(f_(z x x), #0), unn(f_(z x y), #0), unn(f_(z x z), #0)), #1),
      unn(vec(unn(f_(z y x), #1), unn(f_(z y y), #0), unn(f_(z y z), #0)), #1),
      unn(vec(unn(f_(z z x), #1), unn(f_(z z y), #1), unn(f_(z z z), #0)), #0),
    ),
  )
)
$