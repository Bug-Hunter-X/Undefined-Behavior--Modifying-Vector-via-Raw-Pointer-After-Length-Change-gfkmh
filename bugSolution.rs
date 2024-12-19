fn main() {
    let mut v = vec![1, 2, 3];
    let mut new_value = 10; 
    v[0] = new_value; //Using safe method to assign the new value 
    println!( "{:?}", v);
}