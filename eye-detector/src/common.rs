use opencv::{core, imgproc, objdetect, prelude::*};

pub fn prepare_frame(frame: &Mat) -> opencv::Result<Mat> {
    let mut gray = Mat::default();
    let mut equalized = Mat::default();
    imgproc::cvt_color(&frame, &mut gray, imgproc::COLOR_BGR2GRAY, 0)?;
    imgproc::equalize_hist(&gray, &mut equalized)?;
    Ok(equalized)
}

pub fn detect_faces(
    frame: &Mat,
    face_detector: &mut objdetect::CascadeClassifier,
) -> opencv::Result<core::Vector<core::Rect>> {
    let mut faces = core::Vector::<core::Rect>::new();
    face_detector.detect_multi_scale(
        &frame,
        &mut faces,
        1.01,
        40,
        0,
        core::Size {
            width: frame.size()?.width * 0.06 as i32,
            height: frame.size()?.height * 0.06 as i32,
        },
        core::Size {
            width: frame.size()?.width * 0.18 as i32,
            height: frame.size()?.height * 0.18 as i32,
        },
    )?;
    Ok(faces)
}

pub fn detect_eyes(
    frame: &Mat,
    eye_detector: &mut objdetect::CascadeClassifier,
) -> opencv::Result<core::Vector<core::Rect>> {
    let mut eyes = core::Vector::<core::Rect>::new();
    eye_detector.detect_multi_scale(
        &frame,
        &mut eyes,
        1.01,
        40,
        objdetect::CASCADE_SCALE_IMAGE,
        core::Size {
            width: frame.size()?.width * 0.06 as i32,
            height: frame.size()?.height * 0.06 as i32,
        },
        core::Size {
            width: frame.size()?.width * 0.18 as i32,
            height: frame.size()?.height * 0.18 as i32,
        },
    )?;
    Ok(eyes)
}

pub fn draw_in_frame(
    frame: &mut Mat,
    eyes: &core::Vector<core::Rect>,
    face: &core::Rect,
) -> opencv::Result<()> {
    // Draw face :)
    let scaled_face = core::Rect {
        x: face.x,
        y: face.y,
        width: face.width,
        height: face.height,
    };
    imgproc::rectangle(
        frame,
        scaled_face,
        core::Scalar::new(0f64, 0f64, 255f64, 0f64), //color
        1,                                           // thickness
        8,                                           // line type
        0,                                           // shift
    )?;
    // Draw eyes
    if eyes.len() == 2 {
        // Normally, people have 2 eyes
        for eye in eyes.iter() {
            imgproc::rectangle(
                frame,
                core::Rect::new(
                    face.tl().x + eye.tl().x,
                    face.tl().y + eye.tl().y,
                    eye.width,
                    eye.height,
                ), // eye
                core::Scalar::new(0f64, 0f64, 255f64, 0f64), // color
                1,                                           // thickness
                8,                                           // line type
                0,                                           // shift
            )?;
        }
    }
    Ok(())
}

// pub fn detect_faces(
//     frame: &Mat,
//     face_detector: &mut objdetect::CascadeClassifier,
// ) -> opencv::Result<core::Vector<core::Rect>> {
//     let mut faces = core::Vector::new();
//     face_detector.detect_multi_scale(
//         &frame,
//         &mut faces,
//         1.01,   // scale factor
//         3,       // min neighbors (原40改为新版本推荐的3)
//         objdetect::CASCADE_SCALE_IMAGE, // 添加flags参数[1](@ref)
//         core::Size::new(
//             (frame.size()?.width as f32 * 0.06) as i32,
//             (frame.size()?.height as f32 * 0.06) as i32,
//         ),
//         core::Size::new(
//             (frame.size()?.width as f32 * 0.18) as i32,
//             (frame.size()?.height as f32 * 0.18) as i32,
//         ),
//     )?;
//     Ok(faces)
// }

// pub fn detect_eyes(
//     frame: &Mat,
//     eye_detector: &mut objdetect::CascadeClassifier,
// ) -> opencv::Result<core::Vector<core::Rect>> {
//     let mut eyes = core::Vector::new();
//     eye_detector.detect_multi_scale(
//         &frame,
//         &mut eyes,
//         1.01,
//         3,      // min neighbors调整
//         objdetect::CASCADE_SCALE_IMAGE,
//         core::Size::new(
//             (frame.size()?.width as f32 * 0.06) as i32,
//             (frame.size()?.height as f32 * 0.06) as i32,
//         ),
//         core::Size::new(
//             (frame.size()?.width as f32 * 0.18) as i32,
//             (frame.size()?.height as f32 * 0.18) as i32,
//         ),
//     )?;
//     Ok(eyes)
// }

// pub fn draw_in_frame(
//     frame: &mut Mat,
//     eyes: &core::Vector<core::Rect>,
//     face: &core::Rect,
// ) -> opencv::Result<()> {
//     // 绘制人脸
//     imgproc::rectangle(
//         frame,
//         *face,
//         core::Scalar::new(0.0, 0.0, 255.0, 0.0),
//         2,  // 加粗线宽
//         imgproc::LINE_8,
//         0,
//     )?;

//     // 绘制眼睛（优化坐标计算）
//     if eyes.len() == 2 {
//         for eye in eyes {
//             let eye_pos = core::Rect::new(
//                 face.x + eye.x,
//                 face.y + eye.y,
//                 eye.width,
//                 eye.height,
//             );
//             imgproc::rectangle(
//                 frame,
//                 eye_pos,
//                 core::Scalar::new(0.0, 255.0, 0.0, 0.0), // 改为绿色
//                 1,
//                 imgproc::LINE_8,
//                 0,
//             )?;
//         }
//     }
//     Ok(())
// }
