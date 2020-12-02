use neon::prelude::*;
use serde_json::Value;
use std::fs::File;
use std::io::BufReader;

fn count_items(mut cx: FunctionContext) -> JsResult<JsString> {
    let filename = cx.argument::<JsString>(0)?;

    let f = File::open(filename.value()).unwrap();
    let f = BufReader::new(f);

    // Parse the string of data into serde_json::Value.
    let data: Value = serde_json::from_reader(f).unwrap();

    let count = match data {
        Value::Array(array) => Some(array.len()),
        _ => None,
    };

    Ok(cx.string(count.unwrap_or(0).to_string()))
}

register_module!(mut cx, {
    cx.export_function("count_items", count_items);
    Ok(())
});
