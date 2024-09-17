use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    execute,
    style::{self, Stylize},
    terminal::{self, ClearType},
    ExecutableCommand,
};
use std::{
    io::{self, Write},
    time::Duration,
};

struct Stage {
    message: &'static str,
    left_option: &'static str,
    right_option: &'static str,
    next_left_stage: Option<usize>,  
    next_right_stage: Option<usize>, 
    is_ending: bool,                
}

pub fn methodofline() {
    terminal::enable_raw_mode().unwrap();

    let stages = vec![
        Stage {
            message: "breakfast?",
            left_option: "frosties",
            right_option: "sugar puffs",
            next_left_stage: Some(1),
            next_right_stage: Some(1), 
            is_ending: false,
        },
        Stage {
            message: "music?",
            left_option: "thomson twins",
            right_option: "now",
            next_left_stage: Some(2), 
            next_right_stage: Some(2), 
            is_ending: false,
        },
        Stage {
            message: "work for tuckersoft?",
            left_option: "yes",
            right_option: "no",
            next_left_stage: Some(3),     
            next_right_stage: Some(3), 
            is_ending: true,
        },
        Stage {
            message: "talk about mom to the doctor?",
            left_option: "yes",
            right_option: "no",
            next_left_stage: Some(4), 
            next_right_stage: None,  
            is_ending: false,
        },
        Stage {
            message: "are you coming?",
            left_option: "yes",
            right_option: "no",
            next_left_stage: Some(5),  
            next_right_stage: Some(5), 
            is_ending: true,
        },
        Stage {
            message: "which record?",
            left_option: "pheadra",
            right_option: "The BermudaTringle",
            next_left_stage: Some(6),  
            next_right_stage: Some(6), 
            is_ending: true,
        },
        Stage {
            message: "dad is worried ... ",
            left_option: "Throw Tea",
            right_option: "shout at dad",
            next_left_stage: None,     
            next_right_stage: Some(7), 
            is_ending: true,
        },
        Stage {
            message: "where to go ? ",
            left_option: "followin colin",
            right_option: "Doctor",
            next_left_stage: Some(8), 
            next_right_stage: Some(10), 
            is_ending: true,
        },
        Stage {
            message: "Take Drugs?",
            left_option: "yes",
            right_option: "no",
            next_left_stage: Some(9),  
            next_right_stage: Some(9), 
            is_ending: true,
        },
        Stage {
            message: "whos jump?",
            left_option: "colin",
            right_option: "stefan",
            next_left_stage: Some(10), 
            next_right_stage: None, 
            is_ending: true,
        },
        Stage {
            message: "At the Doctor",
            left_option: "bite nails",
            right_option: "pull earlobe",
            next_left_stage: Some(11), 
            next_right_stage: Some(11), 
            is_ending: true,
        },
        Stage {
            message: "medication",
            left_option: "throw away",
            right_option: "flush",
            next_left_stage: Some(12), 
            next_right_stage: Some(12),
            is_ending: true,
        },
        Stage {
            message: "computer is slow",
            left_option: "destroy computer",
            right_option: "hit desk",
            next_left_stage: Some(13), 
            next_right_stage: Some(13),
            is_ending: true,
        },
        Stage {
            message: "PICK UP",
            left_option: "book",
            right_option: "family photo",
            next_left_stage: Some(14), 
            next_right_stage: Some(15),
            is_ending: true,
        },
        Stage {
            message: "password",
            left_option: "pax",
            right_option: "jfd",
            next_left_stage: None, 
            next_right_stage: None,
            is_ending: true,
        },
        Stage {
            message: "computer",
            left_option: "destroy computer",
            right_option: "Throw Tea",
            next_left_stage: Some(16), 
            next_right_stage: Some(16),
            is_ending: true,
        },
        Stage {
            message: "who are you ?",
            left_option: "NETFLIX",
            right_option: "👾",
            next_left_stage: Some(17), 
            next_right_stage: Some(23),
            is_ending: true,
        },
        Stage {
            message: "more action?",
            left_option: "yes",
            right_option: "fuck yeah",
            next_left_stage: Some(18), 
            next_right_stage: Some(18),
            is_ending: true,
        },
        Stage {
            message: "leap through window",
            left_option: "Leap through window",
            right_option: "Fight Dr Haynes",
            next_left_stage: Some(19), 
            next_right_stage: Some(20),
            is_ending: true,
        },
        Stage {
            message: "Director yells cut",
            left_option: "",
            right_option: "",
            next_left_stage: None, 
            next_right_stage: None,
            is_ending: true,
        },
        Stage {
            message: "Fight Dr Haynes",
            left_option: "Karate Chop Dad",
            right_option: "Kick in balls",
            next_left_stage: None, 
            next_right_stage: None,
            is_ending: true,
        },
        Stage {
            message: "Kill dad",
            left_option: "Bury body",
            right_option: "Chop Up body",
            next_left_stage: Some(24), 
            next_right_stage: Some(26),
            is_ending: true,
        },
        Stage {
            message: "Will The Game Be Ready ?",
            left_option: "yes",
            right_option: "no",
            next_left_stage: Some(25), 
            next_right_stage: None,
            is_ending: true,
        },
        Stage {
            message: "hwere is colin?",
            left_option: "no idea",
            right_option: "he jumped",
            next_left_stage: None, 
            next_right_stage: None,
            is_ending: true,
        },
        Stage {
            message: "Chop Up body",
            left_option: "destroy computer",
            right_option: "throw tea",
            next_left_stage: Some(1), 
            next_right_stage: Some(1),
            is_ending: true,
        },
    ];

    game_logic(&stages);

    terminal::disable_raw_mode().unwrap();
}

