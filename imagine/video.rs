/*******************************************************************************
  video.rs
********************************************************************************
  Copyright 2023 Menelik Eyasu

  Licensed under the Apache License, Version 2.0 (the "License");
  you may not use this file except in compliance with the License.
  You may obtain a copy of the License at

      http://www.apache.org/licenses/LICENSE-2.0

  Unless required by applicable law or agreed to in writing, software
  distributed under the License is distributed on an "AS IS" BASIS,
  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
  See the License for the specific language governing permissions and
  limitations under the License.
*******************************************************************************/

extern crate ffmpeg_sys_next as ffmpeg;

use ffmpeg::*;
use core::ffi::CStr;
use pyo3::prelude::*;
use std::ffi::CString;
use core::ffi::c_void;

// use std::path::Path;

#[inline(always)]
#[allow(non_snake_case)]
pub const fn FFMPEG_AVERROR(e: std::os::raw::c_int) -> std::os::raw::c_int {
  -e
}

pub struct Video {
  pub writing: bool,

  pts: i64,
  container: *mut AVFormatContext,

  image_stream: *mut AVStream,
  image_context: *mut AVCodecContext,
  image_packet: *mut AVPacket,
  image_buffer: *mut AVFrame,

  audio_stream: *mut AVStream,
  audio_context: *mut AVCodecContext,
  audio_packet: *mut AVPacket,
  audio_buffer: *mut AVFrame
}

unsafe impl Send for Video {}

impl Video {
  pub fn new() -> Self {
    // for codec in Self::get_available_codecs().iter() {
    //   info!(codec);
    // }

    Self {
      writing: false,
      pts: 0,
      container: std::ptr::null_mut(),
      image_stream: std::ptr::null_mut(),
      image_context: std::ptr::null_mut(),
      image_packet: std::ptr::null_mut(),
      image_buffer: std::ptr::null_mut(),
      audio_stream: std::ptr::null_mut(),
      audio_context: std::ptr::null_mut(),
      audio_packet: std::ptr::null_mut(),
      audio_buffer: std::ptr::null_mut()
    }
  }

  pub fn get_width(&self) -> Option<i32> {
    unsafe {
      if self.image_context.is_null() { None }
      else { Some((*self.image_context).width) }
    }
  }

  pub fn get_height(&self) -> Option<i32> {
    unsafe {
      if self.image_context.is_null() { None }
      else { Some((*self.image_context).height) }
    }
  }

  pub fn get_fps(&self) -> Option<i32> {
    unsafe {
      if self.image_context.is_null() { None }
      else { Some((*self.image_context).time_base.den) }
    }
  }

  pub fn make(
    &mut self, 
    filename: &str,
    fps: i32,
    width: i32,
    height: i32,
    // bitrate: usize,
  ) {
    self.writing = true;

    // let format = CString::new("mp4").unwrap();
    let path = CString::new(filename).unwrap();
    unsafe {
      avformat_alloc_output_context2(
        &mut self.container,
        std::ptr::null_mut(),
        std::ptr::null_mut(), // format.as_ptr()
        path.as_ptr()
      );
  
      let video_codec = avcodec_find_encoder(AVCodecID::AV_CODEC_ID_MPEG4);
      self.image_stream = avformat_new_stream(
        self.container,
        std::ptr::null_mut()
      );
      self.image_context = avcodec_alloc_context3(video_codec);
  
      let time_base = AVRational { num: 1, den: fps };
      let frame_rate = AVRational { num: fps, den: 1 };
      (*self.image_stream).id = ((*self.container).nb_streams-1) as i32;
      (*self.image_stream).time_base = time_base;
      (*self.image_stream).r_frame_rate = frame_rate;
      (*self.image_stream).avg_frame_rate = frame_rate;
      (*self.image_context).codec_id = AVCodecID::AV_CODEC_ID_MPEG4;
      (*self.image_context).gop_size = 12;
      (*self.image_context).width = width;
      (*self.image_context).height = height;
      // (*self.image_context).bit_rate = 400000;
      (*self.image_context).time_base = time_base;
      (*self.image_context).framerate = frame_rate;
      (*self.image_context).pix_fmt = AVPixelFormat::AV_PIX_FMT_YUV420P;
      if (*(*self.container).oformat).flags & AVFMT_GLOBALHEADER != 0 {
        (*self.image_context).flags |= AV_CODEC_FLAG_GLOBAL_HEADER as i32;
      }

      self.image_packet = av_packet_alloc();
      self.image_buffer = av_frame_alloc();
      (*self.image_buffer).format = AVPixelFormat::AV_PIX_FMT_YUV420P as i32;
      (*self.image_buffer).width = width;
      (*self.image_buffer).height = height;
      av_frame_get_buffer(self.image_buffer, 0);
  
      let options: *mut *mut AVDictionary = &mut std::ptr::null_mut();
      avcodec_open2(self.image_context, video_codec, options);
      avcodec_parameters_from_context(
        (*self.image_stream).codecpar,
        self.image_context
      );

      // let audio_codec = avcodec_find_encoder(AVCodecID::AV_CODEC_ID_AAC);
      // self.audio_stream = avformat_new_stream(container, std::ptr::null_mut());
      // self.audio_context = avcodec_alloc_context3(audio_codec);
      // (*self.audio_context).sample_fmt = AVSampleFormat::AV_SAMPLE_FMT_FLTP;
      // (*self.audio_context).bit_rate = 64000;
      // (*self.audio_context).sample_rate = 44100;
      // self.audio_packet = av_packet_alloc();
  
      avio_open(&mut (*self.container).pb, path.as_ptr(), AVIO_FLAG_WRITE);
      avformat_write_header(self.container, std::ptr::null_mut());
    }
  }

