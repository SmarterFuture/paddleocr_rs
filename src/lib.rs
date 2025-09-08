mod det;
mod error;

pub use error::PaddleOcrError;
pub use det::Det;

#[cfg(test)]
mod tests {
    use crate::det::Det;

    #[test]
    fn it_works() -> Result<(), Box<dyn std::error::Error>> {
        let mut det = Det::from_file("./models/ch_PP-OCRv4_det_infer.onnx")?;
        // let mut det = Det::from_file("./models/ch_PP-OCRv5_mobile_det.onnx")?;
        println!("peter pan");
        let img = image::open("./test/tmp.protokol5_1.png")?;
        let a = det.find_text_rect(&img)?;
        println!("{}", a.len());
        // for sub in det.find_text_rect(&img)? {
        //     println!("{:?}", sub);
        // }
        Ok(())
    }
}
