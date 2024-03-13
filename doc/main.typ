
//#import "@preview/charged-ieee:0.1.0": ieee
#import "common_styles.typ": *
#import "boxes.typ": *
#import "project.typ": *
#import "defs.typ": *
#import "@preview/algo:0.3.3": algo, i, d, comment, code

#show: common_styles
#show: project.with(
  title: "Efficient Higher Order Multi-variable Forward Accumulation mode Automatic Differentiation",
  authors: (
    (
      name: "Luca Ciucci",
      affiliation: "University of Pisa",
      department: "Department of Physics",
      location: "Pisa, Italy",
      email: "luca.ciucci99@gmail.com",
    ),
  ),
  abstract: [
    We present a novel /*TODO maybe advanced/generic instead of advances??*/ implementation of the forward mode automatic differentiation algorithm written in Rust, named #diff-package. This package enables the computation of derivatives of any order and for any number of variables efficiently. The proposed implementation allows for a flexible approach to be used in the implementation, i.e. both a _Taylor expansion_-based and _nested_-based approach can coexist. The implementation is also generic over the underlying field used to store derivatives and allows for both dynamic and compile time shape instantiation.
  ],
)

//#show page: (page) => {
//  page
//}

= Introduction

Automatic Differentiation (AD) is a method that provides a way to compute the derivative of a function by using the chain rule without explicitly computing finite differences as they are subjected both to truncation error and round-off error @autodiff-in-machine-learning. Not all problems require an accurate derivatives estimation, for example in some optimization problems @adam, but in other cases we may want a correct and efficient estimation of the derivatives, for example in some Monte Carlo @stochastic-ADMC or Hybrid Monte Carlo @autodiff-in-machine-learning processes.\
AD is usually divided into two main categories: Forward Accumulation mode (FAD) and Reverse Accumulation mode (RAD). In this work we focus on the FAD, which is more suitable for problems with a small number of input variables.\
Hybrid methods that combine the two modes are also possible, but they are not the focus of this work.\

FAD can easily be implemented for the first order, but higher orders are less trivial to implement. A naive approach to the problem consists in composing the derivative operation until the desired order is reached, but this requires a combinatorial number of operations and memory. // TODO reference https://openreview.net/forum?id=SkxEF3FNPH
This approach is described in @ad-first-order-ad.\
Another approach is to use a Taylor expansion polynomial, but the resulting implementation for some primitives can become complex to express and maintain. Also in this implementation the polynomial coefficients are stored and computed, while, in practice, we are usually interested just in the value of the derivatives.


= Target usage <target-usage>

This work focuses on a practical implementation of FAD in Rust and the target usage consist in the differentiation of specific functions or generic-typed functions in the form:
```rust
fn f<T: Real>(x: T) -> T {
  // ...
}
```
where `Real` is a trait that represents a real number, and `T` is a generic type that implements the `Real` trait.\
This is a natural approach with statically typed languages, but other possibilities have been explored before, for example leveraging on source code or IR manipulation #ref-enzyme. We focus on a regular generic based approach since it is a straightforward approach and relatively simple to use for many applications as it does not require a specific compiler pipeline, even though it requires the code to be compatible with the provided types.

= Possible implementations for FAD <possible-implementations>

The core of the implementation consists in choosing the representation. We present some obvious choices.

We (possibly improperly) call the type defined by the implementation `Differential` as it is the name chosen in our implementation.

== 1D Taylor series <ad-1d-taylor-series>

This may be the most obvious choice. In this implementation the `Differential` is just the arbitrary order 1D Taylor expansion of the function.\
Many basic operations are defined in terms of the Taylor expansion by just using the basic operations defined on the Taylor polynomial.\

With this implementation it is not always straightforward to write the expansion of every operation as working with arbitrary order $N$-dimensional Taylor expansion is not trivial.// Also higher order coefficients may end up being very small. // TODO meaningless

== Plain first-order AD <ad-first-order-ad>

This may look like a downgrade compared to the first implementation as it computes just the first derivative.\
The advantage is that now expressing basic operations is trivial and there is an obvious way of representing higher order derivatives as shown below.

