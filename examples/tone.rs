extern crate bela_sys;

use bela_sys::{BelaInitSettings, BelaContext};
use std::{mem, slice, thread, time};
use std::os::raw::c_void;
use std::mem::transmute;
use std::f32::consts::PI;

pub unsafe extern
fn render(ctx: *mut BelaContext, user_data: *mut std::os::raw::c_void) {
  let len = (*ctx).audioFrames;
  let channels = (*ctx).audioOutChannels;
  let rate = (*ctx).audioSampleRate;

  let out: &mut [f32] = slice::from_raw_parts_mut((*ctx).audioOut as *mut f32, (len * channels) as usize);
  let phase = transmute::<*mut c_void, *mut f32>(user_data);

  let phase_incr = (2. * PI) * 440. / rate;

  for s in out {
    *s = (*phase).sin();
    *phase += phase_incr;
    if *phase > 2.0 * PI {
      *phase -= 2.0 * PI;
    }
  }
}

fn main() {
  unsafe {
    let mut settings: BelaInitSettings = mem::uninitialized();
    bela_sys::Bela_defaultSettings(&mut settings);
    settings.render = Some(render);

    let mut phase: f32 = 0.0;
    let user_data = transmute::<*mut f32, *mut c_void>(&mut phase);

    if bela_sys::Bela_initAudio(&mut settings, user_data) != 0 {
      panic!("init audio err");
    }

    if bela_sys::Bela_startAudio() != 0 {
      panic!("start audio err");
    }

    while bela_sys::gShouldStop == 0 {
      thread::sleep(time::Duration::from_millis(10));
    }

    bela_sys::Bela_stopAudio();
    bela_sys::Bela_cleanupAudio();
  }
}
