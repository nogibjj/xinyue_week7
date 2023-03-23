use image::{ImageBuffer, Rgba};
//use image::ImageOutputFormat;
fn main() {
    // 创建一个新的图像，大小为 300x300，背景为白色
    let mut img = ImageBuffer::from_pixel(300, 300, Rgba([255, 255, 255, 255]));

    // 绘制一个红色的大圆
    let radius = 100;
    let center = (150, 150);
    for x in 0..300 {
        for y in 0..300 {
            let distance_squared = ((x as i32 - center.0).pow(2) + (y as i32 - center.1).pow(2)) as f64;
            if distance_squared <= (radius as f64).powi(2) {
                img.put_pixel(x, y, Rgba([255, 0, 0, 255]));
            }
        }
    }

    // 绘制一个绿色的小圆
    let radius = 50;
    let center = (150, 150);
    for x in 100..200 {
        for y in 100..200 {
            let distance_squared = ((x as i32 - center.0).pow(2) + (y as i32 - center.1).pow(2)) as f64;
            if distance_squared <= (radius as f64).powi(2) {
                img.put_pixel(x, y, Rgba([0, 255, 0, 255]));
            }
        }
    }

    // 绘制一个蓝色的直线
    for x in 0..300 {
        let y = 150;
        img.put_pixel(x, y, Rgba([0, 0, 255, 255]));
    }

    // 保存图像为 PNG 文件
    //img.save_with_format("output.png", ImageOutputFormat::Png).unwrap();
}
