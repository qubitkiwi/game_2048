use iced::alignment::Horizontal;
use iced::theme::{self, Theme};
use iced::widget::{
    button, column, container, row, text, Column, Row
};
use iced::{
    Alignment, Application, Command, Element, Length, Settings, 
    Subscription, executor, keyboard
};

mod custom_theme;

use rand::thread_rng;
use rand::seq::SliceRandom;

const BOARD_LENGTH: usize = 4;

pub fn main() -> iced::Result {

    Game2048::run(Settings {
        ..Settings::default()
    })
}

#[derive(Clone, PartialEq, Debug)]
enum TileState {
    Empty,
    Value(u32),
}

struct Game2048 {
    board: Vec<Vec<TileState>>,
    score: u32,
    game_over: bool,
}

#[derive(Debug, Clone)]
enum Arrow {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone)]
enum Message {
    Move(Arrow),
    Reset,
}

fn view_tile(tile: &TileState) -> Element<Message>  {

    let background;

    let t = match tile {
        TileState::Empty => {
            background = custom_theme::CustomColor::Empty;

            text(" ")
                .size(35)
                .horizontal_alignment(Horizontal::Center)
        },
        TileState::Value(x) => {
            background = match x {
                2=> {custom_theme::CustomColor::Power1},
                4=> {custom_theme::CustomColor::Power2},
                8=> {custom_theme::CustomColor::Power3},
                16=> {custom_theme::CustomColor::Power4},
                32=> {custom_theme::CustomColor::Power5},
                64=> {custom_theme::CustomColor::Power6},
                128=> {custom_theme::CustomColor::Power7},
                256=> {custom_theme::CustomColor::Power8},
                512=> {custom_theme::CustomColor::Power9},
                1024=> {custom_theme::CustomColor::Power10},
                2048=> {custom_theme::CustomColor::Power11},
                _=> {custom_theme::CustomColor::Power12},
            };


             text(format!("{}", x))
                .size(35)
                .horizontal_alignment(Horizontal::Center)
            
        }
    };

    container(t)
        .width(Length::Fixed(100.0))
        .height(Length::Fixed(100.0))
        .center_y()
        .center_x()
        .style(theme::Container::Custom(
            Box::new(custom_theme::CustomContainer {
                background,
            })
        ))
        .into()

}



fn spawn_tile(board: &Vec<Vec<TileState>>) -> Option<Vec<Vec<TileState>>> {
    let mut cloned_board: Vec<Vec<TileState>> = board.iter()
        .map(|inner_vector| inner_vector.clone())
        .collect();

    let mut rng: rand::prelude::ThreadRng = thread_rng();
    let empty_tiles: Vec<(usize, usize)> = (0..4)
        .flat_map(|i| (0..4).map(move |j| (i, j)))
        .filter(|&(i, j)| cloned_board[i][j] == TileState::Empty)
        .collect();

    if let Some((i, j)) = empty_tiles.choose(&mut rng) {
        cloned_board[*i][*j] = TileState::Value(
            if rand::random::<f32>() < 0.7 { 2 } else { 4 });
        Some(cloned_board)
    } else {
        None
    }
    
}

fn is_over(board: &Vec<Vec<TileState>>) -> bool {
    // move albe
    for h in 0..BOARD_LENGTH {
        for w in 0..BOARD_LENGTH {
            if board[h][w] == TileState::Empty {
                return false;
            }
        }
    }

    // marge albe
    for h in 0..BOARD_LENGTH {
        for w in 0..(BOARD_LENGTH - 1) {
            if let TileState::Value(x) = board[h][w] {
                for i in (w + 1)..BOARD_LENGTH {
                    if let TileState::Value(y) = board[h][i] {
                        if x == y {
                            return false;
                        }
                        break;
                    }
                }
            }
        }
    }
    for w in 0..BOARD_LENGTH {
        for h in 0..(BOARD_LENGTH - 1) {
            if let TileState::Value(x) = board[h][w] {
                for i in (h + 1)..BOARD_LENGTH {
                    if let TileState::Value(y) = board[i][w] {
                        if x == y {                            
                            return false;
                        }
                        break;
                    }
                }
            }
        }
    }

    return true;
}


