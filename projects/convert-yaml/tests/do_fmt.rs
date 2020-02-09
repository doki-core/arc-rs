use arc_from_yaml::to_arc;

#[test]
fn main() {
    let s =
        r#"
# Example 2.16.  Indentation determines scope
name: Mark McGwire
accomplishment: >
  Mark set a major league
  home run record in 1998.
stats: |
  65 Home Runs
  0.278 Batting Average

# Example 2.17. Quoted Scalars
unicode: "Sosa did fine.\u263A"
control: "\b1998\t1999\t2000\n"
hex esc: "\x0d\x0a is \r\n"

single: '"Howdy!" he cried.'
quoted: ' # Not a ''comment''.'
tie-fighter: '|\-*-/|'

# Example 2.18. Multi-line Flow Scalars
plain:
  This unquoted scalar
  spans many lines.

quoted: "So does this
  quoted scalar.\n"

canonical: 12345
decimal: +12345

octal: 0o14
hexadecimal: 0xC

canonical2: 1.23015e+3
exponential: 12.3015e+02
fixed: 1230.15

negative infinity: -.inf
not a number: .NAN

null:
booleans: [ true, false ]
string: '012345'

canonical3: 2001-12-15T02:59:43.1Z
iso8601: 2001-12-14t21:59:43.10-05:00
spaced: 2001-12-14 21:59:43.10 -5
date: 2002-12-14
"#;
    let t = to_arc(s);
    println!("{}", t.unwrap_or_default());
}
