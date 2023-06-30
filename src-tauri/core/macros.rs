#![allow(unused_macros)]

macro_rules! endpoint {
    ($method:expr, $endpoint:expr, $endpointname:ident, $returntype:ident $(, $body:ident)? $(, [$($param:expr),+])?) => {
        paste::paste! {
            impl crate::HueHueHue {
                pub async fn [<$method:lower _ $endpointname>](&self $(, body: $body)? $(, $($param: impl Into<String>),+)?) -> Result<$returntype, crate::HueHueHueError> {
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
            pub async fn [<$method:lower _ $endpointname>]($(body: $body, )? $($($param: String),+,)? state: tauri::State<'_, crate::HueHueHueState>) -> Result<$returntype, crate::HueHueHueError> {
                let huehuehue = state.0.lock().await;
                crate::HueHueHue::[<$method:lower _ $endpointname>](&huehuehue $(,body as $body)? $(,$($param)+)?).await
            }
        }
    };
}

macro_rules! get {
    ($endpoint:expr, $endpointname:ident, $returntype:ident $(, $body:ident)? $(, [$($param:expr),+])?) => {
        endpoint!(get, $endpoint, $endpointname, $returntype $(, $body)? $(, [$($param),+])?);
    };
}

macro_rules! head {
    ($endpoint:expr, $endpointname:ident, $returntype:ident $(, $body:ident)? $(, [$($param:expr),+])?) => {
        endpoint!(head, $endpoint, $endpointname, $returntype $(, $body)? $(, [$($param),+])?);
    };
}

macro_rules! post {
    ($endpoint:expr, $endpointname:ident, $returntype:ident $(, $body:ident)? $(, [$($param:expr),+])?) => {
        endpoint!(post, $endpoint, $endpointname, $returntype $(, $body)? $(, [$($param),+])?);
    };
}

macro_rules! put {
    ($endpoint:expr, $endpointname:ident, $returntype:ident $(, $body:ident)? $(, [$($param:expr),+])?) => {
        endpoint!(put, $endpoint, $endpointname, $returntype $(, $body)? $(, [$($param),+])?);
    };
}

macro_rules! delete {
    ($endpoint:expr, $endpointname:ident, $returntype:ident $(, $body:ident)? $(, [$($param:expr),+])?) => {
        endpoint!(delete, $endpoint, $endpointname, $returntype $(, $body)? $(, [$($param),+])?);
    };
}

macro_rules! connect {
    ($endpoint:expr, $endpointname:ident, $returntype:ident $(, $body:ident)? $(, [$($param:expr),+])?) => {
        endpoint!(connect, $endpoint, $endpointname, $returntype $(, $body)? $(, [$($param),+])?);
    };
}

macro_rules! options {
    ($endpoint:expr, $endpointname:ident, $returntype:ident $(, $body:ident)? $(, [$($param:expr),+])?) => {
        endpoint!(options, $endpoint, $endpointname, $returntype $(, $body)? $(, [$($param),+])?);
    };
}

macro_rules! trace {
    ($endpoint:expr, $endpointname:ident, $returntype:ident $(, $body:ident)? $(, [$($param:expr),+])?) => {
        endpoint!(trace, $endpoint, $endpointname, $returntype $(, $body)? $(, [$($param),+])?);
    };
}

macro_rules! patch {
    ($endpoint:expr, $endpointname:ident, $returntype:ident $(, $body:ident)? $(, [$($param:expr),+])?) => {
        endpoint!(patch, $endpoint, $endpointname, $returntype $(, $body)? $(, [$($param),+])?);
    };
}

macro_rules! handlers {
    ($($handler:ident),*) => {
        #[macro_export]
        macro_rules! bindings {
            ($path:expr) => {
                #[cfg(debug_assertions)]
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

