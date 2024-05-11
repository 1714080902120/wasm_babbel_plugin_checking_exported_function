// use js_sys::Reflect;
// use js_sys::Error;
// 递归长度有限制，不好处理
// #[macro_use]
// macro_rules! get_nested_property {
//     ($obj:expr, $key:expr, $depth:expr $(, $keys:expr)* ) => {
//         if $depth > 20 {
//             panic!("Recursion depth exceeded 10");
//         } else {
//             match $obj.get(&$key) {
//                 Some(value) => match value.as_object() {
//                     Some(obj) => get_nested_property!(obj, $key $(, $keys)*, $depth + 1),
//                     None => None,
//                 },
//                 None => None,
//             }
//         }
//     };
//     ($obj:expr, $key:expr) => {
//         get_nested_property!($obj, $key, 1)
//     };
// }
