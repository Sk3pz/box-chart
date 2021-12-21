use std::io::stdout;
use crossterm::cursor::MoveTo;
use crossterm::execute;

fn draw_y(x: u16, y: u16, y1: u16) {
    if y == y1 {
        return;
    }
    let mut stdout = stdout();
    execute!(stdout, MoveTo(x, y)).unwrap();
    if y1 < y { // up
        let dy = y - y1;
        print!("┘");
        // the line moves up, so y is lower than y1
        for i in 1..dy {
            execute!(stdout, MoveTo(x, y1 + i)).unwrap();
            print!("│");
        }
        execute!(stdout, MoveTo(x, y1)).unwrap();
        print!("┌");
    } else { // down
        let dy = y1 - y;
        print!("┐");
        for i in 1..dy {
            execute!(stdout, MoveTo(x, y + i)).unwrap();
            print!("│");
        }
        execute!(stdout, MoveTo(x, y1)).unwrap();
        print!("└");
    }
}

pub fn draw_line(x: u16, x1: u16, y: u16, y1: u16) {
    // draws a line in the terminal with *'s between two points
    let mut stdout = stdout();
    if x > x1 {
        // ensure lines can not go out of bounds
        return;
    }

    let dx = x1 - x;
    let dy_raw = y1 as i16 - y as i16;
    let dy = if dy_raw < 0 {
        (dy_raw * -1) as u16
    } else {
        dy_raw as u16
    };

    // no line can be drawn, so return
    if dx < 0 {
        return;
    }

    // the line is too short for normal calculations
    if dx == 0 {
        if dy != 0 {
            // the line only goes vertical, so no need for change in x
            draw_y(x, y, y1);
        } else {
            // the line is only one point
            execute!(stdout, MoveTo(x, y)).unwrap();
            print!("-");
        }
        return;
    }

    // the line is too short for normal calculations, but too long for the above
    if dx == 1 {
        if dy != 0 {
            // the line only goes vertical, so no need for change in x
            draw_y(x, y, y1);
        } else {
            // the line is only one point
            execute!(stdout, MoveTo(x, y)).unwrap();
            print!("╶╴");
        }
        return;
    }

    // the line is horizontal
    if y == y1 {
        execute!(stdout, MoveTo(x, y)).unwrap();
        print!("╶{}╴", "─".repeat((dx - 1) as usize));
        return;
    }

    // special edge case for dx == 2
    if dx == 2 {
        if dy == 1 {
            // the line is a single point
            execute!(stdout, MoveTo(x, y)).unwrap();
            print!("╶╴");
            return;
        }
        // the line is a vertical line
        execute!(stdout, MoveTo(x, y)).unwrap();
        print!("╶");
        draw_y(x + 1, y, y1);
        execute!(stdout, MoveTo(x1, y1)).unwrap();
        print!("╴");
        return;
    }

    // draw the end points
    execute!(stdout, MoveTo(x, y)).unwrap();
    print!("╶");
    execute!(stdout, MoveTo(x1, y1)).unwrap();
    print!("╴");

    // get the length of the two x sides of the line
    let half_x = dx / 2;
    // ensure the line is the correct length in case dx is odd
    // if dx is greater than half_x * 2, then half_x is one less than it should be, so add one to
    // this side to make it even without extending the line beyond what it should be.
    // this adds the difference in dx and half_x * 2 to half_x so if there is a difference, it is
    // added back.
    let right_line_x = half_x + (dx - (half_x * 2));
    // draw the left side of the line
    for i in 0..half_x {
        execute!(stdout, MoveTo(x + i + 1, y)).unwrap();
        print!("─");
    }
    // draw the vertical portion of the line
    draw_y(x + half_x + 1, y, y1);
    // draw the right side of the line
    let right_line_start = x + half_x + 1;
    if right_line_x < 2 {
        // nothing to do here?
        return;
    }
    for i in 0..right_line_x - 2 {
        execute!(stdout, MoveTo(right_line_start + i + 1, y1)).unwrap();
        print!("─");
    }
}