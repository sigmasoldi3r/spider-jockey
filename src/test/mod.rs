use crate::parser::Contract;

#[test]
fn test() {
    let def: Contract = serde_json::from_str(include_str!("CompanyDao.json")).unwrap();
    println!("{:?}", def.abi);
    assert!(false)
}
