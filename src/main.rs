fn main() {

    let k = 6;

    println!( "{}", k.to_string() );
    println!( "{}", test(k).to_string() );
    println!( "{}", k.to_string() );
}


fn test( mut var: u64 ) -> u64 {
    let var2: u64 = 11;
    var = 21;
    println!( "{}", var.to_string() );
    var2
}

#[test]
fn test_test() {
    assert_eq!( test(5), 11 );
}