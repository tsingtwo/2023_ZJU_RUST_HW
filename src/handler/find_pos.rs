pub fn find_pos(a:Vec<String>) -> i32{
    let mut x = 1;
    let len = a.len();
    while x < len{
        if a[x] == "114514" {
            break;
        }
        x += 1;
    }
    return x as i32
}