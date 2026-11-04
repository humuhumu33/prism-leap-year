# Leap year

A Prism application that tells whether a Gregorian year is a leap year.
Send it a year as decimal text; it answers `leap` or `common`, and
`invalid request` for anything that is not a year.

Live: https://humuhumu33.github.io/prism-leap-year/ — the page loads the
app, verifies its fingerprint in your browser, and lets you send a year
and replay the twelve examples the archive carries.

## The claim

`LY-01`: the app answers `leap` exactly when the year is divisible by 4
and, if divisible by 100, also by 400.

The rule is stated as a proposition, `Leap`, in
[`app/src/LeapRule.lex.tex`](app/src/LeapRule.lex.tex), and the theorem
`isLeap_decides` proves that the executable check `isLeap` in
[`app/src/LeapYear.lex.tex`](app/src/LeapYear.lex.tex) is true exactly
when `Leap` holds, for every natural number. The Lean kernel checks the
theorem every time `prismpm verify` builds the app, and the test
`leap_ly_01` pins the theorem's statement, the rule in `Leap`, and that
the app's answer is decided by `isLeap`, so weakening any of them fails
the gate.

Attestation of the proof, from the UOR MCP server:
`3e8e57c9f6b7e1355927f25eba2d434bdd93c54b65d69aeef50bfa87fa67ae1d`.
Build attestation, from `prismpm verify`:
`cc4dfb329378e97fe0e492a5febfca2049974c3bea653dd23e50de7f06f382d6`.

## Check it yourself

In the browser: open the live page, read `verified` and the fingerprint,
press Replay all.

From a shell, with Node 22:

```sh
curl -sLO https://humuhumu33.github.io/prism-leap-year/app.holo
npx prism-code@0.2.0 vectors app.holo
npx prism-code@0.2.0 inspect app.holo
```

## Build it yourself

The whole gate runs inside the SDK image `prismpm.lock` pins; nothing
else is installed. In the devcontainer, or any shell inside that image:

```sh
just vv
```

`just app` alone formats and locks the model, then runs `prismpm verify`,
which builds the holo, replays its examples, and has the Lean kernel check
the theorem. It took 13 minutes on one laptop core.

## How it was made

From [UOR-Foundation/template](https://github.com/UOR-Foundation/template),
following the `prism-new-app` skill at https://uor.foundation/new.md, with
the model checked and proved through https://mcp.uor.foundation/ before it
was built here. Two shapes in the model are deliberate: the theorem lives
in its own entrypoint module that the executable module does not import,
and the year is parsed digit by digit from bytes; both are workarounds for
PrismPM limits recorded in the commit history.

Licensed under [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE).
