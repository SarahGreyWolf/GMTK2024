use godot::classes::{
    BoxMesh, CollisionShape2D, Gradient, GradientTexture1D, INode2D,
    IStaticBody2D, InputEvent, MeshInstance2D, Node2D, RectangleShape2D,
    StaticBody2D,
};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct BarGraph {
    base: Base<Node2D>,
    #[export]
    spacing: f32,
    #[export]
    bar_heights: Array<f32>,
}

#[godot_api]
impl INode2D for BarGraph {
    fn init(base: Base<Node2D>) -> Self {
        Self {
            base,
            spacing: 35.,
            bar_heights: Array::new(),
        }
    }

    fn ready(&mut self) {
        if self.bar_heights.len() > 25 {
            godot_error!("Bar Count cannot exceed 25!");
            return;
        }
        let bar_heights = self.bar_heights.clone();
        for (idx, height) in bar_heights.iter_shared().enumerate() {
            self.create_bar(idx as u32, height);
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
    fn create_bar(&mut self, index: u32, height: f32) {
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

        let positions = size.x / self.bar_heights.len() as f32;

        let mut bar = Bar::create_with_height_and_index(index, height);

        {
            let graph_transform = self.base().get_transform();
            let mut bar_transform = bar.get_transform();
            bar_transform.origin = graph_transform.origin;
            bar_transform.origin.y -= 14.0;
            bar.set_transform(bar_transform);
        }

        let half_spacing = self.spacing / 2.;

        bar.translate(Vector2::new(
            (-400. - half_spacing) + (positions * index as f32) + half_spacing,
            0.,
        ));
        let scale = bar.get_scale();
        if self.bar_heights.len() > 10 {
            bar.set_scale(
                scale
                    + Vector2::new(
                        2. / (self.bar_heights.len() as f32 - 10.),
                        0.,
                    ),
            );
        } else {
            bar.set_scale(scale + Vector2::new(2., 0.));
        }

        self.base_mut().add_child(bar.clone());
    }
}

#[derive(GodotClass)]
#[class(base=StaticBody2D)]
pub struct Bar {
    base: Base<StaticBody2D>,
    index: u32,
    height: f32,
}

#[godot_api]
impl IStaticBody2D for Bar {
    fn init(base: Base<StaticBody2D>) -> Self {
        Self {
            base,
            index: 0,
            height: 1.,
        }
    }

    fn ready(&mut self) {
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

        self.base_mut().add_child(coll_shape.clone());
        self.base_mut().add_child(mesh_instance.clone());
        let scale = self.base().get_scale();
        let height = self.height;
        self.base_mut().set_scale(scale + Vector2::new(0.0, height));
        self.base_mut().translate(Vector2::new(0.0, height / -0.08));
    }
}

#[godot_api]
impl Bar {
    #[func]
    fn increase_height(mut object: Gd<Self>, height: f32) {
        let scale = object.get_scale();
        object.set_scale(scale + Vector2::new(0.0, height));
        object.translate(Vector2::new(0.0, height / -0.08));
    }

    #[func]
    fn decrease_height(mut object: Gd<Self>, height: f32) {
        let scale = object.get_scale();
        object.set_scale(scale + Vector2::new(0.0, -height));
        object.translate(Vector2::new(0.0, -height / -0.08));
    }

    fn create_with_height_and_index(index: u32, height: f32) -> Gd<Self> {
        Gd::from_init_fn(|base| Self {
            base,
            height,
            index,
        })
    }
}
