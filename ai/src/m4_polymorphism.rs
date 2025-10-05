use ethers::types::Address;
use std::str::FromStr;


trait EthereumAddress{
    fn convert_address(&self)->Result<Address,&'static str>;

}

impl EthereumAddress for &str {
    fn convert_address(&self)->Result<Address,&'static str>{
        match Address::from_str(self) {
            Ok(a)=>Ok(a),
            Err(_e)=>Err("Address not convertible to Ethereum Address")
        }
    }
}

impl EthereumAddress for Address{
    fn convert_address(&self)->Result<Address,&'static str>{
        Ok(*self)
    }
}

fn get_ethereum_address<T:EthereumAddress>(address:T)->Address {
    let converted = match address.convert_address() {
        Ok(x) => x,
        Err(_) => todo!(),
    };
    converted
}


#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn convert_address_test(){
        let test_address:Address=Address ::from_str("0x22eEC85ba6a5cD97eAd4728eA1c69e1D9c6fa778").unwrap();

        let new_address=get_ethereum_address(test_address);
        dbg!(new_address);
        assert_eq!(new_address,Address::from_str("0x22eEC85ba6a5cD97eAd4728eA1c69e1D9c6fa778").unwrap());
    }
}