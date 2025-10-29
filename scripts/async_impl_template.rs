// Template for converting async trait implementations
// 
// BEFORE (async_trait):
// #[async_trait]
// impl MyTrait for MyStruct {
//     async fn my_method(&self) -> Result<String> {
//         // implementation
//         Ok("result".to_string())
//     }
// }
//
// AFTER (native async):
// impl MyTrait for MyStruct {
//     fn my_method(&self) -> impl std::future::Future<Output = Result<String>> + Send {
//         async move {
//             // implementation
//             Ok("result".to_string())
//         }
//     }
// }
