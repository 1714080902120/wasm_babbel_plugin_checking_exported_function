use std::sync::Mutex;
#[macro_use]
mod my_macro;
mod tools;
use js_sys::{global, Array, Function, JsString, Number, Object, Promise, Reflect};
use regex::Regex;
use tools::get_nested_property;
use wasm_bindgen::{closure, prelude::*};

use web_sys::console;

// ExportDeclaration(path) {
//     const fileName = path?.hub?.file?.opts?.filename;
//     if (fileName.includes('\\utils\\' || '\\api\\' || '\\wxs\\') && !fileName.includes('node_modules') && fileName.includes('.js')) {
//       const node = path?.node || {};
//       const nodeDeclaration = node?.declaration || {};
//       if (node.leadingComments == undefined) {
//         if (nodeDeclaration.type === 'FunctionDeclaration') {
//           const item = nodeDeclaration.id;
//           const { line, column } = item?.loc?.start;
//           console.log(`公共文件${"\033"}[40;33m${fileName}${"\033"}[0m里的方法${"\033"}[40;32m${item.name}${"\033"}[0m没有写注释，麻烦遵循规范，谢谢！位置：第${line}行${column}列`)
//           // throw new Error(`公共文件里的方法${"\033"}[40;32m${item.name}${"\033"}[0m没有写注释，麻烦遵循规范，谢谢！位置：第${line}行${column}列`)
//         } else if (nodeDeclaration.type === 'VariableDeclaration') {
//           node?.declaration?.declarations?.map(e => {
//             const { line, column } = node?.loc?.start || {};
//             if (['ArrowFunctionExpression', 'FunctionExpression'].includes(e?.init?.type)) {
//               console.log(`公共文件${"\033"}[40;33m${fileName}${"\033"}[0m里的方法${"\033"}[40;32m${e.id.name}${"\033"}[0m没有写注释，麻烦遵循规范，谢谢！位置：第${line}行${column}列`)
//               // throw new Error(`公共文件里的方法${"\033"}[40;32m${e.id.name}${"\033"}[0m没有写注释，麻烦遵循规范，谢谢！位置：第${line}行${column}列`)
//             }
//             return e
//           })
//         }
//       }
//     }
//   }

