use std::path::Path;
use relm4::prelude::*;
use niri_config::Config;

struct App {
    pub config: Config
}

impl SimpleComponent for App {
    type Input = ();
    type Output = ();
    type Init = &'static str;
    type Root = gtk::Window;
    type Widgets = (gtk::Stack, gtk::StackSidebar);

    fn init_root() -> Self::Root {
        gtk::Window::builder()
            .title("NiGuiRi")
            .build()
    }
    fn init (
        init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>
    ) -> ComponentParts<Self> {
        let model = Self {
            config: Config::load(Path::new(init)).unwrap_or_default()
        };
        
        println!("{:?}", model.config);

        let stack = gtk::Stack::builder().build();
        let sidebar = gtk::StackSidebar::builder().build();
        sidebar.set_stack(&stack);

        let widgets = (
            stack,
            sidebar
        );

        ComponentParts {
            model,
            widgets,
        }
    }
}

fn main() {
    let app = RelmApp::new("au.alysia.niguiri");
    app.run::<App>("testing/niri-test.kdl");
}