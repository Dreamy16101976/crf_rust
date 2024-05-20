# crf_rust
<i>crf_rust</i> - implementation of the programming language Rust registration procedures falling particles of cosmic radiation (cosmic rays) in the digital camera sensor (<i>Cosmic Ray Finder</i>).<br/><br/>
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
./crf_rust
```

or 

```
cargo run
```

<br>
By default, the camera with index 0 is selected for capturing frames. You can change the active camera by specifying its index on the command line, for example:

```
./crf_rust 1
```

<br>
When the program starts (and every 10,000 frames), calibration is performed to determine the limit when an event is detected:<br>

```
Calibration...
MAX: 60 58 55 56 58 56 52 65 62 54 62 62 62 54 54 60 64 59 60 68 55 63 59 62 57 57 59 53 61 55
Camera has been successfully calibrated
LIMIT: 89
```
<br>

Frames from the camera is read in a loop and it is determined whether for the pixel with the largest color distance, the level of any color channel exceeds the specified limit.<br>
if so, the event is logged and the frame is saved in PNG-file <i>YYYYMMDDHHMMSSmmm.png</i>.<br>

```
EVENT!
R: 207 G: 231 B: 179
TIME: 2024/05/14 18:59:21.842
20240514185921842.png
```

You can view the particle traces registered using this program in the folder <i>events</i>.<br>
Press Ctrl-C to exit from the program.<br><br>

<i>v4l2-ctl</i> is a utility allowing to control the camera subsystem.<br>
You can install v4l2-ctl by running the following command:
```
sudo apt install v4l-utils
```
You may use the following command to check which devices are connected to your computer at the moment:

```
v4l2-ctl --list-devices
```
Result:<br>
```
Vimicro USB Camera (Altair): Vi (usb-0000:00:14.0-4.2):
	/dev/video2
	/dev/video3
	/dev/media1

USB2.0 VGA UVC WebCam: USB2.0 V (usb-0000:00:14.0-5):
	/dev/video0
	/dev/video1
	/dev/media0
```
To increase the frame capture speed, you can use the following commands (in <i>d 0</i> 0 is selected camera index (0 - for /dev/video0, 2 - for /dev/video2)):<br>
```
v4l2-ctl -d 0 -c auto_exposure=1
v4l2-ctl -d 0 -c exposure_time_absolute=250
```
In my case, this increased the capture speed from 451 to 895..923 for UVC cam and from 487 to 895..937 for Vimicro cam.<br>

I also experimented with increasing the exposure time for Vimicro cam:

```
v4l2-ctl -d 0 -c auto_exposure=1
v4l2-ctl -d 0 -c exposure_time_absolute=16384
```

In this case, the capture speed stabilizes at the level 300-301.<br>


## Notes:
This can surely be optimized. 

## Related publications

Polatoğlu, A., & Yeşilyaprak, C. (2023). Using and Testing Camera Sensors with Different Devices at Cosmic Ray Detection. Erzincan University Journal of Science and Technology, 16(2), 590-597. [PDF download](https://dergipark.org.tr/en/download/article-file/2616216)

DIY Cosmic Ray Detector Using a Webcam and Lead Shielding [YouTube](https://youtu.be/k-Nxso1DdhA?feature=shared)

## Alternative projects

[WebCam-Cosmic-Rays](https://github.com/Naimish240/WebCam-Cosmic-Rays)



