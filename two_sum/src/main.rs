fn main() {
    let nums = vec![3,2,4];
    let target = 6;
    let mut output:Vec<i32> = Vec::new();


    for i in 0..nums.len() {
        for j in i+1..nums.len(){
            if nums[j] == target - nums[i]{
                output.push(i as i32);
                output.push(j as i32) ;
            }

        }
    }
    println!("{:?}", output)
}
