use hsv::Hsv;
use hsl::Hsl;
use rgb::Rgb;
use hsi::Hsi;
use angle::*;

pub struct TestColor {
    pub hsv: Hsv<f32>,
    pub chroma: f32,
    pub rgb: Rgb<f32>,
    pub hsl: Hsl<f32>,
    pub hsi: Hsi<f32>,
    pub circular_chroma: f32,
}

pub fn make_test_array() -> Vec<TestColor> {
    vec![TestColor {
             hsv: Hsv::from_channels(Deg(0.0), 0.000, 1.000),
             chroma: 0.000,
             rgb: Rgb::from_channels(1.000, 1.000, 1.000),
             hsl: Hsl::from_channels(Deg(0.0), 0.000, 1.000),
             hsi: Hsi::from_channels(Deg(0.0), 0.000, 1.000),
             circular_chroma: 0.000,
         },

         TestColor {
             hsv: Hsv::from_channels(Deg(0.0), 0.000, 0.500),
             chroma: 0.000,
             rgb: Rgb::from_channels(0.500, 0.500, 0.500),
             hsl: Hsl::from_channels(Deg(0.0), 0.000, 0.500),
             hsi: Hsi::from_channels(Deg(0.0), 0.000, 0.500),
             circular_chroma: 0.000,
         },

         TestColor {
             hsv: Hsv::from_channels(Deg(0.0), 0.000, 0.000),
             chroma: 0.000,
             rgb: Rgb::from_channels(0.000, 0.000, 0.000),
             hsl: Hsl::from_channels(Deg(0.0), 0.000, 0.000),
             hsi: Hsi::from_channels(Deg(0.0), 0.000, 0.000),
             circular_chroma: 0.000,
         },

         TestColor {
             hsv: Hsv::from_channels(Deg(0.0), 1.000, 1.000),
             chroma: 1.000,
             rgb: Rgb::from_channels(1.000, 0.000, 0.000),
             hsl: Hsl::from_channels(Deg(0.0), 1.000, 0.500),
             hsi: Hsi::from_channels(Deg(0.0), 1.000, 0.333),
             circular_chroma: 1.000,
         },

         TestColor {
             hsv: Hsv::from_channels(Deg(60.0), 1.000, 0.750),
             chroma: 0.750,
             rgb: Rgb::from_channels(0.750, 0.750, 0.000),
             hsl: Hsl::from_channels(Deg(60.0), 1.000, 0.375),
             hsi: Hsi::from_channels(Deg(60.0), 1.000, 0.500),
             circular_chroma: 0.750,
         },

         TestColor {
             hsv: Hsv::from_channels(Deg(120.0), 1.000, 0.500),
             chroma: 0.500,
             rgb: Rgb::from_channels(0.000, 0.500, 0.000),
             hsl: Hsl::from_channels(Deg(120.0), 1.000, 0.250),
             hsi: Hsi::from_channels(Deg(120.0), 1.000, 0.167),
             circular_chroma: 0.500,
         },

         TestColor {
             hsv: Hsv::from_channels(Deg(180.0), 0.500, 1.000),
             chroma: 0.500,
             rgb: Rgb::from_channels(0.500, 1.000, 1.000),
             hsl: Hsl::from_channels(Deg(180.0), 1.000, 0.750),
             hsi: Hsi::from_channels(Deg(180.0), 0.400, 0.833),
             circular_chroma: 0.500,
         },

         TestColor {
             hsv: Hsv::from_channels(Deg(240.0), 0.500, 1.000),
             chroma: 0.500,
             rgb: Rgb::from_channels(0.500, 0.500, 1.000),
             hsl: Hsl::from_channels(Deg(240.0), 1.000, 0.750),
             hsi: Hsi::from_channels(Deg(240.0), 0.250, 0.667),
             circular_chroma: 0.500,
         },

         TestColor {
             hsv: Hsv::from_channels(Deg(300.0), 0.667, 0.750),
             chroma: 0.500,
             rgb: Rgb::from_channels(0.750, 0.250, 0.750),
             hsl: Hsl::from_channels(Deg(300.0), 0.500, 0.500),
             hsi: Hsi::from_channels(Deg(300.0), 0.571, 0.583),
             circular_chroma: 0.500,
         },

         TestColor {
             hsv: Hsv::from_channels(Deg(61.8), 0.779, 0.643),
             chroma: 0.501,
             rgb: Rgb::from_channels(0.628, 0.643, 0.142),
             hsl: Hsl::from_channels(Deg(61.8), 0.638, 0.393),
             hsi: Hsi::from_channels(Deg(61.5), 0.699, 0.471),
             circular_chroma: 0.494,
         },

         TestColor {
             hsv: Hsv::from_channels(Deg(251.1), 0.887, 0.918),
             chroma: 0.814,
             rgb: Rgb::from_channels(0.255, 0.104, 0.918),
             hsl: Hsl::from_channels(Deg(251.1), 0.832, 0.511),
             hsi: Hsi::from_channels(Deg(250.0), 0.756, 0.426),
             circular_chroma: 0.750,
         },

         TestColor {
             hsv: Hsv::from_channels(Deg(134.9), 0.828, 0.675),
             chroma: 0.559,
             rgb: Rgb::from_channels(0.116, 0.675, 0.255),
             hsl: Hsl::from_channels(Deg(134.9), 0.707, 0.396),
             hsi: Hsi::from_channels(Deg(133.8), 0.667, 0.349),
             circular_chroma: 0.504,
         },

         TestColor {
             hsv: Hsv::from_channels(Deg(49.5), 0.944, 0.941),
             chroma: 0.888,
             rgb: Rgb::from_channels(0.941, 0.785, 0.053),
             hsl: Hsl::from_channels(Deg(49.5), 0.893, 0.497),
             hsi: Hsi::from_channels(Deg(50.5), 0.911, 0.593),
             circular_chroma: 0.821,
         },

         TestColor {
             hsv: Hsv::from_channels(Deg(283.7), 0.792, 0.897),
             chroma: 0.710,
             rgb: Rgb::from_channels(0.704, 0.187, 0.897),
             hsl: Hsl::from_channels(Deg(283.7), 0.775, 0.542),
             hsi: Hsi::from_channels(Deg(284.8), 0.686, 0.596),
             circular_chroma: 0.636,
         },

         TestColor {
             hsv: Hsv::from_channels(Deg(14.3), 0.661, 0.931),
             chroma: 0.615,
             rgb: Rgb::from_channels(0.931, 0.463, 0.316),
             hsl: Hsl::from_channels(Deg(14.3), 0.817, 0.624),
             hsi: Hsi::from_channels(Deg(13.2), 0.446, 0.570),
             circular_chroma: 0.556,
         },

         TestColor {
             hsv: Hsv::from_channels(Deg(56.9), 0.467, 0.998),
             chroma: 0.466,
             rgb: Rgb::from_channels(0.998, 0.974, 0.532),
             hsl: Hsl::from_channels(Deg(56.9), 0.991, 0.765),
             hsi: Hsi::from_channels(Deg(57.4), 0.363, 0.835),
             circular_chroma: 0.454,
         },

         TestColor {
             hsv: Hsv::from_channels(Deg(162.4), 0.875, 0.795),
             chroma: 0.696,
             rgb: Rgb::from_channels(0.099, 0.795, 0.591),
             hsl: Hsl::from_channels(Deg(162.4), 0.779, 0.447),
             hsi: Hsi::from_channels(Deg(163.4), 0.800, 0.495),
             circular_chroma: 0.620,
         },

         TestColor {
             hsv: Hsv::from_channels(Deg(248.3), 0.750, 0.597),
             chroma: 0.448,
             rgb: Rgb::from_channels(0.211, 0.149, 0.597),
             hsl: Hsl::from_channels(Deg(248.3), 0.601, 0.373),
             hsi: Hsi::from_channels(Deg(247.3), 0.533, 0.319),
             circular_chroma: 0.420,
         },

         TestColor {
             hsv: Hsv::from_channels(Deg(240.5), 0.316, 0.721),
             chroma: 0.228,
             rgb: Rgb::from_channels(0.495, 0.493, 0.721),
             hsl: Hsl::from_channels(Deg(240.5), 0.290, 0.607),
             hsi: Hsi::from_channels(Deg(240.4), 0.135, 0.570),
             circular_chroma: 0.227,
         }]
}
