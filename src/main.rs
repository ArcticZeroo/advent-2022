extern crate core;

mod days;
mod common;
mod vm;

fn main() {
    days::day1::run();
    days::day2::run();
    days::day3::run();
    days::day4::run();
    days::day5::run();
    days::day6::run();
    days::day8::run();
    days::day9::run();
    days::day10::run();
    // todo: optimize these so I can run every day at once (:
    // days::day11::run();
    // days::day12::run();
    // days::day13::run();
    // days::day14::run();
    days::day15::run();
}
