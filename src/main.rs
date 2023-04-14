use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
    Result,
};
use crossterm::event::{poll, read, Event};
use std::{env, io::stdout, thread, time::Duration};

fn quick_sort<T: Ord + std::fmt::Display>(arr: &mut [T]) -> Result<()> {
    if arr.len() > 1 {
        let pivot_index = partition(arr)?;
        quick_sort(&mut arr[0..pivot_index])?;
        quick_sort(&mut arr[pivot_index + 1..])?;
    }
    Ok(())
}

fn partition<T: Ord + std::fmt::Display>(arr: &mut [T]) -> Result<usize> {
    let len = arr.len();
    let pivot_index = len / 2;
    arr.swap(pivot_index, len - 1);

    let mut i = 0;
    for j in 0..len - 1 {
        if arr[j] <= arr[len - 1] {
            arr.swap(i, j);
            i += 1;
        }
        visualize(arr, pivot_index, Some((i, j)))?;
    }

    arr.swap(i, len - 1);
    visualize(arr, pivot_index, None)?;
    Ok(i)
}

fn visualize<T: std::fmt::Display>(
    arr: &[T],
    pivot_index: usize,
    highlights: Option<(usize, usize)>,
) -> Result<()> {
    execute!(
        stdout(),
        SetForegroundColor(Color::Yellow),
        Print("Array: "),
        ResetColor
    )?;
    for (i, item) in arr.iter().enumerate() {
        if let Some((x, y)) = highlights {
            if i == x || i == y {
                execute!(stdout(), SetForegroundColor(Color::Cyan))?;
            }
        }

        if i == pivot_index {
            execute!(stdout(), SetForegroundColor(Color::Red))?;
        }

        execute!(stdout(), Print(format!("{} ", item)), ResetColor)?;
    }
    execute!(stdout(), Print("\n"))?;
    thread::sleep(Duration::from_millis(100));
    Ok(())
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();
    let mut numbers: Vec<i32> = args
        .iter()
        .filter_map(|arg| arg.parse::<i32>().ok())
        .collect();

    if numbers.is_empty() {
        eprintln!("Please provide a list of numbers separated by spaces as an arguments.");
        return Ok(());
    }

    println!("Before: {:?}", numbers);
    execute!(stdout(), EnterAlternateScreen)?;
    quick_sort(&mut numbers)?;
    execute!(stdout(), Print("Press any key to exit..."))?;

    loop {
        if poll(Duration::from_millis(100))? {
            if let Event::Key(_) = read()? {
                break;
            }
        }
    }

    execute!(stdout(), LeaveAlternateScreen)?;
    println!("After: {:?}", numbers);
    Ok(())
}
