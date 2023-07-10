#![allow(unused_macros)]

macro_rules! endpoint {
    ($method:expr, $endpoint:expr, $endpointname:ident, $returntype:ident $(, $body:ident)? $(, [$($param:expr),+])?) => {
        paste::paste! {
            impl crate::HueHueHue {
                pub async fn [<$method:lower _ $endpointname>](&self $(, body: $body)? $(, $($param: impl Into<String>),+)?) -> Result<$returntype, crate::HueHueHueBackendError> {
                    #[allow(unused_mut)]
                    let mut endpoint: String = $endpoint.into();
                    $($(endpoint = endpoint.replace(format!("{{{}}}", stringify!($param)).as_str(), $param.into().as_str()));+;)?
                    let res = self.client.[<$method:lower>](format!("{}{}",self.get_base_url(), endpoint))
                        $(.json(&(body as $body)))?
                        .send()
                        .await?
                        .json()
                        .await?;

                    Ok(res)
                }
            }

            #[tauri::command]
            #[specta::specta]
            pub async fn [<$method:lower _ $endpointname>]($(body: $body, )? $($($param: String),+,)? state: tauri::State<'_, crate::HueHueHueState>) -> Result<$returntype, crate::HueHueHueBackendError> {
                let huehuehue = state.0.lock().await;
                crate::HueHueHue::[<$method:lower _ $endpointname>](&huehuehue $(,body as $body)? $(,$($param)+)?).await
            }
        }
    };
}

macro_rules! http_method {
    ($method:ident) => {
        macro_rules! $method {
            ($$endpoint:expr, $$endpointname:ident, $$returntype:ident $$(, $$body:ident)? $$(, [$$($$param:expr),+])?) => {
                endpoint!($method, $$endpoint, $$endpointname, $returntype $$(, $$body)? $$(, [$$($$param),+])?);
            };
        }
    };
}

http_method!(get);
http_method!(head);
http_method!(post);
http_method!(put);
http_method!(delete);
http_method!(connect);
http_method!(options);
http_method!(trace);
http_method!(patch);

macro_rules! handlers {
    ($($handler:ident),*) => {
        #[macro_export]
        macro_rules! bindings {
            ($path:expr) => {
                tauri_specta::ts::export(specta::collect_types![$($handler,)*], $path).unwrap();
            };
        }
        #[macro_export]
        macro_rules! huehuehue_handlers {
            ($app:expr) => {
                $app.invoke_handler(tauri::generate_handler![$($handler,)*])
            };
        }
    };
}
