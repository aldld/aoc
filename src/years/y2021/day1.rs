use std::mem;

use crate::types::{Input, UnitResult};

pub fn part1(_input: Input) -> UnitResult {
    println!("asdf");
    Ok(())
}

pub fn part2(input: Input) -> UnitResult {
    let measurements = input
        .map(|line| line.unwrap().parse::<u32>().unwrap());

    // This could also be computed by tracking the sum, and updating rather than summing
    // each window.
    let mut windowed_measurements = Windowed::exec(measurements, 3, |window| window.iter().sum::<u32>());

    let mut prev_measurement: u32 = windowed_measurements.next().unwrap();

    let mut count = 0;
    for measurement in windowed_measurements {
        if measurement > prev_measurement {
            count += 1;
        }
        prev_measurement = measurement;
    }

    println!("{}", count);

    Ok(())
}

/// A fixed-size sliding window implemented as a circular buffer.
#[derive(Debug)]
struct Window<T> {
    inner: Box<[T]>,
    start: usize,
}

impl<T> Window<T> {
    fn new(inner: Vec<T>) -> Self {
        Self {
            inner: inner.into_boxed_slice(),
            start: 0,
        }
    }

    fn insert(&mut self, value: T) -> T {
        let removed = mem::replace(&mut self.inner[self.start], value);
        self.start = (self.start + 1) % self.inner.len();
        removed
    }

    fn get(&self, index: usize) -> &T {
        &self.inner[(self.start + index) % self.inner.len()]
    }

    fn len(&self) -> usize {
        self.inner.len()
    }

    fn iter(&self) -> WindowIter<'_, T> {
        WindowIter { window: &self, cur_idx: 0 }
    }
}

struct WindowIter<'a, T> {
    window: &'a Window<T>,
    cur_idx: usize,
}

impl<'a, T> Iterator for WindowIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur_idx >= self.window.len() {
            None
        } else {
            let value = self.window.get(self.cur_idx);
            self.cur_idx += 1;
            Some(value)
        }
    }
}

struct Windowed<I, F, T>
where
    I: Iterator,
    F: Fn(&Window<I::Item>) -> T,
{
    inner: I,
    window: Option<Window<I::Item>>,
    func: F,
}

impl<I, F, T> Windowed<I, F, T>
where
    I: Iterator,
    F: Fn(&Window<I::Item>) -> T,
{
    fn exec(mut inner: I, size: usize, func: F) -> Self {
        let initial_window: Vec<I::Item> = inner.by_ref().take(size).collect();
        Self { inner, window: Some(Window::new(initial_window)), func }
    }
}

impl<I, F, T> Iterator for Windowed<I, F, T>
where
    I: Iterator,
    F: Fn(&Window<I::Item>) -> T,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        // TODO edge cases e.g. initial window empty

        if let Some(window) = &mut self.window {
            let value = (self.func)(&window);

            // Update the next window.
            if let Some(next) = self.inner.next() {
                window.insert(next);
            } else {
                self.window = None;
            }

            Some(value)
        } else {
            None
        }
    }
}

