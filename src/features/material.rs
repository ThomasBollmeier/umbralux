use crate::core::Color;

pub struct MaterialBuilder {
    color: Color,
    ambient: f64,
    diffuse: f64,
    specular: f64,
    shininess: f64,
}

impl MaterialBuilder {
    pub fn new() -> MaterialBuilder {
        MaterialBuilder {
            color: Color::new(1.0, 1.0, 1.0),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
        }
    }

    pub fn color(&mut self, color: Color) -> &mut Self {
        self.color = color;
        self
    }

    pub fn ambient(&mut self, value: f64) -> &mut Self {
        self.ambient = value;
        self
    }

    pub fn diffuse(&mut self, value: f64) -> &mut Self {
        self.diffuse = value;
        self
    }

    pub fn specular(&mut self, value: f64) -> &mut Self {
        self.specular = value;
        self
    }

    pub fn shininess(&mut self, value: f64) -> &mut Self {
        self.shininess = value;
        self
    }

    pub fn build(&self) -> Material {
        Material {
            color: self.color,
            ambient: self.ambient,
            diffuse: self.diffuse,
            specular: self.specular,
            shininess: self.shininess,
        }
    }

}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Material {
    pub color: Color,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
}

#[cfg(test)]
mod tests {
    use crate::core::Color;
    use crate::features::material::MaterialBuilder;
    use crate::testutil::assert_color_eq;

    #[test]
    fn default_material() {

        let builder = MaterialBuilder::new();
        let material = builder.build();

        assert_color_eq(material.color, Color::new(1.0, 1.0, 1.0));
        assert_float_absolute_eq!(material.ambient, 0.1);
        assert_float_absolute_eq!(material.diffuse, 0.9);
        assert_float_absolute_eq!(material.specular, 0.9);
        assert_float_absolute_eq!(material.shininess, 200.0);

    }

    #[test]
    fn modfied_material() {

        let material = MaterialBuilder::new()
            .color(Color::new(2.0, 2.0, 2.0))
            .ambient(0.2)
            .diffuse(1.0)
            .specular(1.2)
            .shininess(400.0)
            .build();

        assert_color_eq(material.color, Color::new(2.0, 2.0, 2.0));
        assert_float_absolute_eq!(material.ambient, 0.2);
        assert_float_absolute_eq!(material.diffuse, 1.0);
        assert_float_absolute_eq!(material.specular, 1.2);
        assert_float_absolute_eq!(material.shininess, 400.0);

    }



}