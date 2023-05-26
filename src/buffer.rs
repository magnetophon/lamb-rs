/*
   ┏━━━┓╱╱╱╱╱┏┓╱╱┏┓╱╱╱╱╱┏┓╱╱
   ┃┏━┓┃╱╱╱╱╱┃┃╱╱┃┃╱╱╱╱╱┃┃╱╱╱
   ┃┗━┛┣┓┏┳━━┫┃┏┓┃┃╱╱┏━━┫┗━┳━━┓
   ┃┏━━┫┃┃┃┏┓┃┗┛┃┃┃╱┏┫┏┓┃┏┓┃━━┫
   ┃┃╱╱┃┗┛┃┃┃┃┏┓┃┃┗━┛┃┏┓┃┗┛┣━━┃
   ┗┛╱╱┗━━┻┛┗┻┛┗┛┗━━━┻┛┗┻━━┻━━┛
    ━━━━━━━━━━━━━━━━━━━━━━━━━━

    Copyright (c) 2022 Punk Labs LLC

    This section is part of OneTrick

    OneTrick is free software: you can redistribute it and/or modify it
    under the terms of the GNU General Public License as published by the Free
    Software Foundation, either version 3 of the License, or (at your option)
    any later version.

    OneTrick is distributed in the hope that it will be useful, but WITHOUT
    ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or
    FITNESS FOR A PARTICULAR PURPOSE.  See the GNU General Public License for
    more details.

    You should have received a copy of the GNU General Public License along with
    OneTrick.  If not, see <http://www.gnu.org/licenses/>.
*/
use nih_plug::prelude::*;

#[derive(Default)]
pub struct TempBuffer {
    data: Vec<Vec<f32>>,
}

#[allow(unused)]
impl TempBuffer {
    pub fn channel_count(&self) -> usize {
        self.data.len()
    }
    pub fn resize(&mut self, channel_count: usize, max_frames: usize) {
        if self.data.len() < channel_count || (channel_count > 0 && self.data[0].len() < max_frames)
        {
            self.data.clear();
            for _i in 0..channel_count {
                self.data.push(vec![0.0; max_frames]);
            }
        }
    }
    pub fn data(&self) -> &Vec<Vec<f32>> {
        &self.data
    }

    pub fn data_mut(&mut self) -> &mut Vec<Vec<f32>> {
        &mut self.data
    }

    pub fn clear(&mut self) {
        for channel in self.data.iter_mut() {
            channel.fill(0.0);
        }
    }
    pub fn clear_frames(&mut self, frames: usize) {
        for channel in self.data.iter_mut() {
            channel[0..frames].fill(0.0);
        }
    }

    pub fn add_to_temp_buffer(&self, to_buffer: &mut TempBuffer) {
        for (self_channel, to_channel) in self.data.iter().zip(to_buffer.data_mut()) {
            for (to, from) in to_channel.iter_mut().zip(self_channel) {
                *to += *from;
            }
        }
    }
    pub fn add_to_temp_buffer_frames(&self, to_buffer: &mut TempBuffer, frames: usize) {
        for (self_channel, to_channel) in self.data.iter().zip(to_buffer.data_mut()) {
            for (to, from) in to_channel[0..frames].iter_mut().zip(self_channel) {
                *to += *from;
            }
        }
    }

    pub fn add_to_buffer(&self, to_buffer: &mut [&mut [f32]]) {
        for (self_channel, to_channel) in self.data.iter().zip(to_buffer.iter_mut()) {
            for (to, from) in to_channel.iter_mut().zip(self_channel) {
                *to += *from;
            }
        }
    }
    pub fn add_to_buffer_frames(&self, to_buffer: &mut [&mut [f32]], frames: usize) {
        for (self_channel, to_channel) in self.data.iter().zip(to_buffer.iter_mut()) {
            for (to, from) in to_channel[0..frames].iter_mut().zip(self_channel) {
                *to += *from;
            }
        }
    }

