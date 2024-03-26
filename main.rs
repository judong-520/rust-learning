/// 固定类型（比如i32）的数组排序
pub fn bubble_sort(arr: &mut [i32]) {
    let len = arr.len();
    for i in 0..len.saturating_sub(1) {
        for j in 0..(len - 1 - i) {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}


/// 使用范型和PartialOrd实现对任意类型的排序
fn bubble_sort_pro<T: PartialOrd + Copy>(list: &mut Vec<T>) {
    for i in 0..list.len() {
        for x in 0..list.len() - 1 {
            //实际交换次数等于 n-1
            if list[x] > list[x + 1] {
                list.swap(x, x + 1); //元素交换位置
            }
        }
    }
}



fn main() {
    let mut nums = [1, 34, 25, 12, 22, 101, 90];
    bubble_sort(&mut nums);
    println!("{:?} ", nums);

    let mut list = vec![1, 354, 5560, 200, 34, 51, 25, 100, 65];
    bubble_sort(&mut list);
    println!("{:?} ", list);

    let mut list = vec!['D', 'e', 'A', 'C', 'a', 'W'];
    bubble_sort_pro(&mut list);
    println!("{:?} ", list);
}







