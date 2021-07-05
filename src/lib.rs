pub mod m2macro;

fn main(){
    print!("hello");
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}