fn move_up(board: &Vec<Vec<TileState>>) -> Option<(Vec<Vec<TileState>>, u32)> {
    
    let mut cloned_board: Vec<Vec<TileState>> = board.iter()
        .map(|inner_vector| inner_vector.clone())
        .collect();
    
    let mut merge_check = false;
    let mut move_check: bool = false;
    let mut score = 0;

    // merge
    for w in 0..BOARD_LENGTH {
        for h in 0..(BOARD_LENGTH - 1) {
            if let TileState::Value(x) = cloned_board[h][w] {
                for i in (h + 1)..BOARD_LENGTH {
                    if let TileState::Value(y) = cloned_board[i][w] {
                        if x == y {
                            cloned_board[h][w] = TileState::Value(x*2);
                            cloned_board[i][w] = TileState::Empty;
                            merge_check = true;
                            score += x*2;

                        }
                        break;
                    }
                }
            }
        }
    }

    // move
    for w in 0..BOARD_LENGTH {
        for h in 0..(BOARD_LENGTH - 1) {
            if let TileState::Empty = cloned_board[h][w] {
                for i in (h + 1)..BOARD_LENGTH {
                    if let TileState::Value(x) = cloned_board[i][w] {
                        cloned_board[h][w] = TileState::Value(x);
                        cloned_board[i][w] = TileState::Empty;
                        move_check = true;

                        break;
                    }
                }
            }
        }
    }


    if merge_check || move_check {
        Some((cloned_board, score))
    } else {
        None
    }
}

fn move_down(board: &Vec<Vec<TileState>>) -> Option<(Vec<Vec<TileState>>, u32)> {
    
    let mut cloned_board: Vec<Vec<TileState>> = board.iter()
        .map(|inner_vector| inner_vector.clone())
        .collect();
    
    let mut merge_check = false;
    let mut move_check: bool = false;
    let mut score = 0;

    // merge
    for w in 0..BOARD_LENGTH {
        for h in (1..BOARD_LENGTH).rev() {
            if let TileState::Value(x) = cloned_board[h][w] {
                for i in (0..h).rev() {
                    if let TileState::Value(y) = cloned_board[i][w] {
                        if x == y {
                            cloned_board[h][w] = TileState::Value(x*2);
                            cloned_board[i][w] = TileState::Empty;
                            merge_check = true;
                            score += x*2;
                        }
                        break;
                    }
                }
            }
        }        
    }

    // move
    for w in 0..BOARD_LENGTH {
        for h in (1..BOARD_LENGTH).rev() {
            if let TileState::Empty = cloned_board[h][w] {
                for i in (0..h).rev() {
                    if let TileState::Value(x) = cloned_board[i][w] {
                        cloned_board[h][w] = TileState::Value(x);
                        cloned_board[i][w] = TileState::Empty;
                        move_check = true;
                        break;
                    }
                }
            }
        }        
    }

    if merge_check || move_check{
        Some((cloned_board, score))
    } else {
        None
    }
}

fn move_left(board: &Vec<Vec<TileState>>) -> Option<(Vec<Vec<TileState>>, u32)> {
    let mut cloned_board: Vec<Vec<TileState>> = board.iter()
        .map(|inner_vector| inner_vector.clone())
        .collect();
    
    let mut merge_check = false;
    let mut move_check = false;
    let mut score = 0;

    // merge
    for h in 0..BOARD_LENGTH {
        for w in 0..(BOARD_LENGTH - 1) {
            if let TileState::Value(x) = cloned_board[h][w] {
                for i in (w + 1)..BOARD_LENGTH {
                    if let TileState::Value(y) = cloned_board[h][i] {
                        if x == y {
                            cloned_board[h][w] = TileState::Value(x*2);
                            cloned_board[h][i] = TileState::Empty;
                            merge_check = true;
                            score += x*2;
                        }
                        break;
                    }
                }
            }
        }
    }
    // move
    for h in 0..BOARD_LENGTH {
        for w in 0..(BOARD_LENGTH - 1) {
            if let TileState::Empty = cloned_board[h][w] {
                for i in (w + 1)..BOARD_LENGTH {
                    if let TileState::Value(x) = cloned_board[h][i] {
                        cloned_board[h][w] = TileState::Value(x);
                        cloned_board[h][i] = TileState::Empty;
                        move_check = true;
                        break;
                    }
                }
            }
        }
    }

    if merge_check || move_check {
        Some((cloned_board, score))
    } else {
        None
    }
}