We define the differential:
#let differential(f) = $D lr(angle.l #f angle.r)$
#let jac(f) = $bold(J) lr(angle.l #f angle.r)$
$
differential(cal(F)) = (cal(F), jac(cal(F)))
$ <ad-differentials-composition>
where $cal(F)$ is a field and $jac(cal(F))$ is a jacobian object over the space of $cal(F)$. This notation is chosen to resemble the practical Rust implementation. \
Then, a first order differential like described in @ad-differentials-composition would just be $differential(RR)$.\
In order to compute higher orders we can just compose derivatives and define a second order differential as $differential(differential(RR))$, this can be iterated to any order.

As a practical example, let's analyze what $D(D(RR))$ is:
$
differential(differential(RR)) = (differential(RR), jac(differential(RR))) = ((RR, jac(RR)), (jac(RR), jac(jac(RR))))
$
This, unfortunately, explicitly requires and exponential amount of memory and combinatorial number of operations to compute and store the derivatives.\
If, for example, we want to take the derivative with respect to two variables $x$ and $y$, a differential would look like:
$
((f, f_y), (f_x, f_(x y)))
$
where we have only one second order derivative.\
Also, if we want the second order with respect to the same variable, we would get:
$
((f, f_x), (f_x, f_(x x)))
$ <ad-differential-composition-repetition-problem>
which is sub-optimal as the first order derivatives are computed and stored twice.\

== Hybrid representation approach <ad-hybrid-representation-approach>

If we want to recover the advantages of @ad-1d-taylor-series in terms of memory layout, we would like to represent the terms of the `Differential` as a tuple:
$
(f, underbracket(diff_alpha f, #[jacobian]), underbracket(diff_alpha^2 f, #[hessian]), diff_alpha^3 f, ...)
$ <ad-differential-serialized>
that, for a single variable, is the same layout of @ad-1d-taylor-series.

The number of derivatives is still the same of @ad-1d-taylor-series, but we could leverage on the symmetry of second derivatives to reduce the number of derivatives to compute and store since, for the functions we are usually interested in computing derivatives of, derivatives will commute. // conditions ??? we are interested in well behaved cases anyway

If we try to write down the 3rd order differential for 3 variables, we would get @fig-n3k3.

//#place(
  //auto,
  //float: true,
  //[
    #figure(
    text(10pt, include "figs/n3k3.typ"),
    caption: [
      Explicit representation of the 3rd order differential for 3 variables as a tuple. Spatial organization has the sole purpose of being visually interpretable. \
      Repeated elements are grayed out.
    ],
  ) <fig-n3k3>
  //],
//)

And we see that there are $1 + 3 + 9 + 27 = #(1 + 3 + 9 + 27)$ elements, but only $1 + 3 + 6 + 10 = #(1 + 3 + 6 + 10)$ unique values because of @symmetry-of-second-derivatives.\
These values can be stored in an array but the indexing operation requires a non trivial mapping function.

We decided to use this approach in our implementation as it requires the minimum amount of memory.

The different pieces of the array can then be interpreted as:
$
(f, diff_alpha f, diff_alpha^2 f, diff_alpha^3 f, ..., diff_alpha^K f)
$ <ad-differential-serialized-plain>
that has a useful property: if we recall @ad-differentials-composition we see that this can also be viewed like @ad-differentials-composition as:
$
(underbracket(#$f, diff_alpha f, diff_alpha^2 f, diff_alpha^3 f, ...$, #[value]), diff_alpha^K f)
<==>
(f, underbracket(#$diff_alpha f, diff_alpha^2 f, diff_alpha^3 f, ..., diff_alpha^K f$, #[derivative]))
$ <ad-differential-serialized-plain-reinterpretation>
Where considering the "value" or the "derivative" is just a matter of taking the reference to the first or last elements of the array and just reinterpreting them as a `Differential` of order $K - 1$ or a Jacobian made of #(`Differential`)s of order $K - 1$.

= Indexing <memory-layout>

The base of our our mapping is computing the number of elements in in our representation. The following formula is used (see #code_ref("src/layout.rs", line: "fn number_of_elements_impl")):
$
"#"(N, K) = cases( // TODO maybe "#elements" instead of "#"
  1 space "if" K = 0,
  1 + sum_(n = 1)^(N) "#"(n, K - 1)
)
$ <ad-differential-serialized-size>
Where $N$ is the number of variables and $K$ is the order of the differential.

The meaning of @ad-differential-serialized-size can be seen in @fig-n3k3-counting-colored.
In fact, we can write the set of derivatives:
$
D_(N, K) = union.big_(k in 0..K) { lr((diff^(k) f )/(diff_(x_1)^k_1 diff_(x_2)^k_2 ... diff_(x_N)^k_N) |)_(sum k_i = k) }
$ <dnk-set>
and if $K > 0$:
$
D_(N, K) = {f} union union.big_(n in 1..N) union.big_(k in 0..K) { diff/diff_(x_n) lr((diff^(k) f )/(diff_(x_1)^k_1 diff_(x_2)^k_2 ... diff_(x_N)^k_N) |)_cases(
  sum k_i = k - 1,
  k_i = 0 space "for" i > n
) }
$ <dnk-set-expanded>
#todo[
  verify that @dnk-set-expanded is correct
]
so:
$
\# D_(N, K) &= 1 + sum_(n in 1..N) union.big_(k in 0..K) { diff/diff_(x_i) lr((diff^(k) f )/(diff_(x_1)^k_1 diff_(x_2)^k_2 ... diff_(x_N)^k_N) |)_cases(
  sum k_i = k - 1,
  k_i = 0 space "for" i > n
) }\
&= 1 + sum_(n in 1..N) union.big_(k in 0..K) \# D_(n, K - 1)
$ <cardinality-dnk-set>
that is @ad-differential-serialized-size.

#place(
  auto,
  float: true,
  [#figure(
    text(10pt, include "figs/n3k3-counting-colored.typ"),
    caption: [
      Visual representation of a differential. Derivatives where $n_y = n_z = 0$ and $n_x > 0$ are colored in red, derivatives where $n_z = 0$ and $n_x, n_y > 0$ are colored in green, derivatives where $n_x, n_y, n_z > 0$ are colored in blue.
    ],
  ) <fig-n3k3-counting-colored>],
)

#place(
  auto,
  float: true,
  [#figure(
    image("figs/number_of_elements.svg", width: 60%),
    caption: [
      "Number of elements in the representation of a differential of order $K$ for $N$ variables."
    ],
  )],
)

