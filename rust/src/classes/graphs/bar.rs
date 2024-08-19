use godot::classes::{
    BoxMesh, CollisionShape2D, Gradient, GradientTexture1D, INode2D,
    IStaticBody2D, InputEvent, Label, MeshInstance2D, Node2D, RectangleShape2D,
    Resource, StaticBody2D,
};
use godot::global::HorizontalAlignment;
use godot::global::Key;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct BarGraph {
    base: Base<Node2D>,
    #[export]
    offset: f32,
    #[export]
    spacing: f32,
    #[export]
    bar_details: Array<Gd<BarDetails>>,
}

#[godot_api]
impl INode2D for BarGraph {
    fn init(base: Base<Node2D>) -> Self {
        Self {
            base,
            offset: 0.,
            spacing: 50.,
            bar_details: Array::new(),
        }
    }

    fn ready(&mut self) {
        if self.bar_details.len() > 25 {
            godot_error!("Bar Count cannot exceed 25!");
            return;
        }
        let bar_details = self.bar_details.clone();
        for (idx, mut details) in bar_details.iter_shared().enumerate() {
            self.create_bar(idx as u32, details);
        }
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        let change = 0.25;
        #[cfg(debug_assertions)]
        if event.is_action_pressed("DebugUp".into()) {
            let children = self.base_mut().get_children();
            for child in children.iter_shared() {
                let child = child.try_cast::<Bar>().unwrap();
                Bar::increase_height(child, change);
            }
        }
        #[cfg(debug_assertions)]
        if event.is_action_pressed("DebugDown".into()) {
            let children = self.base_mut().get_children();
            for child in children.iter_shared() {
                let child = child.try_cast::<Bar>().unwrap();
                Bar::decrease_height(child, change);
            }
        }
    }
}

impl BarGraph {
    fn create_bar(&mut self, index: u32, mut details: Gd<BarDetails>) {
        let Some(graph) = self.base().get_parent() else {
            godot_error!("Graph parent did not exist");
            return;
        };

        let Ok(graph) = graph.try_cast::<Node2D>() else {
            godot_error!("Parent was not Node2D");
            return;
        };

        let graph_scale = graph.get_scale();

        let size = graph_scale * Vector2::new(1000., 100.);

        let positions = size.x / self.bar_details.len() as f32;

        let height: f32 = details.call("get_height".into(), &[]).to();
        let name: String = details.call("get_name".into(), &[]).to();

        let mut bar =
            Bar::create_with_height_index_and_name(index, height, &name);

        {
            let graph_transform = self.base().get_transform();
            let mut bar_transform = bar.get_transform();
            bar_transform.origin = graph_transform.origin;
            bar_transform.origin.y -= 14.0;
            bar.set_transform(bar_transform);
        }

        let half_spacing = self.spacing / 2.;

        bar.translate(Vector2::new(
            -400.
                + self.offset
                + (positions * index as f32)
                + (self.spacing * index as f32),
            0.,
        ));

        self.base_mut().add_child(bar.clone());
        let Some(node) = bar.get_child(0) else {
            godot_error!("Could not get first child of Bar");
            return;
        };
        let mut node = node.try_cast::<StaticBody2D>().unwrap();
        let scale = node.get_scale();
        // let scale = bar.get_scale();
        if self.bar_details.len() > 10 {
            node.set_scale(
                scale
                    + Vector2::new(
                        2. / (self.bar_details.len() as f32 - 10.),
                        0.,
                    ),
            );
        } else {
            node.set_scale(scale + Vector2::new(2., 0.));
        }
    }
}

#[derive(GodotClass)]
#[class(tool, init, base=Resource)]
struct BarDetails {
    #[export]
    name: GString,
    #[export]
    height: f32,
    base: Base<Resource>,
}

#[derive(GodotClass)]
#[class(base=StaticBody2D)]
pub struct Bar {
    base: Base<StaticBody2D>,
    name: String,
    index: u32,
    height: f32,
}

#[godot_api]
impl IStaticBody2D for Bar {
    fn init(base: Base<StaticBody2D>) -> Self {
        Self {
            base,
            name: "".into(),
            index: 0,
            height: 1.,
        }
    }