fn move_right(board: &Vec<Vec<TileState>>) -> Option<(Vec<Vec<TileState>>, u32)> {
    let mut cloned_board: Vec<Vec<TileState>> = board.iter()
        .map(|inner_vector| inner_vector.clone())
        .collect();
    
    let mut merge_check = false;
    let mut move_check = false;
    let mut score = 0;

    // merge
    for h in 0..BOARD_LENGTH {
        for w in (1..BOARD_LENGTH).rev() {
            if let TileState::Value(x) = cloned_board[h][w] {
                for i in (0..w).rev() {
                    if let TileState::Value(y) = cloned_board[h][i] {
                        if x == y {
                            cloned_board[h][w] = TileState::Value(x*2);
                            cloned_board[h][i] = TileState::Empty;
                            merge_check = true;
                            score += x*2;
                        }
                        break;
                    }
                }
            }
        }
    }

    // move
    for h in 0..BOARD_LENGTH {
        for w in (1..BOARD_LENGTH).rev() {
            if let TileState::Empty = cloned_board[h][w] {
                for i in (0..w).rev() {
                    if let TileState::Value(y) = cloned_board[h][i] {
                        cloned_board[h][w] = TileState::Value(y);
                        cloned_board[h][i] = TileState::Empty;
                        move_check = true;
                        break;
                    }
                }
            }
        }
    }

    if merge_check || move_check {
        Some((cloned_board, score))
    } else {
        None
    }
}

impl Application for Game2048 {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        
        let board = vec![vec![TileState::Empty; BOARD_LENGTH]; BOARD_LENGTH];
        let board = spawn_tile(&board).unwrap();

        (            
            Self {
                board,
                score: 0,
                game_over: false,
            },
            Command::none()
        )
    }

    fn title(&self) -> String {
        String::from("Game2048 - Iced")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Move(arr) => {
                
                let f = match arr {
                    Arrow::Up   => { move_up },
                    Arrow::Down => { move_down },
                    Arrow::Left => { move_left },
                    Arrow::Right=> { move_right },
                };

                if let Some((x, score)) = f(&self.board) {
                    if let Some(y) = spawn_tile(&x) {
                        self.board = y;
                        self.score += score;
                    } else {
                        self.board = x;
                        self.score += score;
                    }
                } else if is_over(&self.board) {
                    self.game_over = true;
                }

                Command::none()
            },
            Message::Reset => {
                let board = vec![vec![TileState::Empty; BOARD_LENGTH]; BOARD_LENGTH];
                let board = spawn_tile(&board).unwrap();

                self.board = board;
                self.score = 0;
                self.game_over = false;        
                
                Command::none()
            }
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        use keyboard::key;

        keyboard::on_key_press(|key, _| {
            let keyboard::Key::Named(key) = key else {
                return None;
            };

            match key {
                key::Named::ArrowUp     => Some(Message::Move(Arrow::Up)),
                key::Named::ArrowDown   => Some(Message::Move(Arrow::Down)),
                key::Named::ArrowLeft   => Some(Message::Move(Arrow::Left)),
                key::Named::ArrowRight  => Some(Message::Move(Arrow::Right)),
                _ => None,
            }
        })
        
    }

    fn view(&self) -> Element<Message> {

        let board = if !self.game_over {
            (0..BOARD_LENGTH).into_iter().fold(Column::new().spacing(10).padding(10) ,|c, i|
                c.push(Element::from(
                    (0..BOARD_LENGTH).into_iter().fold(Row::new().spacing(10).align_items(Alignment::Center) ,|c, j|
                        c.push(
                            view_tile(&self.board[i][j])
                        )
                    )
                ))
            )
        } else {
            column!(
                text("Game over").size(30),
            )
        };
        
        container(
            column![
                container(
                    row!(
                        button(text("reset").size(30.0).horizontal_alignment(Horizontal::Center)).on_press(Message::Reset),
                        text("              ").size(35),
                        text(format!("score : {}", self.score)).size(30),
                    )
                ),
                container(board)
            ]
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_y()
        .center_x()
        .into()
        
    }
}
