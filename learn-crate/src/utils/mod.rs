mod http;

pub use http::get;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::error::{AppError, AppResult};
pub fn test() {
    println!("I am a function in the utils module.")
}

/**
 * 手动将是错误信息转换成 AppError
 */
pub fn to_json_with_app_error<T: Serialize>(data: &T) -> Result<String, AppError> {
    let json = serde_json::to_string(data);
    match json {
        Ok(j) => Ok(j),
        Err(e) => Err(AppError::JsonError(e)),
    }
}
/**
* 使用 Result<T, E> 封装错误信息 使用 ? 运算符

*/
pub fn to_json<T: Serialize>(data: &T) -> AppResult<String> {
    Ok(serde_json::to_string(data)?)
}

/**
 * 反序列时候使用生命周期
 */
pub fn from_json_with_life_cycle<'a, T: Deserialize<'a>>(s: &'a str) -> AppResult<T> {
    Ok(serde_json::from_str(s)?)
}
/**
 * 反序列时候使用 trait bound
 */
pub fn from_json<T: DeserializeOwned>(s: &str) -> AppResult<T> {
    Ok(serde_json::from_str(s)?)
}
