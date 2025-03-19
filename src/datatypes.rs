pub mod vertex;
pub mod model_vertex;
pub mod texture;
pub mod model;
pub mod mesh;
pub mod material;
pub mod draw_model;

pub mod prelude {
    pub use crate::datatypes::vertex::Vertex;
    pub use crate::datatypes::model_vertex::ModelVertex;
    pub use crate::datatypes::model::Model;
    pub use crate::datatypes::mesh::Mesh;
    pub use crate::datatypes::material::Material;
    pub use crate::datatypes::draw_model::DrawModel;


    pub use crate::datatypes::texture::prelude::*;
}
