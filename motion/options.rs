//! test which variant
//! Result: is_ok, is_err
//! Option: is_some, is_none
//! 场景
//! 1. 需要知道类型
//! 2. 不需要获取值
//! 3. 不能测试内部值
//! 4. 文本测试
//! 5. 控制流
//! 6. 不能用于unwrap
fn test1() {
    let option_value = Some(25);
    // if option_value.is_some() {
    //     let inner = option_value.unwrap();
    //     println!("inner = {}", inner);
    // }
    if let Some(inner) = option_value {
        println!("inner = {}", inner);
    }
}

//! converting Result -> Option and Option -> Result
//! Result -> Option 使用ok方法
//! Ok(T).ok() -> Some(T)
//! Err(E).ok() -> None
//! Option -> Result 使用ok_or方法
//! Some(T).ok_or(err_value) -> Ok(T)
//! None.ok_or(err_value) -> Err(err_value)

//! Fallback values
//! unwrap_or方法
//! Ok(T).unwrap_or(fallback) -> T
//! Some(T).unwrap_or(fallback) -> T
//! Err(E).unwrap_or(fallback) -> fallback
//! None.unwrap_or(fallback) -> fallback
//! unwrap_or_default方法，调用类型的DefaultTrait
//! Ok(T).unwrap_or_default() -> T
//! Some(T).unwrap_or_default() -> T
//! Err(E).unwrap_or_default() -> Default::default()
//! None.unwrap_or_default() -> Default::default()

//! Transform Ok or Some values only
//! map方法，仅对Ok、Some生效
//! Ok(T).map(|v| v.operation()) -> T.operation()
//! Some(T).map(|v| v.operation()) -> T.operation()
//! Err(E).map(|v| v.operation()) -> Err(E)
//! None.map(|v| v.operation()) -> None