use std::rc::Rc;
use crate::core::Color;
use crate::features::pattern::Pattern;

pub struct MaterialBuilder {
    color: Color,
    pattern: Option<Rc<dyn Pattern>>,
    ambient: f64,
    diffuse: f64,
    specular: f64,
    shininess: f64,
}

impl MaterialBuilder {
    pub fn new() -> MaterialBuilder {
        MaterialBuilder {
            color: Color::new(1.0, 1.0, 1.0),
            pattern: None,
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

    pub fn pattern(&mut self, pattern: &Rc<dyn Pattern>) -> &mut Self {
        self.pattern = Some(pattern.clone());
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
            pattern: match &self.pattern {
                Some(pattern) => Some(pattern.clone()),
                None => None
            },
            ambient: self.ambient,
            diffuse: self.diffuse,
            specular: self.specular,
            shininess: self.shininess,
        }
    }

}

#[derive(Clone, Debug)]
pub struct Material {
    pub color: Color,
    pub pattern: Option<Rc<dyn Pattern>>,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
}

impl PartialEq for Material {

    fn eq(&self, other: &Self) -> bool {
        if self.color != other.color {
            return false;
        }

        match &self.pattern {
            Some(pattern) => match &other.pattern {
                Some(other_pattern) => {
                    if Rc::ptr_eq(pattern, other_pattern) {
                        return false;
                    }
                }
                None => {
                    return false;
                }
            }
            None => if let Some(_) = &other.pattern {
                return false;
            }
        }

        if self.ambient != other.ambient {
            return false;
        }

        if self.diffuse != other.diffuse {
            return false;
        }

        if self.specular != other.specular {
            return false;
        }

        if self.shininess != other.shininess {
            return false;
        }

        true
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
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