#todo[
  - describe all the mapping functions
  //- ELI5
  - port them in a more mathematical form, justify them
  - try to find a closed forms
  - analyze the size trend (i suppose polynomial $O(K^N)$ but i'm not sure)
  - analyze the complexity of the mapping functions
]

= Primitives implementation

In order for this library to be useful, we need basic operations to be defined on #(`Differential`)s.

Defining operations over all the possible shapes of the `Differential` may look challenging, and in fact it is if we want to explicitly write them down using the Taylor expansion. But @ad-differential-serialized-plain-reinterpretation provides a convenient way for expressing the operations in terms of the first order differential. This is what a nested approach with derivative composition would do.\

We now show some relevant implementation examples.

== Multiplication <multiplication>

Let us consider the case of the multiplication of two differentials `mul(a, b)` because it is most important non trivial operation.

For order 1, the chain rule gives us:
$
(f, diff_alpha f) (g, diff_alpha g) = (f g, f space diff_alpha g + g space diff_alpha f)
$

We then have:
#algorithm(
  title: [Nested multiplication algorithm],
  label: "nested-mul"
)[
  #algo(
    title: "mul",
    parameters: ($f$, $g$),
  )[
    if $K$ == 0: #i\
      return $"val"(f) * "val"(g)$#d\
    else: #i\
      return (#i\
        $"val"(f) * "val"(g)$, #comment($f g$)\
        $"val"(f) * "deriv"(g) + "val"(g) * "deriv"(f)$ #comment($f space diff_alpha g + g space diff_alpha f$)#d\
      )#d
  ]
]
where $f$ and $g$ are the differentials and $K$ is the order of the differential.
The corresponding relevant Rust code would look something like (see #code_ref("src/impls/std_ops/mul.rs", line: "let derivatives = lhs.derivatives() * &rhs.drop_one_order() + rhs.derivatives() * &lhs.drop_one_order();")):
```rs
a.derivatives() * b.drop_one_order() + b.derivatives() * a.drop_one_order()
```

This means that for any shape of the differential we can easily express any operation in terms of the first order differential.

In practice, for the multiplication case, this would be sub-optimal as we would compute the intermediate pieces ```rs a.derivatives() * b.drop_one_order()``` and, by recursion, produce an exponential complexity with respect to the order of the differential.\
We know though that for the univariate case ($N = 1$) there is an explicit form that is the Faà di Bruno's formula @faa-di-bruno-thesis. This is implemented in #code_ref("src/impls/std_ops/mul.rs", line: "data_slice[i + j] += lhs[i].clone() * &rhs[j];")\
The current implementation actually converts the coefficients in polynomial form and then back to the differential form in order to avoid computing the factorial terms, but this might not be an optimal choice and there is certainly room for improvement.\
At the time of write, the implementation uses the general @nested-mul for the multivariate case but, as a future work, this could be optimized since @nested-mul order has exponential complexity with respect to the order of the differential.

== Square root <square-root>

A slightly more elaborate example is the square root operation.\
The chain rule gives:
$
sqrt((f, diff_alpha f)) = (sqrt(f), frac(diff_alpha f, 2 sqrt(f)))
$
that can be implemented with @general-sqrt.
#algorithm(
  title: [Plain square root algorithm],
  label: "general-sqrt"
)[
  #algo(
    title: "sqrt",
    parameters: ($f$,),
    breakable: true,
  )[
    if $K$ == 0: #i\
      return $sqrt(f_0)$#d\
    else: #i\
      let $r$ = SQRT(val($f$)) #comment[recursion for $sqrt(f)$]\
      return (#i\
        $r_0$, #comment($sqrt(f)$)\
        $"deriv"(f) \/ 2r$ #comment($frac(diff_alpha f, 2 sqrt(f))$)#d\
      )#d
  ]
]
There is also another obvious way of computing the square root, which is the Heron's method @heron-sqrt described in in @babylon_sqrt:
#algorithm(
  title: [Heron's square root AKA Babylonian square root],
  label: "babylon_sqrt",
)[
  #algo(
    title: "babylon_sqrt",
    parameters: ($x$, "n_steps"),
  )[
    $r <- sqrt(x_0)$ #comment[our initial guess is the *constant* root]\
    for \_ in 0..n_steps: #i\
      $r <- (r + x \/ r)/2$ #comment[Heron's step] #d\
    return $r$
  ]
]
In practice, we see that the derivatives converge in a few steps (usually 5), this was expected as described in @ad-convergence-in-recursive-algorithms.
#todo[
  an appropriate test and analysis of the convergence of the derivatives / error. Decide which one is the most efficient and/or accurate.

  #place(
    auto,
    float: true,
    [#figure(
      stack(dir: ltr, stack(image("time_vs_n.svg", width: 40%), [(a)]), stack(image("time_vs_order.svg", width: 40%), [(b)])),
      caption: [
        Time per evaluation of @babylon_sqrt with `n_steps = 10`.\
        (a) Time per evaluation as a function of the number of variables (first order), we see a linear trend.\
        (b) Time per evaluation as a function of the order of the differential (one variable), we see quadratic trend.
      ]
    )],
  )
]

