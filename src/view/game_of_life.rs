use egui::{Color32, Painter, Pos2, Rect, Sense, Ui, Vec2};
use std::time::{Duration, Instant};

use super::View;

// 游戏状态

#[derive(Debug, PartialEq, Clone)]
pub struct GameOfLifeView {
    grid: Vec<Vec<bool>>, // 细胞网格
    width: usize,         // 网格宽度
    height: usize,        // 网格高度
    cell_size: f32,       // 细胞显示大小
    running: bool,        // 是否正在运行
    last_update: Instant, // 上次更新时间
    update_interval: f32, // 更新间隔（秒）
}

impl Default for GameOfLifeView {
    fn default() -> Self {
        Self {
            grid: vec![vec![false; 50]; 50],
            width: 50,
            height: 50,
            cell_size: 15.0,
            running: false,
            last_update: Instant::now(),
            update_interval: 0.1,
        }
    }
}

impl GameOfLifeView {
    fn new(width: usize, height: usize, cell_size: f32) -> Self {
        Self {
            grid: vec![vec![false; height]; width],
            width,
            height,
            cell_size,
            running: false,
            last_update: Instant::now(),
            update_interval: 0.1,
        }
    }

    // 计算下一代
    fn next_generation(&mut self) {
        let mut new_grid = self.grid.clone();

        for x in 0..self.width {
            for y in 0..self.height {
                let neighbors = self.count_neighbors(x, y);

                new_grid[x][y] = match (self.grid[x][y], neighbors) {
                    (true, 2) | (true, 3) => true, // 存活
                    (true, _) => false,            // 死亡
                    (false, 3) => true,            // 新生
                    _ => false,
                };
            }
        }
        self.grid = new_grid;
    }

    // 计算周围存活细胞数量
    fn count_neighbors(&self, x: usize, y: usize) -> u8 {
        let mut count = 0;
        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }

                let nx = (x as isize + dx).rem_euclid(self.width as isize) as usize;
                let ny = (y as isize + dy).rem_euclid(self.height as isize) as usize;

                if self.grid[nx][ny] {
                    count += 1;
                }
            }
        }
        count
    }

    // 处理点击事件
    fn handle_click(&mut self, pos: Pos2, rect: Rect) {
        let cell_x = ((pos.x - rect.left()) / self.cell_size).floor() as usize;
        let cell_y = ((pos.y - rect.top()) / self.cell_size).floor() as usize;

        if cell_x < self.width && cell_y < self.height {
            self.grid[cell_x][cell_y] ^= true;
        }
    }

    fn draw_grid(&self, painter: &Painter, rect: Rect) {
        // 绘制背景
        painter.rect_filled(rect, 0.0, Color32::from_gray(20));

        // 绘制细胞
        for x in 0..self.width {
            for y in 0..self.height {
                if self.grid[x][y] {
                    let pos = Pos2::new(
                        rect.left() + x as f32 * self.cell_size,
                        rect.top() + y as f32 * self.cell_size,
                    );

                    painter.rect_filled(
                        Rect::from_min_size(pos, Vec2::splat(self.cell_size - 1.0)),
                        2.0,
                        Color32::from_rgb(100, 200, 100),
                    );
                }
            }
        }

        // 绘制网格线
        let stroke = egui::Stroke::new(0.5, Color32::from_gray(50));
        for x in 0..=self.width {
            let x = rect.left() + x as f32 * self.cell_size;
            painter.line_segment(
                [Pos2::new(x, rect.top()), Pos2::new(x, rect.bottom())],
                stroke,
            );
        }
        for y in 0..=self.height {
            let y = rect.top() + y as f32 * self.cell_size;
            painter.line_segment(
                [Pos2::new(rect.left(), y), Pos2::new(rect.right(), y)],
                stroke,
            );
        }
    }
}

impl View for GameOfLifeView {
    fn render(&mut self, _ctx: &egui::Context, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            if ui
                .button(if self.running {
                    "⏸ Pause"
                } else {
                    "▶ Start"
                })
                .clicked()
            {
                self.running ^= true;
            }
            ui.add(
                egui::Slider::new(&mut self.update_interval, 0.05..=1.0)
                    .text("Speed")
                    .suffix("s"),
            );

            if ui.button("🧹 Clear").clicked() {
                self.grid = vec![vec![false; self.height]; self.width];
            }
        });

        // 游戏网格绘制
        let (response, painter) = ui.allocate_painter(
            Vec2::new(
                self.cell_size * self.width as f32,
                self.cell_size * self.height as f32,
            ),
            Sense::click_and_drag(),
        );

        // 处理点击事件
        if response.clicked() {
            if let Some(pos) = response.interact_pointer_pos() {
                self.handle_click(pos, response.rect);
            }
        }
        // 自动更新逻辑
        if self.running
            && self.last_update.elapsed() > Duration::from_secs_f32(self.update_interval)
        {
            self.next_generation();
            self.last_update = Instant::now();
        }

        // 绘制网格
        self.draw_grid(&painter, response.rect);
    }
}
