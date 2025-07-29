pub fn f2024_01(list1: &mut [i32], list2: &mut [i32]) -> i32 {
  list1.sort();
  list2.sort();

  let len = std::cmp::min(list1.len(), list2.len());
  let mut result: i32 = 0;

  for i in 0..len {
    result += (list1[i] - list2[i]).abs();
  }

  result
}