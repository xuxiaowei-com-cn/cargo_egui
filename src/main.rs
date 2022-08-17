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

        // 参考：https://github.com/emilk/egui/blob/0.17.0/eframe/examples/custom_font.rs

        // 此内容存在 bug，即：运行时闪烁，会从乱码转换到不乱码的状态

        // 从默认字体开始（我们将添加而不是替换它们）。
        let mut fonts = egui::FontDefinitions::default();

        // 安装我自己的字体（也许支持非拉丁字符）。
        // 支持 .ttf 和 .otf 文件。
        // STSONG.TTF 来自 Windows 10 系统的 C:\Windows\Fonts\STSONG.TTF
        fonts.font_data.insert("my_font".to_owned(), egui::FontData::from_static(include_bytes!("STSONG.TTF")));

        // 将我的字体放在首位（最高优先级）用于比例文本：
        fonts.families.entry(egui::FontFamily::Proportional).or_default().insert(0, "my_font".to_owned());

        // 将我的字体作为等宽字体的最后后备：
        fonts.families.entry(egui::FontFamily::Monospace).or_default().push("my_font".to_owned());

        // 告诉 egui 使用这些字体
        // 新字体将在下一帧开始时激活。
        // https://docs.rs/egui/latest/egui/struct.Context.html#method.set_fonts
        ctx.set_fonts(fonts);

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
