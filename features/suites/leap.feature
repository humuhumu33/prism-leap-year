Feature: leap

  The Prism application in app/: given a year as decimal text, it answers
  "leap" or "common" by the Gregorian rule, and "invalid request" for
  anything that is not a year.

  @LY-01 @build
  Scenario: The app answers leap exactly when the year is divisible by 4 and, if divisible by 100, also by 400
    Given the leap-year model in app/src/LeapYear.lex.tex and its rule in app/src/LeapRule.lex.tex
    When the Lean kernel checks isLeap_decides during prismpm verify
    Then isLeap is true exactly when the year is divisible by 4 and, if divisible by 100, also by 400
    And the answer the application returns is decided by that same isLeap
