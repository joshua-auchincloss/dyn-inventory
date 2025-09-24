use std::io::Write;

pub trait Greeter {
    fn handle(
        &self,
        value: String,
        w: &mut dyn Write,
    ) -> std::io::Result<()>;
}

dyn_inventory::dyn_inventory! {
    Flag<Handle: Greeter> {
        flag_name: &'static str,
        flag: Handle
    };
}

mod name {
    use crate::*;
    use dyn_inventory::emit;

    emit! {
        Handle Greeter as Flag {
            flag_name = "name"
        }
    }

    impl Greeter for Handle {
        fn handle(
            &self,
            name: String,
            w: &mut dyn Write,
        ) -> std::io::Result<()> {
            writeln!(w, "Hello, {name}")
        }
    }
}

pub fn run_args<A: IntoIterator<Item = String>, W: Write>(
    args: A,
    wr: &mut W,
) -> std::io::Result<()> {
    let mut args = args.into_iter();

    let _ = args.next();
    let rem: Vec<_> = args.collect();

    let flags = FlagCollector::new();

    let mut skip = vec![];
    for (pos, flag) in rem.iter().enumerate() {
        if skip.contains(&pos) {
            continue;
        }

        let (flag, maybe_value) = maybe_value(flag.clone());

        let value = if let Some(value) = maybe_value {
            value
        } else {
            skip.push(pos + 1);

            rem.get(pos + 1)
                .expect("value must be provided")
                .clone()
        };

        for known in &flags.plugins {
            if flag == known.flag_name {
                known.flag.handle(value.clone(), wr)?;
            }
        }
    }
    Ok(())
}

fn maybe_value(flag: String) -> (String, Option<String>) {
    let pre = flag.trim_start_matches("--");
    if pre.contains("=") {
        let mut split = pre.split("=");
        let (flag, value) = (split.next().unwrap(), split.next().unwrap());
        (flag.into(), Some(value.into()))
    } else {
        (pre.into(), None)
    }
}

#[cfg(test)]
mod test {

    use test_case::test_case;

    #[test_case(vec!["bin", "--name=Alice"], "Hello, Alice\n"; "flag with equals")]
    #[test_case(vec!["bin", "--name", "Bob"], "Hello, Bob\n"; "flag with space")]
    #[test_case(vec!["bin", "--name=Charlie"], "Hello, Charlie\n"; "another name")]
    #[test_case(vec!["bin", "--name", "Dana"], "Hello, Dana\n"; "space variant")]
    fn test_run_args(
        args: Vec<&str>,
        expected: &str,
    ) {
        let mut buf = Vec::new();

        super::run_args(args.into_iter().map(String::from), &mut buf).unwrap();

        let read = String::from_utf8_lossy(&buf);
        assert_eq!(read.into_owned(), expected);
    }
}