#[wasm_bindgen]
pub fn func(path: &Object) {
    let file_name: JsString =
        get_nested_property(&path, &vec!["hub", "file", "opts", "filename"], 0)
            .unwrap_or_else(|_e| {
                console::warn_1(&_e);
                JsString::from("").into()
            })
            .into();
    let file_name = file_name.as_string().unwrap_or_else(|| String::new());
    let reg =
        Regex::new(r#"^(?:[^\\]*\\){2,}(utils|api|wxs)[^\\]*\\.*\.js$"#).expect("get regexp fail");
    if !file_name.contains("node_modules") && reg.is_match(&file_name) {
        let node = get_nested_property(&path, &vec!["node"], 0).unwrap_or_else(|_e| {
            console::warn_1(&_e);
            JsValue::undefined()
        });
        let node_declaration =
            get_nested_property(&node, &vec!["declaration"], 0).unwrap_or_else(|_e| {
                console::warn_1(&_e);
                JsValue::undefined()
            });
        let node_leading_comments = get_nested_property(&node, &vec!["leadingComments"], 0)
            .unwrap_or_else(|_e| {
                console::warn_1(&_e);
                JsValue::undefined()
            });
        let line: Number = get_nested_property(&node, &vec!["loc", "start", "line"], 0)
            .unwrap_or_else(|_| Number::from(-1).into())
            .into();

        let column: Number = get_nested_property(&node, &vec!["loc", "start", "column"], 0)
            .unwrap_or_else(|_| Number::from(-1).into())
            .into();
        // if file_name.contains("purchase.js") {
        //     let ttt =
        //         get_nested_property(&node_declaration, &vec!["type"], 0).unwrap_or_else(|_e| {
        //             console::log_1(&_e);
        //             JsString::from("").into()
        //         });
        //     if ttt.as_string().unwrap_or_else(|| "".to_string()).as_str() == "VariableDeclaration" {
        //         let declarations: Array =
        //             get_nested_property(&node_declaration, &vec!["declarations"], 0)
        //                 .unwrap_or_else(|_e| {
        //                     console::log_1(&_e);
        //                     Array::new().into()
        //                 })
        //                 .into();
        //         console::log_1(&declarations);
        //         let res = declarations.map(&mut |e, _i, _arr| {
        //             let init_type = get_nested_property(&e, &vec!["init", "type"], 0)
        //             .unwrap_or_else(|_e| {
        //                 console::log_1(&_e);
        //                 JsString::from("").into()
        //             }).as_string().unwrap_or_else(|| String::new());
        //             let item_name = get_nested_property(&e, &vec!["id", "name"], 0).unwrap_or_else(|_e| {
        //                 console::log_1(&_e);
        //                 JsString::from("").into()
        //             }).as_string().unwrap_or_else(||String::new());
        //             if item_name.contains("mallBuy") {
        //                 console::log_1(&e);
        //             }
        //             match init_type.as_str() {
        //                "ArrowFunctionExpression" | "FunctionExpression" => {
        //                 console::log_1(&JsString::from(format!(
        //                     "公共文件“{}”里的方法“{}”没有写注释，麻烦遵循规范，谢谢！位置：第“{}”行“{}”列",
        //                     file_name, item_name, &line, &column
        //                 )));
        //                },
        //                _ => {}
        //             }
        //         e
        //     });
        //     }
        // }
        match (
            get_nested_property(&node_declaration, &vec!["type"], 0)
                .unwrap_or_else(|_e| {
                    console::warn_1(&_e);
                    JsString::from("").into()
                })
                .as_string()
                .unwrap_or_else(|| "".to_string())
                .as_str(),
            node_leading_comments.is_undefined(),
        ) {
            ("FunctionDeclaration", true) => {
                let item =
                    get_nested_property(&node_declaration, &vec!["id"], 0).unwrap_or_else(|_e| {
                        console::warn_1(&_e);
                        JsValue::undefined()
                    });

                let item_name = get_nested_property(&item, &vec!["name"], 0)
                    .unwrap_or_else(|_e| {
                        console::warn_1(&_e);
                        JsString::from("").into()
                    })
                    .as_string()
                    .unwrap_or_else(|| String::new());

                console::warn_1(&JsString::from(format!(
                    "\x1B[32mGreen 公共文件“{}”里的方法“{}”没有写注释，麻烦遵循规范，谢谢！位置：第“{}”行“{}”列\x1B[0m",
                    file_name, item_name, &line, &column
                )));
            }
            ("VariableDeclaration", true) => {
                let declarations: Array =
                    get_nested_property(&node_declaration, &vec!["declarations"], 0)
                        .unwrap_or_else(|_e| {
                            console::warn_1(&_e);
                            Array::new().into()
                        })
                        .into();

                let mut map_clo = |e, _i, _arr| {
                    let init_type = get_nested_property(&e, &vec!["init", "type"], 0)
                        .unwrap_or_else(|_e| {
                            console::warn_1(&_e);
                            JsString::from("").into()
                        })
                        .as_string()
                        .unwrap_or_else(|| String::new());

                    match init_type.as_str() {
                        "ArrowFunctionExpression" | "FunctionExpression" => {
                            let item_name = get_nested_property(&e, &vec!["id", "name"], 0)
                                .unwrap_or_else(|_e| {
                                    console::warn_1(&_e);
                                    JsString::from("").into()
                                })
                                .as_string()
                                .unwrap_or_else(|| String::new());
                            console::warn_1(&JsString::from(format!(
                            "\x1B[32mGreen 公共文件“{}”里的方法“{}”没有写注释，麻烦遵循规范，谢谢！位置：第“{}”行“{}”列\x1B[0m",
                            file_name, item_name, &line, &column
                        )));
                        }
                        _ => {}
                    }
                    e
                };

                let res = declarations.map(&mut map_clo);
            }
            _ => {}
        }
    }
}

// static MY_PLUGIN_OBJ: Lazy<Mutex<Object>> = Lazy::new(|| Mutex::new(Object::new()));

#[wasm_bindgen]
pub fn init() -> Result<Object, JsValue> {
    let mut obj = Object::new();
    Reflect::set(&obj, &JsString::from("name"), &JsString::from("my_plugin"))?;
    let mut visitor = Object::new();
    let func =
        wasm_bindgen::closure::Closure::wrap(Box::new(func) as Box<dyn FnMut(&Object) + 'static>);
    Reflect::set(
        &visitor,
        &JsString::from("ExportDeclaration"),
        &func.into_js_value(),
    )?;

    Reflect::set(&obj, &JsString::from("visitor"), &visitor)?;

    Ok(obj)
}
