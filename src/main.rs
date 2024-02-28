/* 

view는 언제 업데이트가 되는가?
    주기적으로?
    값이 변했을 때?
    


예제 파일 에디터에는 update 시 처리해야될 것?이 비동기로 되있다. 비동기 처리를 해야하는가?
    Command::perform(load_file(default_file()), Message::FileOpened),
    비동기 처리를 위해서는 Command::perform를 붙혀야 하는듯 하다.



2048 내부 로직
    숫자를 옮긴다. + 합친다.
    랜덤한 숫자(2,4,8)를 랜덤한 자리에 넣는다.
    넣을 자리가 없다면 종료 - > GameOver 메시지


GameOver 메시지를 받았으면 기존 화면 위에 띄워야하는데 가능한가?

*/


use iced::alignment::Horizontal;
// use iced::widget::pane_grid::TitleBar;
use iced::executor;
use iced::keyboard;
use iced::theme::Theme;
use iced::widget::{
    column, row, container, text,
};
use iced::{
    Alignment, Application, Command, Element, Length, Settings,
    Subscription, 
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

// #[derive(Copy)]
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

fn view_tile(tile: &TileState) -> Element<Message>  {

    let t = match tile {
        TileState::Empty => {
            text(" ")
                .size(40)
                // .style(Color::from([0.6, 0.6, 0.6]))
                .horizontal_alignment(Horizontal::Center)
                // .into()
        },
        TileState::Value(x) => {
             text(format!("{}", x))
                .size(40)
                // .style(Color::from([0.5, 0.5, 0.5]))
                .horizontal_alignment(Horizontal::Center)
            
        }
    };

    container(t)
        .width(Length::Fixed(100.0))
        .height(Length::Fixed(100.0))
        .center_y()
        .center_x()
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
            _ => {
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
        
        // let mut content = Column::new()
        //     .width(Length::Fill)
        //     .height(Length::Fill)
        //     .align_items(Alignment::Center)
        //     .spacing(10);
        
        // for i in 0..BOARD_LENGTH {
        //     let mut row_content = Row::new();

        //     for j in 0..BOARD_LENGTH {
        //         row_content.push(
        //             view_tile(&self.board[i][j])
        //         ); 
        //     }
        //     content.push(row_content);
        // }

        // container(content)
        //     .width(Length::Fill)
        //     .height(Length::Fill)
        //     .center_y()
        //     .center_x()
        //     .into()
        
        
        let context1 = row![
            view_tile(&self.board[0][0]),
            view_tile(&self.board[0][1]),
            view_tile(&self.board[0][2]),
            view_tile(&self.board[0][3]),
        ]
        .spacing(10)
        .align_items(Alignment::Center);
        
        let context2 = row![
            view_tile(&self.board[1][0]),
            view_tile(&self.board[1][1]),
            view_tile(&self.board[1][2]),
            view_tile(&self.board[1][3]),
        ]
        .spacing(10)
        .align_items(Alignment::Center);
        
        let context3 = row![
            view_tile(&self.board[2][0]),
            view_tile(&self.board[2][1]),
            view_tile(&self.board[2][2]),
            view_tile(&self.board[2][3]),
        ]
        .spacing(10)
        .align_items(Alignment::Center);
        
        let context4 = row![
            view_tile(&self.board[3][0]),
            view_tile(&self.board[3][1]),
            view_tile(&self.board[3][2]),
            view_tile(&self.board[3][3]),
        ]
        .spacing(10)
        .align_items(Alignment::Center);


        let t = column![
            context1,
            context2,
            context3,
            context4,
        ]
        .spacing(10)
        .padding(10);
        
        column![
            text(format!("score : {}", self.score))
                .horizontal_alignment(Horizontal::Left),
            container(t)
                .width(Length::Fill)
                .height(Length::Fill)
                .center_y()
                .center_x()
                // .style()
            ]
            // .spacing(10)
            // .padding(10)
            .into()
        
        
    }
}
