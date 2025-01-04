use lapix::TestImage;

use lapix::color::{BLACK, TRANSPARENT};
use lapix::{Color, Event, Point, Size, State};

#[test]
fn empty_canvas() {
    let side = 10;
    let state = State::<TestImage>::new(Size::new(side, side), None, None);

    for i in 0..side {
        for j in 0..side {
            assert_eq!(state.canvas().pixel(Point::new(i, j)), TRANSPARENT);
        }
    }
}

#[test]
fn draw_line() {
    let side = 10;
    let mut state = State::<TestImage>::new(Size::new(side, side), None, None);
    state.execute(Event::LineStart(Point::new(0, 0))).unwrap();
    state
        .execute(Event::LineEnd(Point::new(side - 1, side - 1)))
        .unwrap();

    for i in 0..side {
        for j in 0..side {
            let color = if i == j { BLACK } else { TRANSPARENT };

            assert_eq!(state.canvas().pixel(Point::new(i, j)), color);
        }
    }
}

#[test]
fn draw_red_line() {
    let side = 10;
    let mut state = State::<TestImage>::new(Size::new(side, side), None, None);
    let red = Color::new(255, 0, 0, 255);
    state.execute(Event::SetMainColor(red)).unwrap();
    state.execute(Event::LineStart(Point::new(0, 0))).unwrap();
    state
        .execute(Event::LineEnd(Point::new(side - 1, side - 1)))
        .unwrap();

    for i in 0..side {
        for j in 0..side {
            let color = if i == j { red } else { TRANSPARENT };

            assert_eq!(state.canvas().pixel(Point::new(i, j)), color);
        }
    }
}

#[test]
fn draw_line_then_clear_canvas() {
    let side = 10;
    let mut state = State::<TestImage>::new(Size::new(side, side), None, None);
    state.execute(Event::LineStart(Point::new(0, 0))).unwrap();
    state
        .execute(Event::LineEnd(Point::new(side - 1, side - 1)))
        .unwrap();
    state.execute(Event::ClearCanvas).unwrap();

    for i in 0..side {
        for j in 0..side {
            assert_eq!(state.canvas().pixel(Point::new(i, j)), TRANSPARENT);
        }
    }
}

#[test]
fn bucket() {
    let side = 10;
    let mut state = State::<TestImage>::new(Size::new(side, side), None, None);
    state.execute(Event::Bucket(Point::new(0, 0))).unwrap();

    for i in 0..side {
        for j in 0..side {
            assert_eq!(state.canvas().pixel(Point::new(i, j)), BLACK);
        }
    }
}

#[test]
fn bucket_then_erase() {
    let side = 10;
    let mut state = State::<TestImage>::new(Size::new(side, side), None, None);
    state.execute(Event::Bucket(Point::new(0, 0))).unwrap();
    state.execute(Event::EraseStart).unwrap();
    state.execute(Event::Erase(Point::new(0, 0))).unwrap();
    state
        .execute(Event::Erase(Point::new(side - 1, side - 1)))
        .unwrap();
    state.execute(Event::EraseEnd).unwrap();

    for i in 0..side {
        for j in 0..side {
            let color = if i == j { TRANSPARENT } else { BLACK };
            assert_eq!(state.canvas().pixel(Point::new(i, j)), color);
        }
    }
}
