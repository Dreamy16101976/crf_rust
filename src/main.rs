//Cosmic Ray Finder (Rust version)
//v.0.0.3
//(C) 2024 Alexey "FoxyLab" Voronin
//https://acdc.foxylab.com

/*
Whats new:
v.0.0.1 - first version
v.0.0.2 - added sealed camera lens test
v.0.0.3 - added camera select & camera test
*/

extern crate camera_capture;
extern crate chrono;
extern crate image;

use chrono::{DateTime, Local};
use std::fs::File;
use std::path::Path;
use std::time::SystemTime;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::env;

const SEALED_LIMIT: f32 = 70.0; //limit for sealed camera lens
const LIMIT: u8 = 150; //limit for color channel for event
const CNT_MAX: u16 = 1000; //number of frames for speed calc
const FRAMERATE: f64 = 30.0;

fn main() {
    println!("Cosmic Ray Finder (Rust version) v.0.0.3");
    println!("(C) 2024 Alexey \"FoxyLab\" Voronin");
    println!("https://acdc.foxylab.com");
    //camera select
    let cam_idx: u32;
    let args: Vec<String> = env::args().collect();
    if args.len()>1 {
        cam_idx = args[1].parse::<u32>().unwrap();
        println!("Camera {} selected", cam_idx);
    } else {
        cam_idx = 0;
        println!("Camera 0 selected by default");
    }
    
    
    //camera test
    let cam1 = camera_capture::create(cam_idx);
        if let Err(_) = cam1 {
            println!("Could not open camera {}! Exited...", cam_idx);
            std::process::exit(0);
        }
    println!("Camera {} has been successfully opened", cam_idx);

    let cam2 = cam1.unwrap().fps(FRAMERATE).unwrap().start();
        if let Err(_) = cam2 {
            println!("Could retrieve data from camera {}! Exited...", cam_idx);
            std::process::exit(0);
        }
    println!("Camera {} has been successfully configured", cam_idx);
    let mut cam = cam2.unwrap();
    
    let img = cam.next().unwrap();
    let img_width = img.dimensions().0;
    let img_height = img.dimensions().1;
    println!("WIDTH: {}", img_width);
    println!("HEIGHT: {}", img_height);

    let mut max;
    let mut red;
    let mut green;
    let mut blue;
    let mut flag;
    let mut red_evt;
    let mut green_evt;
    let mut blue_evt;
    let mut distance: f32;
    let mut check = true;
    
    let kill = Arc::new(AtomicBool::new(false));
    let kill_cloned = kill.clone();

    ctrlc::set_handler(move || {
        println!("");
        println!("CTRL-C pressed...");
        kill_cloned.store(true, Ordering::SeqCst);
    }).expect("Error setting Ctrl-C handler");

    let mut start = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).expect("");
    let mut stop;
    let mut cnt: u16 = 0;
    loop {
        if kill.load(Ordering::SeqCst) == true {
            println!("Killed...");
            break;
        }
        let img = cam.next().unwrap();
        max = 0.0;
        if check {
            println!("Sealed camera lens check...");
            //sealed cam test
            for pixel in img.pixels() { //loop for all pixels
                //color channels for current pixel get
                red = pixel[0];
                green = pixel[1];
                blue = pixel[2];
                //color distance calc
                distance =
                    (red as f32 * red as f32 + green as f32 * green as f32 + blue as f32 * blue as f32)
                        .sqrt();
                if distance > max {
                    max = distance;
                }
            }
            println!("MAX: {}", max as u32);
            if max > SEALED_LIMIT {
                println!("Seal the camera lens from light and try again!");
                let test_path = Path::new("check.png");
                let _ = &mut File::create(&test_path).unwrap();
                img.save(&test_path).unwrap();
                println!("Test frame saved to check.png");
                std::process::exit(0);
            }
            check = false;
            println!("O.K.");
            println!("Capture start...");
            start = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).expect("");
            cnt = 0;
        }
        flag = false;
        red_evt = 0;
        green_evt = 0;
        blue_evt = 0;
        for pixel in img.pixels() { //loop for all pixels
            //color channels for current pixel get
            red = pixel[0];
            green = pixel[1];
            blue = pixel[2];
            //color distance calc
            distance =
                (red as f32 * red as f32 + green as f32 * green as f32 + blue as f32 * blue as f32)
                    .sqrt();
            if distance > max {
                max = distance;
                flag = false;
                if red > LIMIT { //event detected
                    flag = true;
                }
                if green > LIMIT { //event detected
                    flag = true;
                }
                if blue > LIMIT { //event detected
                    flag = true;
                }
                if flag == true { //color channels for event save
                    red_evt = red;
                    green_evt = green;
                    blue_evt = blue;
                }
            }
        }

        if flag == true {
            println!("EVENT!");
            println!("R: {} G: {} B: {}", red_evt, green_evt, blue_evt);
            let now: DateTime<Local> = Local::now();
            let filename = now.format("%Y%m%d%H%M%S%3f.png");
            let stamp = now.format("%Y/%m/%d %H:%M:%S.%3f");
            println!("TIME: {}", stamp);
            let file_name = filename.to_string();
            let path = Path::new(&file_name);
            let _ = &mut File::create(&path).unwrap();
            img.save(&path).unwrap();
            println!("{}", file_name);
            cnt = 0;
            start = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .expect("");
        } else {
            cnt = cnt + 1;
            if cnt == CNT_MAX {
                stop = SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .expect("");
                let diff = stop.as_secs() - start.as_secs();
                println!("SPEED: {}", (CNT_MAX as f32 / diff as f32 * 60.0) as u16);
                cnt = 0;
                start = SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .expect("");
            }
        }
    }
}