fn game_logic(stages: &[Stage]) {

    let mut stage_index = 0;

    draw_banner(&stages[stage_index], "center");

    loop {
        if event::poll(Duration::from_millis(500)).unwrap() {
            if let Event::Key(key_event) = event::read().unwrap() {
                match key_event.code {
                    KeyCode::Left => {
                        // <-
                        execute!(io::stdout(), terminal::Clear(ClearType::All)).unwrap();
                        println!(
                            "You chose: {}",
                            stages[stage_index].left_option.green().bold()
                        );

                        if let Some(next_stage) = stages[stage_index].next_left_stage {
                            stage_index = next_stage;
                            draw_banner(&stages[stage_index], "<-");
                        } else if stages[stage_index].is_ending {
                            println!("Game Over - Ending");
                            break;
                        }
                    }
                    KeyCode::Right => {
                        // ->
                        execute!(io::stdout(), terminal::Clear(ClearType::All)).unwrap();
                        println!(
                            "You chose: {}",
                            stages[stage_index].right_option.red().bold()
                        );

                        if let Some(next_stage) = stages[stage_index].next_right_stage {
                            stage_index = next_stage;
                            draw_banner(&stages[stage_index], "->");
                        } else if stages[stage_index].is_ending {
                            println!("Game Over - Ending");
                            break;
                        }
                    }
                    KeyCode::Char('q') => {
                        // exit game
                        println!("{}", "Exiting...".red().bold());
                        break;
                    }
                    _ => {
                        execute!(io::stdout(), terminal::Clear(ClearType::All)).unwrap();
                        draw_banner(&stages[stage_index], "center");
                        println!(
                            "{}",
                            "oops! just use <- or -> or 'q' to quit.".yellow().bold()
                        );
                    }
                }

                io::stdout().flush().unwrap();
            }
        }
    }
}


fn draw_banner(stage: &Stage, choice: &str) {
    let banner_width = 36; 
    let border_top = format!("╔{}╗", "═".repeat(banner_width));
    let border_bottom = format!("╚{}╝", "═".repeat(banner_width));

    let (cols, rows) = terminal::size().unwrap();

    let banner_start_col = (cols / 2) as usize - (banner_width / 2);
    let banner_start_row = (rows / 2) as usize - 2; 

    execute!(io::stdout(), terminal::Clear(ClearType::All)).unwrap();

    execute!(
        io::stdout(),
        cursor::MoveTo(banner_start_col as u16, banner_start_row as u16)
    )
    .unwrap();
    println!("{}", border_top.bold().blue()); 

    execute!(
        io::stdout(),
        cursor::MoveTo(banner_start_col as u16, (banner_start_row + 1) as u16)
    )
    .unwrap();
    println!("║{:^36}║", stage.message.magenta().bold()); 

    execute!(
        io::stdout(),
        cursor::MoveTo(banner_start_col as u16, (banner_start_row + 2) as u16)
    )
    .unwrap();
    println!(
        "║{:^14} | {:^14}║",
        stage.left_option.green(),
        stage.right_option.red()
    ); 

    execute!(
        io::stdout(),
        cursor::MoveTo(banner_start_col as u16, (banner_start_row + 4) as u16)
    )
    .unwrap();
    println!("{}", border_bottom.bold().blue()); 

    match choice {
        "<-" => {
            execute!(
                io::stdout(),
                cursor::MoveTo(banner_start_col as u16, (banner_start_row + 6) as u16)
            )
            .unwrap();
            println!("{}", "<-".green().bold());
        }
        "->" => {
            execute!(
                io::stdout(),
                cursor::MoveTo(banner_start_col as u16, (banner_start_row + 6) as u16)
            )
            .unwrap();
            println!("{}", "->".red().bold());
        }
        _ => (),
    }
}
