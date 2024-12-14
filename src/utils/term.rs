use std::io;
use std::io::{Read, Write};
use termios::*;
pub fn getch() -> char {
    let stdin = 0; // couldn't get std::os::unix::io::FromRawFd to work
    // on /dev/stdin or /dev/tty
    let termios = Termios::from_fd(stdin).unwrap();
    let mut new_termios = termios.clone();  // make a mutable copy of termios
    // that we will modify
    new_termios.c_lflag &= !(ICANON | ECHO); // no echo and canonical mode
    tcsetattr(stdin, TCSANOW, &mut new_termios).unwrap();
    let stdout = io::stdout();
    let mut reader = io::stdin();
    let mut buffer = [0;1];  // read exactly one byte
    stdout.lock().flush().unwrap();
    reader.read_exact(&mut buffer).unwrap();
    tcsetattr(stdin, TCSANOW, & termios).unwrap();

    buffer[0] as char
}

pub(crate) fn speed_scaling(speed: isize) -> isize {
    let sign = speed.signum();
    let speed = speed.abs();
    let mut result = 0;
    for i in 0..speed {
        result += i;
    }
    result * sign
}

pub(crate) fn ch_to_speed(ch: char) -> isize {
    let positive = "1234567890";
    let negative = "!§#$%^&*()";
    if positive.contains(ch) {
        positive.find(ch).unwrap() as isize
    } else if negative.contains(ch) {
        -(negative.find(ch).unwrap() as isize)
    } else {
        0
    }
}