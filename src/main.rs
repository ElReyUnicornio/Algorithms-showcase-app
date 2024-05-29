use iced::widget::canvas::{Frame, Geometry, LineDash, Path, Program};
use iced::mouse::Cursor;
use iced::widget::text::LineHeight;
use iced::{Application, Color, Command, Element, Font, Length, Pixels, Point, Rectangle, Renderer, Settings, Theme};
use iced::widget::{button, canvas, column, row, text, text_input, Text};

pub fn main() -> iced::Result {
    MainPage::run(Settings::default())
}

#[derive(Default)]
struct MainPage {
    text: String,
    tree_nodes: String,
    tree: Node,
    insert_text: String,
    insert: i32,
    search_text: String,
    search: i32,
    result: String,
}

#[derive(Debug, Clone)]
enum Message {
    TreeNodes(String),
    Submit,
    Result(String),
    Insert,
    InsertValue(String),
    Search,
    SearchValue(String),
}

//trees
#[derive(Debug)]
struct Node {
    value: i32,
    childs: f32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Default for Node{
    fn default() -> Self {
        Node {value: 0, left: None, right: None, childs: 0.0}
    }
}

impl Node {
    fn new(value: i32) -> Node {
        Node {value: value, left: None, right: None , childs: 0.0}
    }

    fn import(tree_nodes: String) -> Node {
        if tree_nodes.is_empty() {
            return Node::new(0);
        }

        let nodes: Vec<i32> = tree_nodes.split(",").map(|x| x.parse::<i32>().unwrap()).collect();
        let mut root = Node::new(nodes[0]);
        for node in nodes.iter().skip(1) {
            root.insert(*node);
        }
        root
    }

    fn insert (&mut self, value: i32) {
        let node = self;
        if value > node.value {
            node.childs += 1.0;
            if node.right.is_none() {
                node.right = Some(Box::new(Node {value: value, left: None, right: None, childs: 0.0}));
                return;
            } else {
                node.right.as_mut().unwrap().insert(value);
            }
        }

        if value < node.value {
            node.childs += 1.0;
            if node.left.is_none() {
                node.left = Some(Box::new(Node {value: value, left: None, right: None, childs: 0.0}));
                return;
            } else {
                node.left.as_mut().unwrap().insert(value);
            }
        }
    }

    fn search(&self, value: i32) -> bool {
        let node = self;
        let mut found = false;
        if node.value == value {
            return true;
        }
        if node.left.is_some() {
            found = node.left.as_ref().unwrap().search(value);
        }
        if node.right.is_some() {
            found = node.right.as_ref().unwrap().search(value);
        }
        found
    }
}

fn preorder_traversal(node: &Node) -> String{
    let mut res = node.value.to_string() + ",";
    if node.left.is_some() {
        res += &preorder_traversal(node.left.as_ref().unwrap());
    }
    if node.right.is_some() {
        res += &preorder_traversal(node.right.as_ref().unwrap());
    }
    res
}

fn inorder_traversal(node: &Node) -> String {
    let mut res = "".to_string();
    if node.left.is_some() {
        res += &inorder_traversal(node.left.as_ref().unwrap());
    }
    res += &(node.value.to_string() + ",");
    if node.right.is_some() {
        res += &inorder_traversal(node.right.as_ref().unwrap());
    }
    res
}

fn postorder_traversal(node: &Node) -> String {
    let mut res: String = "".to_string();
    if node.left.is_some() {
        res += &postorder_traversal(node.left.as_ref().unwrap());
    }
    if node.right.is_some() {
        res += &postorder_traversal(node.right.as_ref().unwrap());
    }
    res += &(node.value.to_string() + ",");
    res
}

#[test]
fn test_insert() {
    let mut root = Node {value: 1, left: None, right: None, childs: 0.0};
    root.insert(2);
    root.insert(3);
    root.insert(4);
    root.insert(5);
    root.insert(6);
    root.insert(7);
    root.insert(8);
    root.insert(9);

    println!("Preorder traversal: ");
    println!("{}",preorder_traversal(&root));
    println!("Inorder traversal: ");
    println!("{}",inorder_traversal(&root));
    println!("Postorder traversal: ");
    println!("{}",postorder_traversal(&root));
}

//canvas drawings


impl Program<Message> for Node {
    type State = ();

