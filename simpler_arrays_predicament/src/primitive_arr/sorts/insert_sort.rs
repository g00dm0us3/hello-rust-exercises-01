pub fn insert_sort(arr: &mut [i32], sz: usize) {
    if sz == 0 || sz == 1 {
        return;
    } 
    
    for i in 1..sz {
        let mut j = i;

        // exchg two nearby items not in order
        while j > 0 && arr[j-1] > arr[j] {
            let t = arr[j];
            arr[j] = arr[j-1];
            arr[j-1] = t;
            j -= 1;
        }
    }
}