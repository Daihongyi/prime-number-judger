use eframe::egui;

fn main() {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([480.0, 360.0]),
        ..Default::default()
    };

    let _ = eframe::run_native(
        "Prime Number Judger",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_visuals(egui::Visuals::light());
            Ok(Box::new(PrimeNumberJudger::default()))
        }),
    );
}

#[derive(Default)]
pub struct PrimeNumberJudger {
    input_number: i64,
    previous_input: i64,    // 跟踪上一次判断的数值
    is_prime: bool,
    factors: Vec<i64>,
    unknown_type: bool,
    has_judged: bool,       // 是否已点击判断按钮
}

impl PrimeNumberJudger {
    fn judge(&mut self) {
        self.factors.clear();
        self.is_prime = false;
        self.unknown_type = false;

        let n = self.input_number;
        
        if n <= 1 {
            self.unknown_type = true;
        } else {
            // 检查因数
            let mut i = 2;
            while i * i <= n {
                if n % i == 0 {
                    self.factors.push(i);
                    let complement = n / i;
                    if complement != i {
                        self.factors.push(complement);
                    }
                }
                i += 1;
            }

            self.factors.sort();
            self.factors.dedup();
            self.is_prime = self.factors.is_empty();
        }
        
        self.has_judged = true;
        self.previous_input = self.input_number;
    }
}

impl eframe::App for PrimeNumberJudger {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // 检测输入变化时重置判断状态
        if self.input_number != self.previous_input {
            self.has_judged = false;
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("Prime Number Judger");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                    if ui.button("Dark").clicked() {
                        ctx.set_visuals(egui::Visuals::dark());
                    }
                    if ui.button("Light").clicked() {
                        ctx.set_visuals(egui::Visuals::light());
                    }
                });
            });

            ui.horizontal(|ui| {
                ui.label("Enter number:");
                ui.add(egui::DragValue::new(&mut self.input_number).speed(1));
            });

            if ui.button("Judge").clicked() {
                self.judge();
            }

            ui.separator();

            if self.has_judged {
                if self.unknown_type {
                    ui.label("❌ Number must be greater than 1");
                } else if self.is_prime {
                    ui.label(format!("✅ {} is a prime number", self.input_number));
                } else {
                    ui.label(format!("❌ {} is a composite number", self.input_number));
                    ui.label("Factors found:");
                    ui.label(self.factors.iter().map(|n| n.to_string()).collect::<Vec<_>>().join(", "));
                }
            } else {
                ui.label("Click 'Judge' to check the number");
            }
        });
    }
}