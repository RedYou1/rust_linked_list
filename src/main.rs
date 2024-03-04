use list::List;
use std::fmt::Debug;

pub mod list;

static mut AMOUNT: usize = 0;

#[derive(Clone)]
struct TestNode {
    pub value: i32,
}
impl Debug for TestNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.value.fmt(f)
    }
}
impl Drop for TestNode {
    fn drop(&mut self) {
        unsafe {
            AMOUNT += 1;
        }
    }
}
impl PartialEq<i32> for TestNode {
    fn eq(&self, other: &i32) -> bool {
        self.value == *other
    }
}

fn main() {
    let mut list = List::default();
    let mut vec = Vec::default();
    for i in 0..8 {
        list.push(TestNode { value: i * i });
        vec.push(i * i);
        assert_eq!(unsafe { AMOUNT }, 0, "push index:{i}");
        assert_eq!(list, List::from_iter(vec.clone()));
    }
    list.insert(4, TestNode { value: 16 });
    vec.insert(4, 16);
    println!("{list:?}");
    assert_eq!(unsafe { AMOUNT }, 0, "insert");
    assert_eq!(list, List::from_iter(vec.clone()));
    let mut list = list.clone();
    let a = list.get(3).expect("").value;
    let b = list.get(0).expect("").value;
    list.replace(3, TestNode { value: b });
    vec[3] = b;
    list.replace(0, TestNode { value: a });
    vec[0] = a;
    assert_eq!(unsafe { AMOUNT }, 2, "replace");
    assert_eq!(list, List::from_iter(vec.clone()));
    println!("{list:?}");
    let len = list.len();
    for i in (0..len).rev() {
        list.remove(i);
        vec.remove(i);
        assert_eq!(unsafe { AMOUNT }, len - i + 2, "remove index:{i}");
        assert_eq!(list, List::from_iter(vec.clone()));
    }
    println!("{list:?}");

    let vec = ["ABC", "UFA"];
    let list = List::from_iter(vec);
    println!("{list:?}");
    let vec = vec!["ABC", "UFA"];
    let list = List::from_iter(vec);
    println!("{list:?}");
}