    fn draw(&self, _state: &(), renderer: &Renderer, _theme: &Theme, bounds: Rectangle, _cursor: Cursor) -> Vec<Geometry>{
        let mut frame = Frame::new(renderer, bounds.size());

        fn draw_node(_node: &Node, frame: &mut Frame, _pos: Point, radius: f32) {
            let stroke = canvas::Stroke {
                width: 2.0,
                line_cap: iced::widget::canvas::LineCap::Butt,
                line_join: iced::widget::canvas::LineJoin::Miter,
                style: canvas::Style::Solid(Color::BLACK),
                line_dash: LineDash::default(),
            };

            let mut distance_factor: f32 = _node.childs / 3.0;
            if distance_factor % 1.0 != 0.0 {
                distance_factor += 1.0 - distance_factor % 1.0;
            }

            if _node.left.is_some() {
                let left_pos = Point::new(_pos.x - 50.0 * distance_factor, _pos.y + 50.0);
                let left_line = Path::line(_pos, left_pos);
                frame.stroke(&left_line, stroke.clone());
                draw_node(_node.left.as_ref().unwrap(), frame, left_pos, radius);
            }

            if _node.right.is_some() {
                let right_pos = Point::new(_pos.x + 50.0 * distance_factor, _pos.y + 50.0);
                let right_line = Path::line(_pos, right_pos);
                frame.stroke(&right_line, stroke.clone());
                draw_node(_node.right.as_ref().unwrap(), frame, right_pos, radius);
            }

            let border = Path::circle(_pos, radius + 2.0);
            frame.fill(&border, Color::from_rgb(1.0, 0.0, 0.0));

            let circle = Path::circle(_pos, radius);
            frame.fill(&circle, Color::from_rgb(0.0, 0.0, 1.0));

            let value = canvas::Text {
                content: _node.value.to_string(),
                position: _pos,
                size: Pixels(20.0),
                color: Color::WHITE,
                horizontal_alignment: iced::alignment::Horizontal::Center,
                vertical_alignment: iced::alignment::Vertical::Center,
                line_height: LineHeight::default(),
                font: Font::default(),
                shaping: Default::default(),
            };
            frame.fill_text(value);

            
        }

        let node_radius = 20.0;
        let frame_center_x = frame.center().x.clone();
        draw_node(&self, &mut frame, Point::new(frame_center_x, 0.0 + node_radius + 5.0), node_radius);

        vec![frame.into_geometry()]
    }
}

//Aplication screens
impl Application for MainPage {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (MainPage, Command<Message>) {
        (MainPage::default(), Command::none())
    }

    fn title(&self) -> String {
        String::from("Tree Visualizer")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::TreeNodes(value) => self.text = value,
            Message::Submit => {
                self.tree_nodes = self.text.clone();
                self.tree = Node::import(self.tree_nodes.clone());
            },
            Message::Result(value) => self.result = value,
            Message::InsertValue(value) => self.insert_text = value,
            Message::Insert => {
                self.insert = self.insert_text.parse::<i32>().unwrap();
                self.tree.insert(self.insert);
            },
            Message::SearchValue(value) => self.search_text = value,
            Message::Search => {
                self.search = self.search_text.parse::<i32>().unwrap();
                self.result = self.tree.search(self.search).to_string();
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let label_tree: Text = text("Enter your tree:").size(16);
        let input_tree: text_input::TextInput<Message, Theme, Renderer> = text_input::TextInput::new("ej. 1,2,3,4,5,...", &self.text).on_input(Message::TreeNodes).on_submit(Message::Submit);
        let submit_button: button::Button<Message, Theme, Renderer>= button("Submit").on_press(Message::Submit);
        let tree_input = row![label_tree, input_tree, submit_button].spacing(10);

        let label_insert: Text = text("Enter a value to insert:").size(16);
        let input_insert: text_input::TextInput<Message, Theme, Renderer> = text_input::TextInput::new("ej. 23", &self.insert_text).on_input(Message::InsertValue).on_submit(Message::Insert);
        let submit_insert: button::Button<Message, Theme, Renderer> = button("Insert").on_press(Message::Insert);
        let insert_input: iced::widget::Row<Message, Theme, Renderer> = row![label_insert, input_insert, submit_insert].spacing(10);

        let label_search: Text = text("Enter a value to search:").size(16);
        let input_search: text_input::TextInput<Message, Theme, Renderer> = text_input::TextInput::new("ej. 23", &self.search_text).on_input(Message::SearchValue).on_submit(Message::Search);
        let submit_search: button::Button<Message, Theme, Renderer> = button("Search").on_press(Message::Search);
        let search_input: iced::widget::Row<Message, Theme, Renderer> = row![label_search, input_search, submit_search].spacing(10);

        let button_inorder = button("Inorder").on_press(Message::Result(inorder_traversal(&self.tree)));
        let button_preorder = button("Preorder").on_press(Message::Result(preorder_traversal(&self.tree)));
        let button_postorder = button("Postorder").on_press(Message::Result(postorder_traversal(&self.tree)));
        let traversal_inputs = row![button_inorder, button_preorder, button_postorder].spacing(10);


        let result: Text = text(&self.result).size(20);
        let screen: Element<Message> = canvas::Canvas::new(&self.tree).width(Length::Fill).height(Length::Fill).into();
        println!("{}", self.tree.childs);

        let interface = column![tree_input, insert_input, search_input, traversal_inputs, result, screen].padding(20).align_items(iced::Alignment::Center).spacing(10);
        interface.into()
    }
    }