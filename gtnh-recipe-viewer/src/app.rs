use egui::Color32;
use egui_extras::{Size, StripBuilder};
use std::path::PathBuf;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct GtnhRecipeViewerApp {
    label: String,

    filename: Option<PathBuf>,

    #[serde(skip)]
    recipes_json: Option<gtnh_recipe_lib::types::json::JsonFormat>,
    #[serde(skip)]
    search_results: Vec<(
        String,
        gtnh_recipe_lib::types::gregtech_recipe::GregtechRecipe,
    )>,
    #[serde(skip)]
    selection: std::collections::HashSet<usize>,
}

impl Default for GtnhRecipeViewerApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            filename: None,
            recipes_json: None,
            search_results: vec![],
            selection: Default::default(),
        }
    }
}

impl GtnhRecipeViewerApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }

    fn search(&mut self) {
        if let Some(recipes) = &self.recipes_json {
            //println!("searching...");
            self.search_results = recipes.search(&self.label);
            //dbg!(&self.search_results);
        }
    }

    fn details_windows(&mut self, ui: &mut egui::Ui) {
        for selection in &self.selection {
            let recipe = self.search_results.iter().nth(*selection).unwrap();
            ui.push_id(*selection, |ui| {
                egui::Window::new(format!("Recipe Details (#{})", selection))
                    .default_width(400.0)
                    .show(ui.ctx(), |ui| {
                        ui.heading(format!("{}", recipe.0));
                        ui.label(format!("{}s, {}EU/t", recipe.1.duration, recipe.1.eut));
                        ui.heading("Item Inputs:");
                        for item in &recipe.1.item_inputs {
                            ui.label(format!("{}", item));
                        }
                        ui.heading("Item Outputs:");
                        for item in &recipe.1.item_outputs {
                            ui.label(format!("{}", item));
                        }

                        ui.heading("Fluid Inputs:");
                        for fluid in &recipe.1.fluid_inputs {
                            ui.label(format!("{}", fluid));
                        }
                        ui.heading("Fluid Outputs:");
                        for fluid in &recipe.1.fluid_outputs {
                            ui.label(format!("{}", fluid));
                        }

                        ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                            ui.small(format!("{}", recipe.1));
                        });
                    });
            });
        }
    }

    fn table_ui(&mut self, ui: &mut egui::Ui, reset: bool) {
        use egui_extras::{Column, TableBuilder};

        let text_height = egui::TextStyle::Body
            .resolve(ui.style())
            .size
            .max(ui.spacing().interact_size.y);

        let available_height = ui.available_height();
        let mut table = TableBuilder::new(ui)
            .striped(true)
            .resizable(true)
            .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
            .column(Column::auto())
            .column(Column::remainder().clip(true))
            .column(Column::remainder().clip(true))
            .column(Column::remainder().clip(true))
            .column(Column::remainder().clip(true))
            .column(Column::remainder().clip(true))
            .min_scrolled_height(0.0)
            .max_scroll_height(available_height);

        table = table.sense(egui::Sense::click());

        if reset {
            table.reset();
        }

        table
            .header(20.0, |mut header| {
                header.col(|ui| {
                    ui.strong("Row");
                });
                header.col(|ui| {
                    ui.strong("Machine");
                });
                header.col(|ui| {
                    ui.strong("Items (Input)");
                });
                header.col(|ui| {
                    ui.strong("Fluids (Input)");
                });
                header.col(|ui| {
                    ui.strong("Items (Output)");
                });
                header.col(|ui| {
                    ui.strong("Fluids (Output)");
                });
            })
            .body(|mut body| {
                body.rows(text_height, self.search_results.len(), |mut row| {
                    let row_index = row.index();
                    let search_result = self.search_results.iter().nth(row_index).unwrap();

                    row.set_selected(self.selection.contains(&row_index));

                    row.col(|ui| {
                        ui.label(row_index.to_string());
                    });
                    row.col(|ui| {
                        ui.label(format!("{}", search_result.0));
                    });
                    row.col(|ui| {
                        ui.label(
                            search_result
                                .1
                                .item_inputs
                                .iter()
                                .map(|item| format!("{}", item))
                                .collect::<Vec<String>>()
                                .join(" + "),
                        );
                    });
                    row.col(|ui| {
                        ui.label(
                            search_result
                                .1
                                .fluid_inputs
                                .iter()
                                .map(|fluid| format!("{}", fluid))
                                .collect::<Vec<String>>()
                                .join(" + "),
                        );
                    });
                    row.col(|ui| {
                        ui.label(
                            search_result
                                .1
                                .item_outputs
                                .iter()
                                .map(|item| format!("{}", item))
                                .collect::<Vec<String>>()
                                .join(" + "),
                        );
                    });
                    row.col(|ui| {
                        ui.label(
                            search_result
                                .1
                                .fluid_outputs
                                .iter()
                                .map(|fluid| format!("{}", fluid))
                                .collect::<Vec<String>>()
                                .join(" + "),
                        );
                    });

                    self.toggle_row_selection(row_index, &row.response());
                });
            });
    }

    fn toggle_row_selection(&mut self, row_index: usize, row_response: &egui::Response) {
        if row_response.clicked() {
            if self.selection.contains(&row_index) {
                self.selection.remove(&row_index);
            } else {
                self.selection.insert(row_index);
            }
        }
    }
}