    pub fn write_to_buffer(&self, to_buffer: &mut [&mut [f32]]) {
        for (self_channel, to_channel) in self.data.iter().zip(to_buffer.iter_mut()) {
            to_channel.copy_from_slice(&self_channel[..to_channel.len().min(self_channel.len())]);
        }
    }
    pub fn write_to_buffer_frames(&self, to_buffer: &mut [&mut [f32]], frames: usize) {
        for (self_channel, to_channel) in self.data.iter().zip(to_buffer.iter_mut()) {
            to_channel.copy_from_slice(
                &self_channel[..to_channel.len().min(self_channel.len()).min(frames)],
            );
        }
    }
    pub fn read_from_buffer(&mut self, from_buffer: &mut Buffer) {
        for (buffer_channel, self_channel) in from_buffer
            .as_slice_immutable()
            .iter()
            .zip(self.data.iter_mut())
        {
            for (buffer_sample, self_sample) in buffer_channel.iter().zip(self_channel) {
                *self_sample = *buffer_sample;
            }
        }
    }
    /*
    pub fn read_from_buffer(&mut self, from_buffer: &mut Buffer) {
        for (buffer_channel, mut self_channel) in from_buffer.as_slice().iter().zip(self.data.iter_mut()) {
            let self_len = self_channel.len();
            //self_channel.copy_from_slice( //Allocation
            //    &buffer_channel[..self_len.min(buffer_channel.len())]
            //);
        }
    }
    */
    pub fn read_from_slice(&mut self, from_buffer: &[&[f32]]) {
        for (self_channel, from_channel) in self.data.iter_mut().zip(from_buffer.iter()) {
            let self_len = self_channel.len();
            self_channel.copy_from_slice(&from_channel[..self_len.min(from_channel.len())]);
        }
    }
    pub fn read_from_slice_frames(&mut self, from_buffer: &[&[f32]], frames: usize) {
        for (self_channel, from_channel) in self.data.iter_mut().zip(from_buffer.iter()) {
            let self_len = self_channel.len();
            self_channel
                .copy_from_slice(&from_channel[..self_len.min(from_channel.len()).min(frames)]);
        }
    }
    pub fn read_from_mut_slice(&mut self, from_buffer: &mut [&mut [f32]]) {
        for (self_channel, from_channel) in self.data.iter_mut().zip(from_buffer.iter()) {
            let self_len = self_channel.len();
            self_channel.copy_from_slice(&from_channel[..self_len.min(from_channel.len())]);
        }
    }
    pub fn read_from_mut_slice_frames(&mut self, from_buffer: &mut [&mut [f32]], frames: usize) {
        for (self_channel, from_channel) in self.data.iter_mut().zip(from_buffer.iter()) {
            let self_len = self_channel.len();
            self_channel
                .copy_from_slice(&from_channel[..self_len.min(from_channel.len()).min(frames)]);
        }
    }
    /*
    // Allocates a Vec on the heap, so not suitable for DSP...
    // Relevant Discussion: https://internals.rust-lang.org/t/collecting-iterators-into-arrays/10330
    pub fn slice<const N:usize>(&self) -> [&[f32]; N] {
        let slice = self.data.as_slice();
        let mut sliced = slice.iter().map(|ch| ch.as_slice());
        sliced.collect::<Vec<&[f32]>>().try_into().unwrap()
    }
    */
    pub fn slice1d(&self) -> [&[f32]; 1] {
        let slice = self.data.as_slice();
        let mut sliced = slice.iter().map(|ch| ch.as_slice());
        [sliced.next().unwrap()]
    }
    pub fn slice2d(&self) -> [&[f32]; 2] {
        let slice = self.data.as_slice();
        let mut sliced = slice.iter().map(|ch| ch.as_slice());
        [sliced.next().unwrap(), sliced.next().unwrap()]
    }
    pub fn slice3d(&self) -> [&[f32]; 3] {
        let slice = self.data.as_slice();
        let mut sliced = slice.iter().map(|ch| ch.as_slice());
        [
            sliced.next().unwrap(),
            sliced.next().unwrap(),
            sliced.next().unwrap(),
        ]
    }
    pub fn slice4d(&self) -> [&[f32]; 4] {
        let slice = self.data.as_slice();
        let mut sliced = slice.iter().map(|ch| ch.as_slice());
        [
            sliced.next().unwrap(),
            sliced.next().unwrap(),
            sliced.next().unwrap(),
            sliced.next().unwrap(),
        ]
    }
}

pub trait BufferSlicer {
    fn slice1d(&mut self, start_index: usize, end_index: usize) -> [&mut [f32]; 1];
    fn slice2d(&mut self, start_index: usize, end_index: usize) -> [&mut [f32]; 2];
    fn slice3d(&mut self, start_index: usize, end_index: usize) -> [&mut [f32]; 3];
    fn slice4d(&mut self, start_index: usize, end_index: usize) -> [&mut [f32]; 4];
}

impl<'buffer> BufferSlicer for Buffer<'buffer> {
    fn slice1d(&mut self, start_index: usize, end_index: usize) -> [&mut [f32]; 1] {
        let slice = self.as_slice();
        let mut sliced = slice.iter_mut().map(|ch| &mut ch[start_index..end_index]);
        [sliced.next().unwrap()]
    }

    fn slice2d(&mut self, start_index: usize, end_index: usize) -> [&mut [f32]; 2] {
        let slice = self.as_slice();
        let mut sliced = slice.iter_mut().map(|ch| &mut ch[start_index..end_index]);
        [sliced.next().unwrap(), sliced.next().unwrap()]
    }

    fn slice3d(&mut self, start_index: usize, end_index: usize) -> [&mut [f32]; 3] {
        let slice = self.as_slice();
        let mut sliced = slice.iter_mut().map(|ch| &mut ch[start_index..end_index]);
        [
            sliced.next().unwrap(),
            sliced.next().unwrap(),
            sliced.next().unwrap(),
        ]
    }
    fn slice4d(&mut self, start_index: usize, end_index: usize) -> [&mut [f32]; 4] {
        let slice = self.as_slice();
        let mut sliced = slice.iter_mut().map(|ch| &mut ch[start_index..end_index]);
        [
            sliced.next().unwrap(),
            sliced.next().unwrap(),
            sliced.next().unwrap(),
            sliced.next().unwrap(),
        ]
    }
}
