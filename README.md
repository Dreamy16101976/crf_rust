# crf_rust
<i>crf_rust</i> - implementation of the programming language Rust registration procedures falling particles of cosmic radiation in the digital camera sensor (<i>Cosmic Ray Finder</i>).<br/><br/>
Copyright (C) 2024 <i>Alexey "FoxyLab" Voronin</i><br/>
<i>Email</i>:    support@foxylab.com<br/>
<i>Website</i>:  https://acdc.foxylab.com<br/>
This software is licensed under the GPL v3.0 License.<br/><br/>

## Description
Detailed description (in russian) - https://acdc.foxylab.com/node/43.

## Building
[Building crf_rust](BUILDING.md)

## Using
The webcam is placed in an opaque container (or the laptop camera window is sealed with several layers of insulating tape).<br>
Run program: 
```
<i>crf_rust</i>  
```
or 
```
cargo run
```
<br>
Frames from the camera is read in a loop and it is determined whether for the pixel with the largest color distance, the level of any color channel exceeds the specified limit..<br>
if so, the event is logged and the frame is saved in PNG-file.<br>
Press Ctrl-C to exit from the program.

## Notes:
This can surely be optimized. 

## Related publications

Polatoğlu, A., & Yeşilyaprak, C. (2023). Using and Testing Camera Sensors with Different Devices at Cosmic Ray Detection. Erzincan University Journal of Science and Technology, 16(2), 590-597. [PDF download](https://dergipark.org.tr/en/download/article-file/2616216)

DIY Cosmic Ray Detector Using a Webcam and Lead Shielding [YouTube](https://youtu.be/k-Nxso1DdhA?feature=shared)
