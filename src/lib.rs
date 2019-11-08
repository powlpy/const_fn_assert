
#![no_std]
#![doc(html_root_url = "https://docs.rs/const_fn_assert")]

#![forbid(const_err)]

#[doc(hidden)]
#[allow(dead_code)]
pub const ASSERT: [(); 1] = [()];

#[macro_export]
macro_rules! cfn_assert {
    ($x:expr $(,)?) => {
        let _ = $crate::ASSERT[!{let cond: bool = $x; cond} as usize];
    }
}

#[macro_export]
macro_rules! cfn_assert_eq {
    ($x:expr, $y:expr $(,)?) => {
        $crate::cfn_assert!($x == $y)
    }
}

#[macro_export]
macro_rules! cfn_assert_ne {
    ($x:expr, $y:expr $(,)?) => {
        $crate::cfn_assert!($x != $y)
    }
}

#[cfg(test)]
mod tests{

    #[test]
    fn test_assert(){
        cfn_assert!(true);
        cfn_assert!(true && (true != false));
        cfn_assert!((true && true) != false);
        cfn_assert_eq!(false, false);
    }
    
    const fn sub_fn(x: u8) -> u8 {
        cfn_assert!(x < 5);
        x + 1
    }
    
    const _TEST_ASSERT: u8 = sub_fn(1);
    
    // _TEST_ASSERT_FAIL is compile-fail
    
    #[test]
    fn test_sub_fn_assert(){
        let _ = sub_fn(1);
    }
    
    #[test]
    #[should_panic]
    fn test_sub_fn_assert_fail(){
        let _ = sub_fn(6);
    }
    
    #[test]
    const fn test_const_assert(){
        const FIVE: usize = 5;

        cfn_assert!(FIVE * 2 == 10);
        cfn_assert!(FIVE > 2);
    }
    
    #[test]
    fn test_const_fn_assert(){
        const TEST_CONST_ASSERT: () = test_const_assert();
        let _ = TEST_CONST_ASSERT;
    }
}
