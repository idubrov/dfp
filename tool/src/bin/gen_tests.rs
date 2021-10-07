//! Generator for the test cases based on tests from Intel library (`spec/readtest.in`).
use std::collections::BTreeMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use tool::parse::{TestCase, TestCaseKind};

fn read_spec() -> impl Iterator<Item = String> {
    let spec = File::open("./spec/readtest.patched.in").unwrap();
    let spec = BufReader::new(spec);

    spec.lines()
        .map(|line| {
            let mut line = line.unwrap();
            // Strip comments
            let eol = line.find("--").unwrap_or(line.len());
            line.truncate(eol);
            line
        })
        .filter(|line| !line.trim().is_empty())
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut cases = BTreeMap::new();
    for line in read_spec() {
        let case = line.parse::<TestCase>().unwrap();
        if case.kind != TestCaseKind::Unsupported {
            cases
                .entry(case.op.clone())
                .or_insert_with(Vec::new)
                .push(case);
        }
    }

    let mut out = BufWriter::new(File::create("tests/tests.rs")?);
    writeln!(out, "//! AUTOGENERATED. DO NOT EDIT")?;
    writeln!(out, "#![allow(non_snake_case)]")?;
    writeln!(out, "#![allow(clippy::unreadable_literal)]")?;
    writeln!(out)?;
    writeln!(out, "pub mod util;")?;
    writeln!(out, "use dfp::{{FpCategory, Decimal, Rounding, NearestRoundingContext, DownRoundingContext, UpRoundingContext, ZeroRoundingContext, TiesAwayRoundingContext}};")?;
    writeln!(out, "use self::util::Bits;")?;
    writeln!(out, "use std::ops::{{Add, Sub, Mul}};")?;

    for (key, cases) in cases {
        writeln!(out)?;
        writeln!(out, "#[test]")?;
        writeln!(out, "#[rustfmt::skip]")?;
        writeln!(out, "fn {}() {{", key)?;
        for case in cases {
            writeln!(out, "    {}", case)?;
        }
        writeln!(out, "}}")?;
    }

    Ok(())
}
