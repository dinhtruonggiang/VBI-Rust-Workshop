//Function to check if an array is a subarray of another array
fn is_sub_array(org_arr: &[i32], sub_arr: &[i32]) -> bool {
    //Use 2 pointers to traverse 2 arrays
    let mut i = 0; //pointer for original array
    let mut j = 0;//pointer for sub array

    let m = org_arr.len();// length of original array
    let n = sub_arr.len();// length of sub array

    // Traverse both arrays
    while i < m && j < n {
        // if element matches -> increase both pointers
        if org_arr[i] == sub_arr[j] {
            i += 1;
            j += 1;

            if j == n {
                return true
            }
        } else {
            i = i - j + 1;
            j = 0;
        }
    }

    false
}

fn main() {

    let org_arr = [1, 2,3,5,6,8, 10, 11];
    let sub_arr = [6, 8,10];

    if is_sub_array(&org_arr, &sub_arr) {
        println!("Là mảng con!");
    } else {
        println!("Không là mảng con!");
    }    
}
