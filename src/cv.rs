use egui::{special_emojis::GITHUB, RichText, Vec2};

use crate::{common::Game, life::Life, maze::Maze, pong::Pong, snake::Snake};

#[derive(Default)]
pub struct Cv {
    snake: WindowedGame<Snake>,
    pong: WindowedGame<Pong>,
    maze: WindowedGame<Maze>,
    life: WindowedGame<Life>,
}

#[derive(Default)]
struct WindowedGame<G: Game> {
    /// When `true` it means the window is currently opened.
    opened: bool,
    /// The internal states of the game.
    game: G,

    ///Size of the window
    size: Vec2,
}

impl<G: Game> WindowedGame<G> {
    pub fn clicked(&mut self) {
        self.opened = !self.opened;
    }

    pub fn handle(&mut self, ctx: &egui::Context) {
        if self.opened {
            self.game.update(ctx);
            egui::Window::new(G::name())
                .open(&mut self.opened)
                .show(ctx, |ui| {
                    ui.hyperlink_to(format!("{GITHUB} Source code"), G::github());
                    if self.size != ui.available_size() {
                        self.game.resize(ui);
                        self.size = ui.available_size();
                    }
                    self.game.draw(ctx, ui);
                });
        }
    }
}

impl Cv {
    pub fn ui(&mut self, ctx: &egui::Context) {
        egui::SidePanel::left("left_panel")
            .resizable(true)
            .default_width(500.0)
            .width_range(80.0..=200.0)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading("At a glance");
                });
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.label(RichText::new("LANGUAGES SPOKEN:").strong());
                    ui.label("French (native)");
                    ui.label("Italian (native)");
                    ui.label("English (bilingual)");
                    ui.label("Japanese (intermadiary)");

                    ui.label("\n");
                    ui.separator();

                    ui.label(RichText::new("PROGRAMMING LANGUAGES:").strong());
                    ui.label("Rust");
                    ui.label("SQL");
                    ui.label("HTML");
                    ui.label("CSS");
                    ui.label("Python");

                    ui.label("\n");
                    ui.separator();

                    ui.label(RichText::new("FRAMEWORKS:").strong());
                    ui.label("Actix-web");
                    ui.label("Reqwest");
                    ui.label("Clap");
                    ui.label("Tokio");
                    ui.label("Sqlx");
                    ui.label("Axum");

                    ui.label("\n");
                    ui.separator();

                    ui.label(RichText::new("TOOLS:").strong());
                    ui.label("Git");
                    ui.label("GitHub");
                    ui.label("PosgreSQL");
                    ui.label("Docker");
                    ui.label("Bash");
                    ui.label("Curl");

                    ui.label("\n");
                    ui.separator();

                    ui.label(RichText::new("SOCIAL MEDIA:").strong());
                    ui.hyperlink_to(
                        "LinkedIn",
                        "https://www.linkedin.com/in/luna-ferraraccio-01553a110/",
                    );
                    ui.hyperlink_to("GitHub", "https://github.com/NoodleSamaChan");

                    ui.label("\n");
                    ui.separator();

                    ui.label(RichText::new("REPOS CONTRIBUTIONS:").strong());
                    ui.hyperlink_to(
                        "Top 10 contributor of Meilisearch-Rust",
                        "https://github.com/meilisearch/meilisearch-rust",
                    );
                    ui.hyperlink_to("Lasr", "https://github.com/versatus/lasr");

                    ui.label("\n");
                    ui.separator();

                    ui.label(RichText::new("ANYTHING ELSE?").strong());
                    ui.label("I am also very happy to be a part of the team of Rust Lille, which helps bring together the French speaking Rust community in the North of France (for now!).");
                    ui.hyperlink_to(
                        "Have a look at our website!",
                        "https://rustlille.fr",
                    );
                });
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                // We want the `ScrollArea` to use 100% of the vertical width to make the scrolling bar stick to the right.
                let max_width = ui.available_width();
                ui.set_min_width(max_width);

                ui.vertical_centered(|ui| {
                    let max_width = ui.available_width();
                    // The text shouldn't be larger than the available size
                    ui.set_max_width(max_width.min(1100.));

                    ui.horizontal_wrapped(|ui| {
                        // Trick so we don't have to add spaces in the text below:
                        let width = ui.fonts(|f|f.glyph_width(&egui::TextStyle::Body.resolve(ui.style()), ' '));
                        ui.spacing_mut().item_spacing.x = width;

                        ui.separator();
                        ui.add_space(20.);
                        ui.heading(RichText::new("Hello, my name is Luna Ferraraccio").strong().underline());
                        ui.label("\n");
                        ui.label("\n");
                        ui.label("Junior Back-end developer seeking new opportunities. Before becoming a Rust back-end developer, I actualy used to work in communication.");
                        ui.label("\n");
                        ui.label("I worked for ");
                        ui.label(RichText::new("6 years at Ubisoft,").strong());
                        ui.label("the video game developer, in various community and communication roles.");
                        ui.label("\n");
                        ui.label("I was a");
                        ui.label(RichText::new("Community Manager, a Listening Coordinator, and finally the team lead of the APAC community team.").strong());
                        ui.label("\n");
                        ui.label("I discovered the world of development and coding during my work at ");
                        ui.label(RichText::new("Meilisearch as a Lead Community Developer in the DevRel department.").strong());
                        ui.label("\n");
                        ui.label("I fell in love with coding and decided to start a new adventure in my professional life.\n");
                        ui.label("\n");
                        ui.label("\n");
                        ui.separator();

                        ui.add_space(20.);
                        ui.heading(RichText::new("Concerning my Education:").strong().underline());
                        ui.label("\n");
                        ui.label("\n");
                        ui.label("I studied");
                        ui.label(RichText::new("English language, literature, and history").strong());
                        ui.label("at the university of Le Mans, France, for my Bachelor.");
                        ui.label("\n");
                        ui.label("Then, I also studied");
                        ui.label(RichText::new("Culture, Communication, and globalization (with a minor in gender studies)").strong());
                        ui.label("at the university of Aalborg, Denmark, for my Masters.");
                        ui.label("\n");
                        ui.label("As part of my continued evolution at Ubisoft, I also benefited from a formal");
                        ui.label(RichText::new("Project Management training, as well as a People Management training").strong());
                        ui.label("\n");
                        ui.label("Finally, I started learning coding");
                        ui.label(RichText::new("from July 2023.").strong());
                        ui.label("I initially started with Python, but quickly moved to ");
                        ui.label(RichText::new("Rust.").underline().strong());
                        ui.label("During that time, I've tackled a number of projects in order learn as much as I could about coding, and Rust in particular.\n");
                        ui.label("\n");
                        ui.label("\n");
                        ui.separator();

                        ui.add_space(20.);
                        ui.heading(RichText::new("My notable projects:").strong().underline());
                        ui.label("\n");
                        ui.label("\n");
                        ui.hyperlink_to("LairBnB", "https://github.com/NoodleSamaChan/lairbnb_rs");
                        ui.label("\n");
                        ui.label("My very own version of the famous AirBnB website. In this project, I learned how to manage databases and a REST API.");
                        ui.label("\n");
                        ui.label(RichText::new("Framework used: Tokio, Reqwest, Actix-web, Sqlx").strong());
                        ui.label("\n");
                        ui.label("\n");

                        ui.label("Newsletter");
                        ui.hyperlink_to(" Zero To Production (L. Palmieri)", "https://www.zero2prod.com/index.html");
                        ui.label("\n");
                        ui.label("Palmieri is well known for his educational work in the Rust community and allowed me to create my own Newsletter program for my own website and learn a lot about coding best practices. ");
                        ui.label("\n");
                        ui.label(RichText::new("Framework used: Tokio, Reqwest, Actix-web, Sqlx").strong());
                        ui.label("\n");
                        ui.label("\n");

                        ui.label(RichText::new("Game projects:").strong().underline());
                        ui.label("\n");
                        ui.label("I have produced a series of projects around well known, old style games which allowed me to get much more familiar with programming logic, the language of Rust, project management, code management. You'll be able to try all of these games on this website. Please feel free to try them and have fun. :) I'll link the GitHub repos below if you're curious.");
                        ui.label("\n");
                        ui.label("\n");

                        if ui.button("Snake").clicked() {
                            self.snake.clicked();
                            
                        };
                        ui.label("\n");
                        ui.label("I've implemented a number of functionalities with the Snake project. There's a classic mode of course, but you can setup a number of options should you wish to.");
                        ui.label("\n");
                        ui.label(RichText::new("Ghost mode: ").strong());
                        ui.label("will allow your snake to safely go through walls");
                        ui.label("\n");
                        ui.label(RichText::new("Two players mode: ").strong());
                        ui.label("will create a second snake, if you wish to share a game locally with someone else.");
                        ui.label("\n");
                        ui.label(RichText::new("Bad berry mode: ").strong());
                        ui.label("will generate a 'bad berry' that, if eaten, give you a disadvantage that will only be cancelled by eating a second bad berry.");
                        ui.label("\n");
                        ui.label(RichText::new("Change the colour: ").strong());
                        ui.label("personalise the colours of whichever element of the game you want");
                        ui.label("\n");
                        ui.label(RichText::new("Snake size: ").strong());
                        ui.label("personalise the size of your snake at the start of the game");
                        ui.label("\n");
                        ui.label(RichText::new("Snake speed: ").strong());
                        ui.label("set the speed of your snake to make the game more or less challenging.");
                        ui.label("\n");
                        ui.label("\n");

                        if ui.button("Pong").clicked() {
                            self.pong.clicked();
                        };
                        ui.label("\n");
                        ui.label(RichText::new("Speed of the pongs and the ball: ").strong());
                        ui.label("set the respective speeds of the pongs and the ball to make the game more or less challenging.");
                        ui.label("\n");
                        ui.label(RichText::new("Change the colour: ").strong());
                        ui.label("personalise the colours of whichever element of the game you want");
                        ui.label("\n");
                        ui.label("\n");

                        if ui.button("Maze").clicked() {
                            self.maze.clicked();
                        };
                        ui.label("\n");
                        ui.label("Have fun finding your way out of the maze!");
                        ui.label("\n");
                        ui.label(RichText::new("Change the colour: ").strong());
                        ui.label("personalise the colours of whichever element of the game you want");
                        ui.label("\n");
                        ui.label("\n");

                        if ui.button("Game of Life").clicked() {
                            self.life.clicked();
                        };
                        ui.label("\n");
                        ui.label("The famous game of life");
                        ui.label("\n");
                        ui.label(RichText::new("Change the colour: ").strong());
                        ui.label("personalise the colours of whichever element of the game you want");
                        ui.label("\n");
                        ui.label("\n");

                        ui.label(RichText::new("This resume :)").strong());
                        ui.label("\n");
                        ui.hyperlink_to("GitHub Repo", "https://github.com/NoodleSamaChan/resume");
                        ui.label("\n");
                        ui.label("I have coded my resume entirely in eGui, which necessitated to understand the best way to transpose the code of all my games with this graphic library.");
                        ui.label("\n");
                        ui.label("\n");
                    });
                });
            });
        });
        // Draw all the game at the end to be sure they're drawn over the resume
        self.snake.handle(ctx);
        self.pong.handle(ctx);
        self.maze.handle(ctx);
        self.life.handle(ctx);
    }
}
