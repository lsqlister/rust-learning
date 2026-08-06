fn greet_world() {
    let southern_germany = "Grüß Gott!";
    let chinese = "世界，你好";
    let english = "World, hello";
    let regions = [southern_germany, chinese, english];
    //for region in regions.iter() {
    for region in regions {//简写，在 2021 edition 及以后
        println!("{}", &region);
    }
}

fn main() {
    greet_world();
}