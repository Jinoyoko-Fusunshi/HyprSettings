use std::fmt;
use std::fmt::Display;
use gtk::gdk::RGBA;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use serde::de::Visitor;

#[derive(Debug, Clone)]
pub struct RGBAColor {
    rgba: RGBA,
}

impl Default for RGBAColor {
    fn default() -> Self {
        Self {
            rgba: RGBA::new(0.0, 0.0, 0.0, 0.0)
        }
    }
}

impl Serialize for RGBAColor {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let rgba_string = self.rgba.to_str();
        serializer.serialize_str(&rgba_string)
    }
}

impl Display for RGBAColor {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.rgba.to_string())
    }
}

impl<'de> Deserialize<'de> for RGBAColor {
    fn deserialize<D>(deserializer: D) -> Result<RGBAColor, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RGBAVisitor;
        impl<'de> Visitor<'de> for RGBAVisitor {
            type Value = RGBAColor;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a CSS-style rgba string like 'rgba(255,0,0,0.5)'")
            }

            fn visit_str<E>(self, value: &str) -> Result<RGBAColor, E>
            where
                E: de::Error,
            {
                let rgba = RGBA::parse(value).expect("Cannot parse rgba string");
                Ok(RGBAColor::new(rgba))
            }
        }

        deserializer.deserialize_str(RGBAVisitor)
    }
}

impl RGBAColor {
    pub fn new(rgba: RGBA) -> RGBAColor {
        RGBAColor {
            rgba
        }
    }
    
    pub fn get_rgba(&self) -> &RGBA {
        &self.rgba
    }
}