impl eframe::App for GtnhRecipeViewerApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        if self.filename == None {
            let path = std::env::current_dir().unwrap();

            let res = rfd::FileDialog::new()
                .set_title("Select recipes.json please")
                .add_filter("json", &["json", "json"])
                .set_directory(&path)
                .pick_file();

            println!("The user choose: {:#?}", res);
            self.filename = res;
        }

        if let None = self.recipes_json {
            if let Some(path) = &self.filename {
                self.recipes_json = Some(gtnh_recipe_lib::load_file(path));
            }
        }

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Open new file").clicked() {
                            self.filename = None;
                            self.recipes_json = None;
                        }
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("GTNH recipe viewer");

            ui.horizontal(|ui| {
                ui.label("Search: ");
                let textedit_response = ui.text_edit_singleline(&mut self.label);
                if textedit_response.lost_focus() {
                    textedit_response.ctx.input(|i| {
                        if i.key_pressed(egui::Key::Enter) {
                            self.search();
                        }
                    });
                }
            });

            if ui.button("Search").clicked() {
                //search recipes
                self.search();
            }

            ui.separator();

            StripBuilder::new(ui)
                .size(Size::remainder().at_least(100.0)) // for the table
                .size(Size::exact(50.0)) // for the source code link
                .vertical(|mut strip| {
                    strip.cell(|ui| {
                        // Results table
                        if !&self.search_results.is_empty() {
                            egui::ScrollArea::horizontal().show(ui, |ui| {
                                self.table_ui(ui, false);
                            });
                        } else {
                            ui.label("Search something!");
                        }

                        if !&self.selection.is_empty() {
                            self.details_windows(ui);
                        }

                        ui.separator();

                        if let Some(recipes) = &self.recipes_json {
                            ui.label(format!(
                                "Total recipes loaded: {}, search results: {}",
                                recipes.get_recipe_count(),
                                self.search_results.len()
                            ));
                        } else {
                            ui.colored_label(Color32::RED, "No recipes loaded!");
                        }
                    });
                    strip.cell(|ui| {
                        ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                            powered_by_egui_and_eframe(ui);

                            ui.add(egui::github_link_file!(
                                "https://github.com/konradmoesch/gtnh_recipe_calculator/blob/main/",
                                "Source code."
                            ));
                            egui::warn_if_debug_build(ui);
                        });
                    });
                });
        });
    }

    /// Called by the framework to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}
