
pub fn gcd_non_recurssion(a:u32,b:u32)->u32{
    let mut _gcd = 0;
    let mut _temp = 0;
    let mut _a = a;
    let mut _b = b;
    while _b > 0 {
        _temp = _a % _b;
        if _temp >0 {
            _gcd = _temp;
            _a = _b;
            _b = _gcd;
        }
        else{
            break
        }
    }
    _gcd
}

pub fn gcd_recur(a:u32,b:u32)-> u32 {
    if b > 0{
        gcd_recur(b, a % b )
    }else {
        a
    }
}