use boa_engine::{js_string, Context, JsValue, Source};
use colored::Colorize;
use ignore::Walk;
use rayon::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args().skip(1);
    let filter = args.next().ok_or("filter expression not provided")?;

    let src = format!("(line => ({filter}))");
    // test the provided filter to make sure it's valid
    let mut context = Context::default();
    context
        .eval(Source::from_bytes(&src))?
        .as_callable()
        .ok_or("expression evaluates to function")?
        .call(
            &JsValue::undefined(),
            &[js_string!("An arbitrary string to test the filter expression").into()],
            &mut context,
        )?;

    let current_dir = std::env::current_dir()?;
    let walker = Walk::new(current_dir).par_bridge();
    walker.for_each(|dent_result| {
        let dent = match dent_result {
            Ok(dent) => dent,
            Err(err) => {
                eprintln!("{err}");
                return;
            }
        };
        let Some(file_type) = dent.file_type() else {
            return;
        };
        if !file_type.is_file() {
            return;
        }

        let path = dent.path();

        let Some(file_name) = path.file_name().and_then(std::ffi::OsStr::to_str) else {
            return;
        };
        let file_contents = match std::fs::read_to_string(path) {
            Ok(s) => s,
            Err(err) => {
                eprintln!("{err}");
                return;
            }
        };

        let pad = (file_contents.lines().count() + 1).to_string().len();

        let mut context = Context::default();
        let function = context
            .eval(Source::from_bytes(&src))
            .expect("the provided filter is valid")
            .as_callable()
            .cloned()
            .expect("the provided filter is valid");

        let mut printed_file_name = false;

        for (line_number, line) in file_contents.lines().enumerate() {
            let res = match function.call(
                &JsValue::undefined(),
                &[js_string!(line).into()],
                &mut context,
            ) {
                Ok(v) => v,
                Err(e) => {
                    let line_number = line_number + 1;
                    eprintln!("error @ {file_name}:{line_number}: {e}");
                    continue;
                }
            };
            if res.to_boolean() {
                if !printed_file_name {
                    println!("{}", file_name.blue());
                    printed_file_name = true;
                }
                let line_number = format!("{n:>pad$}", n = line_number + 1);
                println!("{n}:{line}", n = line_number.green());
            }
        }
    });

    Ok(())
}