#todo[
  converging so fast means that maybe we are solving a linear problem with newton? This would be stupid, in this case, prove that the two methods are equivalent.
]

== Exponential <exponential>

#place(
  auto,
  float: true,
  [#figure(
    image("exp_time_vs_order.svg", width: 60%),
  )],
)

#todo[
  proof that $O(K^3)$ and, possibly, that is equivalent to computing the taylor expansion.
]

#todo[
  maybe this is not a good example
]

== Sin and Cos <sin-and-cos>

#todo[
  sin and cose using nested approach: why it is better then the taylor expansion using the nested approach
]

= Static memoization <static-memoization>

#todo[
  describe compile time memoization for shape/mapping functions
]

= Allocations optimization <allocations-optimization>

#todo[
  ...
]

= Benchmarks <benchmarks>

#todo[
  ...
]

= Future work <future-work>

#todo[
  ...
]

= Conclusion <conclusion>

#todo[
  ...
]

#appendix(
  [Convergence of AD in recursive algorithms], // TODO maybe "... in fixed point search algorithms" or something like that
  label: "ad-convergence-in-recursive-algorithms",
)[
  #todo[
    generalize for generic dimension
  ]

  This simple result is required for @babylon_sqrt and might be required in some problems.

  Some algorithms may not be expressed in closed form and, instead, they rely on recursion where the solution is the fixed point of succession.\
  As an example, we might want to apply AD to the result of the Newton algorithm to find roots. In this case we have:
  $
  bold(x)_(n+1) = bold(x)_n - J^(-1)(bold(x)_n) space f(bold(x)_n)
  $ <newton-ad-application-example>
  An example is the application is the recursive algorithm described described in @ad-1d-taylor-series[?].

  Intuitively, it might look obvious that AD works for recursive algorithms because we are just taking the Taylor expansion of the function and, be it recursive or not, the Taylor expansion should always provide us a valid approximation of the function.\
  On the other hand, one might think that, while the central value converges, its is not guaranteed that the Taylor expansion converges (and if so, to what). In other words, we want to be sure that AD does not produce any "sawtooth" pattern.

  Let's consider the parameter-dependant recursive succession:
  $
  a_(n + 1) = f(a_n, p)
  $ <recursive-succession>

  #note[
    We consider only the cases where $f$ is differentiable with respect to $a$ and $p$.
  ]

  We have:
  $
  frac(d, d p) a_(n+1) &= frac(d, d p) f(a_n, p) \
  &= f_x (a_n, p) frac(d, d p) a_n + f_y (a_n, p) \
  $ <recursive-succession-derivative>
  where $f_x = frac(diff, diff x) f(x,y)$ and $f_y = frac(diff, diff y) f(x,y)$ are the partial derivatives of $f$.

  If $a_n$ converges to $a$, the equation for the fixed point of the derivative succession becomes:
  $
  frac(d, d p) a = f_x (a, p) frac(d, d p) a + f_y (a, p)
  $
  hence:
  $
  frac(d, d p) a = frac(f_y (a, p), 1 - f_x (a, p))
  $ <derivative-fixed-point>

  So the fixed point is only one and it is the correct derivative.\
  If the the $a_n$ succession converges because of the _stability criterion of the fixed point_ @fixed-point, we have $abs(f'(x_0)) < 1$ and $f in C^1$ in a neighborhood of the fixed point $x_0$, so @recursive-succession-derivative also converges for the same criterion.\
  This can be iterated for higher order derivatives so all the derivatives also converge.

  This equation would also allow us to compute the derivative of the fixed point with respect to $p$ without having to fully compute the succession using AD.

  The edge case $f_x (a, p) = 1$ is the case where it is not possible to determine the stability of the fixed point using the first derivative criterion and we would have to investigate higher order derivatives, but this is not the case we are interested in.

  For multi-variable differential, the same reasoning applies where, instead of the derivative of the fixed point, we have the Jacobian of the fixed point and the criterion $abs(f'(x_0)) < 1$ becomes $max(abs("eigenvalues"(J))) < 1$.

  #todo[
    $max(abs("eigenvalues"(J))) < 1$ sounds reasonable but I don't have a proof/reference for this at the moment.
  ]

  This explains why @babylon_sqrt and any other newton-like algorithm works with AD.

  #note[
    This gives us two optimization chances:
    + we could compute the algorithm using AD and just compute the derivatives of $f$ once we have the fixed point (this is implemented in line 1 of @babylon_sqrt).\
    + we could use @derivative-fixed-point to explicitly find the derivative and avoid iterating
    #todo[
      try to implement the second point for the square root. and compare with @general-sqrt and @babylon_sqrt. (maybe *it is the same* of @general-sqrt ???)
    ]
  ]
]

