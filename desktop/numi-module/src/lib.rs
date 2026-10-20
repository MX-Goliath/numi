use neon::prelude::*;
use numi::{FormatOptions, NumiModel, NumiModelOptions};
use std::sync::Arc;
use tokio::runtime::Runtime;

struct JsNumiModel {
    runtime: Runtime,
    m: Arc<NumiModel>,
}

impl Finalize for JsNumiModel {}

impl JsNumiModel {
    fn new() -> Self {
        let model_options = NumiModelOptions {
            locale: None,
            format_options: FormatOptions::default(),
            rates_ttl: 60 * 60,
            locale_ttl: 60 * 60 * 24,
        };
        let runtime = Runtime::new().expect("Failed to create runtime");
        let model = NumiModel::new(runtime.handle().clone(), model_options, |_u| {});
        Self {
            runtime,
            m: Arc::new(model),
        }
    }

    fn js_new(mut cx: FunctionContext) -> JsResult<JsBox<JsNumiModel>> {
        let calc_model = JsNumiModel::new();
        Ok(cx.boxed(calc_model))
    }

    fn js_start(mut cx: FunctionContext) -> JsResult<JsPromise> {
        let calc_model = cx.this::<JsBox<JsNumiModel>>()?;
        let model = calc_model.m.clone();
        let (deferred, promise) = cx.promise();
        let channel = cx.channel();
        calc_model.runtime.spawn(async move {
            model.start().await;
            deferred.settle_with(&channel, |mut cx| {
                return Ok(cx.undefined());
            });
        });
        Ok(promise)
    }

    fn js_calc(mut cx: FunctionContext) -> JsResult<JsPromise> {
        let calc_model = cx.this::<JsBox<JsNumiModel>>()?;
        let model = calc_model.m.clone();
        let json_str = cx.argument::<JsString>(0)?.value(&mut cx);
        let (deferred, promise) = cx.promise();
        let channel = cx.channel();
        calc_model.runtime.spawn(async move {
            let res = model.calc_json(&json_str).await;
            deferred.settle_with(&channel, |mut cx| {
                if let Some(res) = res {
                    return Ok(cx.string(res));
                } else {
                    return Ok(cx.string("{}"));
                }
            });
        });
        Ok(promise)
    }
}

fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string("hello node"))
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("hello", hello)?;
    cx.export_function("numi_model_new", JsNumiModel::js_new)?;
    cx.export_function("numi_model_start", JsNumiModel::js_start)?;
    cx.export_function("numi_model_calc", JsNumiModel::js_calc)?;
    Ok(())
}
