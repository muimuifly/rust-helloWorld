fn main() {
    greet_world();
}

fn greet_world() {
    let southern_germany = "Gruß Gott";
    let chinese = "世界，你好";
    let english = "World,hello";
    let regions = [southern_germany,chinese,english];
    for region in regions.iter(){
        println!("{}",&region);
    }
}