#appendix(
  [Meaning _well behaved_ recursive algorithms],
  label: "well-behaved-differentiable-algorithm",
)[
  I don't have a proper definition for this, but clearly there are algorithms where AD is meaningless.\
  For a counter example, we can consider the Metropolis-Hastings algorithm (@the-metropolis-hastings-algorithm[?]).
  We can study the last point generated by the algorithm as a function of the initial point (i.e. $x_N = f_N (x_0)$).
  We expect that, for a finite number of steps and by fixing the PRNG's seed, by changing just a little bit the initial point, the accepted/rejected steps will be the same and, as a consequence:
  $
  f_N (x_0 + delta) = f_N (x_0) + delta space "for" delta "small"
  $
  This means that AD will always produce the following result:
  $
  forall N: cases(
      x_N = f_N (x_0),
      x'_N = x'_0,
      x''_N = x''_0,
      ...
  )
  $
  This looks pretty boring and strange: we expect that, by changing even by little the initial point, the last point generated by the algorithm will change a lot.\
  The truth is that AD is giving us the correct result but, by increasing $N$, the range of validity of the taylor expansion of $f_N$ around $x_0$ decreases and should converge to $0$ as $N$ goes to infinity.\
  This tels us that taking derivatives of the MH algorithm with respect to the initial condition is meaningless. This was obvious but it is just an example of a recursive algorithm where the application of AD is meaningless.
]

#bibliography("bibliography.yaml")