  // #[pyo3(signature = (t=1))]
  pub fn write(&mut self, frame: Vec<u8>, t: i32) {
    unsafe {
      let width = (*self.image_buffer).width as usize;
      let height = (*self.image_buffer).height as usize;
      let frame_size = height * (*self.image_buffer).linesize[0] as usize + width;

      let y_pixels = std::slice::from_raw_parts_mut(
        (*self.image_buffer).data[0], frame_size
      );
      let cb_pixels = std::slice::from_raw_parts_mut(
        (*self.image_buffer).data[1], frame_size / 2
      );
      let cr_pixels = std::slice::from_raw_parts_mut(
        (*self.image_buffer).data[2], frame_size / 2
      );

      for y in 0..height {
        for x in 0..width {
          let start = 4 * (y * width + x);
          let r = frame[start] as i32;
          let g = frame[start+1] as i32;
          let b = frame[start+2] as i32;

          y_pixels[y * (*self.image_buffer).linesize[0] as usize + x] =
              (16 + (66 * r + 129 * g + 25 * b) >> 8) as u8;

          if y % 2 == 0 && x % 2 == 0 {
            let x = x / 2;
            let y = y / 2;

            cb_pixels[y * (*self.image_buffer).linesize[1] as usize + x] =
                (128 + (-38 * r - 74 * g + 112 * b >> 8)) as u8;
            cr_pixels[y * (*self.image_buffer).linesize[2] as usize + x] =
                (128 + (112 * r - 94 * g - 18 * b >> 8)) as u8;
          }
        }
      }

      if t > 0 {
        for _ in 0..t {
          (*self.image_buffer).pts = self.pts;
          self.encode(
            self.image_stream,
            self.image_context,
            self.image_packet,
            self.image_buffer
          );
          self.pts += 1;
        }
      }
    }
  }

  unsafe fn encode(
    &mut self,
    stream: *mut AVStream,
    encoder: *mut AVCodecContext,
    packet: *mut AVPacket,
    frame: *const AVFrame
  ) {
  // ) -> Result<()> {
    let mut status = avcodec_send_frame(encoder, frame);
    // if status < 0 {}
    while status >= 0 {
      status = avcodec_receive_packet(encoder, packet);
      if status == AVERROR_EOF || status == FFMPEG_AVERROR(EAGAIN) {
        break;
      }

      av_packet_rescale_ts(packet, (*encoder).time_base, (*stream).time_base);
      (*packet).stream_index = (*stream).index;
      status = av_interleaved_write_frame(self.container, packet);
      // if status < 0 {}

      av_packet_unref(packet);
    }
  }

  // pub fn add_track(&self, frame: _______) {}

  pub fn free(&self) {
    unsafe {
      avcodec_send_frame(self.image_context, std::ptr::null_mut());
      // avcodec_send_frame(self.audio_context, std::ptr::null_mut());
      av_write_trailer(self.container);

      // Close video stream
      // avcodec_free_context(self.image_context);
      // Close audio stream
      // avcodec_free_context(self.audio_context);

      avio_closep(&mut (*self.container).pb);
      avformat_free_context(self.container);
    }
  }

  pub unsafe fn get_available_codecs() -> Vec<String> {
    let mut codecs = Vec::new();
    let iter: *mut *mut c_void = &mut std::ptr::null_mut();

    loop {
      let codec: *const AVCodec = av_codec_iterate(iter);
      if codec.is_null() { break; }
      if av_codec_is_encoder(codec) > 0 {
        let c_name: &CStr = CStr::from_ptr((*codec).name);
        codecs.push(c_name.to_str().unwrap().to_owned());
      }
    }

    return codecs;
  }
}