use iced::alignment::Horizontal;
use iced::theme::{self, Theme};
use iced::{color, Background, Border, Shadow};
use iced::widget::{
    column, container, row, text, Column, Row
};
use iced::{
    Alignment, Application, Command, Element, Length, Settings, 
    Subscription, executor, keyboard
};

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
}

#[derive(Debug, Clone)]
enum Message {
    MoveUp,
    MoveDown,
    MoveRight,
    MoveLeft,
    Process(Option<(Vec<Vec<TileState>>, u32)>),
}

#[derive(Default)]
enum CustomColor {
    #[default]
    Empty,
    Power1,
    Power2,
    Power3,
    Power4,
    Power5,
    Power6,
    Power7,
    Power8,
    Power9,
    Power10,
    Power11,
    Power12,
}
struct CustomContainer {
    background : CustomColor,
}



impl container::StyleSheet for CustomContainer {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        let background: Option<Background> = match self.background {
            CustomColor::Empty => { Some(Background::Color(color!(0xcc, 0xc0, 0xb4))) },
            CustomColor::Power1 => { Some(Background::Color(color!(0xee, 0xe4, 0xda))) },
            CustomColor::Power2 => { Some(Background::Color(color!(0xed, 0xe0, 0xc8))) },
            CustomColor::Power3 => { Some(Background::Color(color!(0xf2, 0xb1, 0x79))) },
            CustomColor::Power4 => { Some(Background::Color(color!(0xf5, 0x95, 0x63))) },
            CustomColor::Power5 => { Some(Background::Color(color!(0xf6, 0x7c, 0x5f))) },
            CustomColor::Power6 => { Some(Background::Color(color!(0xf6, 0x5e, 0x3b))) },
            CustomColor::Power7 => { Some(Background::Color(color!(0xed, 0xcf, 0x72))) },
            CustomColor::Power8 => { Some(Background::Color(color!(0xed, 0xcc, 0x61))) },
            CustomColor::Power9 => { Some(Background::Color(color!(0xed, 0xc8, 0x50))) },
            CustomColor::Power10 => { Some(Background::Color(color!(0xed, 0xc5, 0x3f))) },
            CustomColor::Power11 => { Some(Background::Color(color!(0xed, 0xc2, 0x2e))) },
            CustomColor::Power12 => { Some(Background::Color(color!(149, 40, 169))) },
        };


        container::Appearance {
            text_color: None,
            background ,
            border: Border::with_radius(3),
            shadow: Shadow::default(),
        }
    }
}

fn view_tile(tile: &TileState) -> Element<Message>  {

    let background;

    let t = match tile {
        TileState::Empty => {
            background = CustomColor::Empty;

            text(" ")
                .size(35)
                .horizontal_alignment(Horizontal::Center)
        },
        TileState::Value(x) => {
            background = match x {
                2=> {CustomColor::Power1},
                4=> {CustomColor::Power2},
                8=> {CustomColor::Power3},
                16=> {CustomColor::Power4},
                32=> {CustomColor::Power5},
                64=> {CustomColor::Power6},
                128=> {CustomColor::Power7},
                256=> {CustomColor::Power8},
                512=> {CustomColor::Power9},
                1024=> {CustomColor::Power10},
                2048=> {CustomColor::Power11},
                _=> {CustomColor::Power12},
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
            Box::new(CustomContainer {
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

async fn process(board: Vec<Vec<TileState>>, f: fn(&Vec<Vec<TileState>>) -> Option<(Vec<Vec<TileState>>, u32)>) -> Option<(Vec<Vec<TileState>>, u32)> {
    if let Some((x, score)) = f(&board) {
        if let Some(y) = spawn_tile(&x) {
            return Some((y, score));
        } else {
            return Some((x, score));
        }
    } else if is_over(&board) {
        None
    } else {
        Some((board, 0))
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
            },
            Command::none()
        )
    }

    fn title(&self) -> String {
        String::from("Game2048 - Iced")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::MoveUp => {
                Command::perform(process(self.board.clone(), move_up), Message::Process)
            },
            Message::MoveDown => {
                Command::perform(process(self.board.clone(), move_down), Message::Process)
            },
            Message::MoveLeft => {
                Command::perform(process(self.board.clone(), move_left), Message::Process)
            },
            Message::MoveRight => {
                Command::perform(process(self.board.clone(), move_right), Message::Process)
            },
            Message::Process(result) => {
                if let Some((x, score)) = result {
                    self.board = x;
                    self.score += score;
                } else {
                    // game over
                    println!("game over");
                }
                Command::none()
            },
        }
    }

    fn subscription(&self) -> Subscription<Message> {        
        use keyboard::key;

        keyboard::on_key_press(|key, _| {
            let keyboard::Key::Named(key) = key else {
                return None;
            };

            match key {
                key::Named::ArrowUp     => Some(Message::MoveUp),
                key::Named::ArrowDown   => Some(Message::MoveDown),
                key::Named::ArrowRight  => Some(Message::MoveRight),
                key::Named::ArrowLeft   => Some(Message::MoveLeft),
                _ => None,
            }
        })
        
    }

    fn view(&self) -> Element<Message> {


        let board = (0..BOARD_LENGTH).into_iter().fold(Column::new().spacing(10).padding(10) ,|c, i|
            c.push(Element::from(
                (0..BOARD_LENGTH).into_iter().fold(Row::new().spacing(10).align_items(Alignment::Center) ,|c, j|
                    c.push(
                        view_tile(&self.board[i][j])
                    )
                )
            ))
        );

        column![
            container(
                text(format!("score : {}", self.score)).size(30)
            )
                .width(Length::Fill)
            ,
            container(board)
                .width(Length::Fill)
                .height(Length::Fill)
                .center_y()
                .center_x(),
        ]
        .into()
        
    }
}
