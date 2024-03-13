// color by count index
#let cbc(it, n) = if n == none {
  text(gray, it)
} else if n == 1 {
  text(red, it)
} else if n == 2 {
  text(green, it)
} else if n == 3 {
  text(blue, it)
} else {
  it
}

#math.bold($
(
  f,
  vec(cbc(f_x, #1), cbc(f_y, #2), cbc(f_x, #3)),
  mat(
    vec(cbc(f_(x x), #1), cbc(f_(x y), #2), cbc(f_(x z), #3)),
    vec(cbc(f_(y x), #none), cbc(f_(y y), #2), cbc(f_(y z), #3)),
    vec(cbc(f_(z x), #none), cbc(f_(z y), #none), cbc(f_(z z), #3)),
  ),
  mat(
    cases(
      cbc(vec(cbc(f_(x x x), #1), cbc(f_(x x y), #2), cbc(f_(x x z), #3)), #0),
      cbc(vec(cbc(f_(x y x), #none), cbc(f_(x y y), #2), cbc(f_(x y z), #3)), #0),
      cbc(vec(cbc(f_(x z x), #none), cbc(f_(x z y), #none), cbc(f_(x z z), #3)), #0),
    ),
    cases(
      cbc(vec(cbc(f_(y x x), #0), cbc(f_(y x y), #0), cbc(f_(y x z), #0)), #none),
      cbc(vec(cbc(f_(y y x), #none), cbc(f_(y y y), #2), cbc(f_(y y z), #3)), #0),
      cbc(vec(cbc(f_(y z x), #none), cbc(f_(y z y), #none), cbc(f_(y z z), #3)), #0),
    ),
    cases(
      cbc(vec(cbc(f_(z x x), #0), cbc(f_(z x y), #0), cbc(f_(z x z), #0)), #none),
      cbc(vec(cbc(f_(z y x), #none), cbc(f_(z y y), #0), cbc(f_(z y z), #0)), #none),
      cbc(vec(cbc(f_(z z x), #none), cbc(f_(z z y), #none), cbc(f_(z z z), #3)), #0),
    ),
  )
)
$)