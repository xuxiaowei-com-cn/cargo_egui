// 在发行版中隐藏 Windows 上的控制台窗口
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;

fn main() {
    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "我的 egui 应用",
        options,
        Box::new(|_cc| Box::new(People::default())),
    );
}

// 人
struct People {
    // 姓名
    name: String,
    // 年龄
    age: u32,
}

// 默认值
impl Default for People {
    fn default() -> Self {
        Self {
            name: "Arthur".to_owned(),
            age: 42,
        }
    }
}

//
impl eframe::App for People {

    // 每次 UI 需要重新绘制时调用
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // 中央面板
        egui::CentralPanel::default().show(ctx, |ui| {

            // 显示大文本
            ui.heading("我的 egui 应用程序");

            // 使用水平布局启动 ui
            ui.horizontal(|ui| {
                // 显示一些文字
                ui.label("你的名字: ");
                // 不允许换行符，按下回车键将导致失去焦点
                ui.text_edit_singleline(&mut self.name);
            });

            // 滑块
            ui.add(egui::Slider::new(&mut self.age, 0..=120).text("岁"));

            // 按钮
            if ui.button("每年点击一次").clicked() {
                self.age += 1;
            }

            // 显示一些文字
            ui.label(format!("你好 '{}', 年龄 {}", self.name, self.age));
        });
    }
}
