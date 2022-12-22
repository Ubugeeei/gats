// Heigh Kind
trait Mappable<A> {
    type T<__>;
    fn map<B, F>(self, f: F) -> Self::T<B>
    where
        F: Fn(A) -> B;
}

// First Order 1
struct MyVec<A>(Vec<A>);
impl<A> Mappable<A> for MyVec<A> {
    type T<__> = MyVec<__>;
    fn map<B, F>(self, f: F) -> Self::T<B>
    where
        F: Fn(A) -> B,
    {
        let MyVec(vec) = self;
        let mut mapped = Vec::with_capacity(vec.len());
        for item in vec {
            mapped.push(f(item));
        }
        MyVec(mapped)
    }
}
// Proper 1 of First Order 1
type MyNumbers = MyVec<i32>;
// Proper 2 of First Order 1
type MyStrings = MyVec<String>;

// First Order 2
struct MyMap<A>(std::collections::HashMap<String, A>);
impl<A> Mappable<A> for MyMap<A> {
    type T<__> = MyMap<__>;
    fn map<B, F>(self, f: F) -> Self::T<B>
    where
        F: Fn(A) -> B,
    {
        let MyMap(map) = self;
        let mut mapped = std::collections::HashMap::with_capacity(map.len());
        for (key, item) in map {
            mapped.insert(key, f(item));
        }
        MyMap(mapped)
    }
}
impl<Item> MyMap<Item> {
    fn insert(&mut self, key: String, value: Item) {
        self.0.insert(key, value);
    }
}
// Proper 1 of First Order 2
type MyMapNumbers = MyMap<i32>;
// Proper 2 of First Order 2
type MyMapStrings = MyMap<String>;

fn main() {
    // ①
    let my_vec: MyNumbers = MyVec(vec![1, 2, 3]);
    let _mapped: MyNumbers = my_vec.map(|x| x + 1); // map

    // ②
    let my_vec: MyStrings = MyVec(vec![String::from("one"), String::from("two")]);
    let _mapped: MyStrings = my_vec.map(|x| x + "1"); // map

    // ③
    let mut my_map: MyMapNumbers = MyMap(std::collections::HashMap::new());
    my_map.insert(String::from("one"), 1);
    my_map.insert(String::from("two"), 2);
    let _mapped: MyMapNumbers = my_map.map(|x| x + 1); // map

    // ④
    let mut my_map: MyMapStrings = MyMap(std::collections::HashMap::new());
    my_map.insert(String::from("one"), String::from("one"));
    my_map.insert(String::from("two"), String::from("two"));
    let _mapped: MyMapStrings = my_map.map(|x| x + "1"); // map
}
