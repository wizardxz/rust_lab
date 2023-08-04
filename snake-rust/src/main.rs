use rand::seq::IteratorRandom;
use std::{
    collections::HashSet,
    io::{self, Write},
};

// 定义范围
const WIDTH: usize = 10;
const HEIGHT: usize = 20;

// 定义规定方向
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

// 定义蛇结构体
struct Snake {
    // 存储蛇身位置的数组
    body: Vec<(usize, usize)>,
    direction: Direction,
    none_occupied: HashSet<(usize, usize)>,
}

impl Snake {
    // 改变方向
    pub fn change_dir(&mut self, dir: Direction) {
        self.direction = dir;
    }

    // 移动
    pub fn move_snake(&mut self) {
        let (cur_x, cur_y) = self.body.last().copied().unwrap();
        let (x, y) = match self.direction {
            Direction::Up => (cur_x - 1, cur_y),
            Direction::Down => (cur_x + 1, cur_y),
            Direction::Left => (cur_x, cur_y - 1),
            Direction::Right => (cur_x, cur_y + 1),
        };
        
        self.none_occupied.remove(&self.body[0]);
        self.none_occupied.insert((x, y));
        self.body.push((x, y));
        self.body.remove(0);
    }
}
impl Snake {
    // 初始化
    fn new() -> Snake {
        let mut snake = Snake {
            body: vec![(WIDTH / 2, WIDTH / 2)],
            direction: Direction::Right,
            none_occupied: HashSet::new(),
        };

        for x in 1..WIDTH - 1 {
            for y in 1..WIDTH - 1 {
                snake.none_occupied.insert((x, y));
            }
        }

        snake
    }

    // 检查游戏结束
    fn is_lose(&self) -> bool {
        let (cur_x, cur_y) = self.body.last().copied().unwrap();
        cur_x == 0 || cur_y == 0 || cur_x == WIDTH - 1 || cur_y == HEIGHT - 1
    }
}

// 绘制游戏界面
fn draw<'a>(snake: &'a Snake, food: (usize, usize)) {
    for i in 0..WIDTH {
        for j in 0..HEIGHT {
            if i == 0 || j == 0 || i == WIDTH - 1 || j == HEIGHT - 1 {
                print!("#");
            } else if snake.body.contains(&(i, j)) {
                print!("S");
            } else if (i, j) == food {
                print!("F");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

// 生成食物，引入生命周期 'a
fn generate_food<'a>(snake: &'a Snake) -> (usize, usize) {
    let mut rng = rand::thread_rng();
    let food = snake
        .none_occupied
        .iter()
        .choose(&mut rng)
        .copied()
        .unwrap();

    food
}
// 接受指令
fn read_input() -> Result<Direction, String> {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .map_err(|e| e.to_string())?;

    let dir = match input.trim() {
        "w" => Direction::Up,
        "a" => Direction::Left,
        "s" => Direction::Down,
        "d" => Direction::Right,
        "q" => return Err("Quit".to_string()),
        _ => return Err("Invalid input".to_string()),
    };

    Ok(dir)
}

fn main() {
    let mut snake = Snake::new();
    let mut food = generate_food(&snake);

    loop {
        draw(&snake, food);

        print!("w->Up, s->Down, a->Left, d->Right, q->Quit: ");
        io::stdout().flush().unwrap(); // 刷新标准输出
        match read_input() {
            Ok(dir) => {
                snake.change_dir(dir);
                snake.move_snake();
            }
            Err(err) => {
                if err == "Quit" {
                    break;
                } else {
                    println!("Error: {}", err);
                }
            }
        }

        if snake.is_lose() {
            println!("You lose!");
            break;
        }

        if snake.body[0] == food {
            snake.body.push(food);
            food = generate_food(&snake);
        }
    }
}