    fn ready(&mut self) {
        let mut container = StaticBody2D::new_alloc();

        let mut coll_shape = CollisionShape2D::new_alloc();
        {
            let mut shape = RectangleShape2D::new_gd();
            shape.set_size(Vector2::new(25., 25.));
            coll_shape.set_shape(shape);
        }

        let mut mesh_instance = MeshInstance2D::new_alloc();
        {
            mesh_instance.set_scale(Vector2::new(25.0, 25.0));
            let mesh = BoxMesh::new_gd();
            let mut texture = GradientTexture1D::new_gd();
            let mut gradient = Gradient::new_gd();
            let mut colours = PackedColorArray::new();
            for _ in 0..2 {
                colours.push(Color {
                    r: 0.,
                    g: 0.,
                    b: 0.,
                    a: 1.,
                });
            }
            gradient.set_colors(colours);
            texture.set_gradient(gradient);

            mesh_instance.set_mesh(mesh);
            mesh_instance.set_texture(texture);
        }

        let mut text = Label::new_alloc();
        text.set_text(self.name.clone().into());
        // let transform = text.get_transform();
        text.set_horizontal_alignment(HorizontalAlignment::CENTER);
        text.set_anchor_and_offset(Side::LEFT, 0.5, -29.);
        text.set_anchor_and_offset(Side::TOP, 0., 0.);
        text.set_anchor_and_offset(Side::RIGHT, 0.5, 29.);
        text.set_anchor_and_offset(Side::BOTTOM, 0., 23.);

        container.add_child(coll_shape);
        container.add_child(mesh_instance);
        // self.base_mut().add_child(coll_shape);
        // self.base_mut().add_child(mesh_instance);

        let scale = self.base().get_scale();
        let height = self.height;
        container.set_scale(scale + Vector2::new(0.0, height));
        container.translate(Vector2::new(0.0, height / -0.08));

        self.base_mut().add_child(container);
        self.base_mut().add_child(text);
    }

    fn process(&mut self, _delta: f64) {
        let input = Input::singleton();
        let key = match self.index {
            0 => Key::KEY_1,
            1 => Key::KEY_2,
            2 => Key::KEY_3,
            3 => Key::KEY_4,
            4 => Key::KEY_5,
            5 => Key::KEY_6,
            6 => Key::KEY_7,
            7 => Key::KEY_8,
            8 => Key::KEY_9,
            9 => Key::KEY_0,
            _ => Key::HOME,
        };
        if input.is_key_pressed(Key::Q) {
            if input.is_key_pressed(key) {
                Bar::increase_height(self.to_gd(), 0.25);
            }
        } else if input.is_key_pressed(key) {
            Bar::decrease_height(self.to_gd(), 0.25);
        }
    }
}

#[godot_api]
impl Bar {
    #[func]
    fn increase_height(mut object: Gd<Self>, height: f32) {
        let Some(node) = object.get_child(0) else {
            godot_error!("Could not get first child of Bar");
            return;
        };
        let mut node = node.try_cast::<StaticBody2D>().unwrap();
        let scale = node.get_scale();
        node.set_scale(scale + Vector2::new(0.0, height));
        node.translate(Vector2::new(0.0, height / -0.08));
    }

    #[func]
    fn decrease_height(mut object: Gd<Self>, height: f32) {
        let Some(node) = object.get_child(0) else {
            godot_error!("Could not get first child of Bar");
            return;
        };
        let mut node = node.try_cast::<StaticBody2D>().unwrap();
        let scale = node.get_scale();
        node.set_scale(scale + Vector2::new(0.0, -height));
        node.translate(Vector2::new(0.0, -height / -0.08));
    }

    fn create_with_height_and_index(index: u32, height: f32) -> Gd<Self> {
        Gd::from_init_fn(|base| Self {
            base,
            height,
            index,
            name: "".into(),
        })
    }

    fn create_with_height_index_and_name(
        index: u32,
        height: f32,
        name: &str,
    ) -> Gd<Self> {
        Gd::from_init_fn(|base| Self {
            base,
            height,
            index,
            name: name.into(),
        })
    }
}
