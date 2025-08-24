use std::{
    collections::{VecDeque, vec_deque::Iter},
    iter,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Packet {
    class: u8,
    flow: u16,
    payload: u64,
}

impl Packet {
    pub fn new(payload: u64, class: u8, flow: u16) -> Self {
        Self {
            payload,
            class,
            flow,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Scheduler {
    queues: [VecDeque<Packet>; 8],
    weights: [usize; 8],
}

impl Default for Scheduler {
    fn default() -> Self {
        Self {
            queues: Default::default(),
            weights: [1, 1, 1, 1, 1, 1, 1, 1], // zero weight will drop the queue entirely
        }
    }
}

impl Scheduler {
    pub fn enqueue(&mut self, packets: &[Packet]) {
        for packet in packets {
            self.queues[packet.class as usize].push_back(*packet);
        }
    }

    pub fn new(weights: [usize; 8]) -> Self {
        Self {
            queues: Default::default(),
            weights,
        }
    }
    pub fn iter(&self) -> SchedulerIter {
        SchedulerIter {
            scheduler: self.queues.iter().map(|queue| queue.iter()).collect(),
            buffer: VecDeque::new(),
            weights: self.weights,
        }
    }
}

impl<'a> IntoIterator for &'a Scheduler {
    type Item = &'a Packet;

    type IntoIter = SchedulerIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

pub struct SchedulerIter<'a> {
    scheduler: Vec<Iter<'a, Packet>>,
    buffer: VecDeque<&'a Packet>,
    weights: [usize; 8],
}

impl<'a> Iterator for SchedulerIter<'a> {
    type Item = &'a Packet;

    fn next(&mut self) -> Option<Self::Item> {
        if self.buffer.is_empty() {
            self.buffer.append(
                &mut self
                    .scheduler
                    .iter_mut()
                    .enumerate()
                    .flat_map(|(index, queue)| {
                        iter::repeat_with(|| queue.next())
                            .take(self.weights[index])
                            .flatten()
                    })
                    .collect(),
            );
        }
        self.buffer.pop_front()
    }
}

#[cfg(test)]
mod tests;
