use std::collections::HashMap;
use std::sync::mpsc::Sender;
use std::thread;
use std::time::Duration;

use colorous::Color;

use crate::fast::{
    fsp::{FastExpReq, LEDState},
    system::InternalEvent,
};

#[derive(Debug, Clone)]
pub struct LedId {
    pub expansion_id: String,
    pub port: u8, // TODO?
    pub index: u8,
}

#[derive(Debug, Clone)]
pub struct LedAnimation {
    // Locations of which LEDs this animation applies to
    pub leds: Vec<LedId>,
    // A frame is list of colors. The index matches up to the index of `leds`
    pub frames: Vec<Vec<Color>>,
    pub fps: u8,
    pub repeat: u16, // 0 = infinite
}

pub fn run(exp_tx: Sender<FastExpReq>, main_tx: Sender<InternalEvent>, anim: LedAnimation) {
    let frame_duration = Duration::from_millis(1000 / u64::from(anim.fps));
    thread::spawn(move || {
        let mut run_count = 0;
        let mut current_frame = 0;

        while anim.repeat == 0 || run_count < anim.repeat {
            let frame = &anim.frames[current_frame];
            let mut grouped: HashMap<String, Vec<LEDState>> = HashMap::new();
            for (i, color) in frame.iter().enumerate() {
                let led_config = &anim.leds[i];

                if !grouped.contains_key(&led_config.expansion_id) {
                    grouped.insert(led_config.expansion_id.clone(), Vec::new());
                }

                let led_state = LEDState {
                    index: led_config.index,
                    r: color.r,
                    g: color.g,
                    b: color.b,
                };
                match grouped.get_mut(&led_config.expansion_id) {
                    Some(vec) => {
                        vec.push(led_state);
                    }
                    None => {
                        grouped.insert(led_config.expansion_id.clone(), vec![led_state]);
                    }
                }
            }

            for (address, states) in grouped {
                let _ = exp_tx.send(FastExpReq::SetLEDs { address, states });
            }

            current_frame += 1;
            if current_frame == anim.frames.len() {
                current_frame = 0;
                if anim.repeat > 0 {
                    run_count += 1;
                }
            }

            thread::sleep(frame_duration);
        }

        // TODO: turn off LEDs once done

        let _ = main_tx.send(InternalEvent::LedAnimationComplete);
    });
}
