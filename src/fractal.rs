/*
    This file is part of grunge, a coherent noise generation library.
*/

//! Types for generating noise related to fractal patterns.
//!
//! These are the main coherent noise implementations. PinkNoise and BillowNoise
//! in particular can be traced back to ideas used by Ken Perlin in
//! [Tron](http://en.wikipedia.org/wiki/Tron), and later demonstrated when he
//! introduced the first "noise" function to industry in 1985 (where they were
//! used to create cloud and fire textures, respectively).
//!
//! <div style="margin: 0 auto; display: block; text-align: center">
//!     <img src="../../static/pink.png" alt="PinkNoise">
//!     <img src="../../static/billow.png" alt="PinkNoise">
//! </div>
//!
//! From left to right: [PinkNoise](./struct.PinkNoise.html), [BillowNoise]
//! (./struct.PinkNoise.html).

use std::default::Default;
use cgmath::vector::Vector2;

use primitives::{snoise_2d, NoiseModule};
use modifiers::Modifiable;

static PINKNOISE_SCALE: f32 = 0.25;
static BILLOWNOISE_SCALE: f32 = 0.25;

/// PinkNoise is generated by calculating the contribution of a number of
/// individual `octaves` of noise samples, and then adding them together.
/// PinkNoise is a kind of fractal noise, because the contributions are self-
/// similar.
///
/// Its output takes the form
///
/// $$M(\mathbf{x}) = p N(f \mathbf{x}) + p\^2 N(fl \mathbf{x}) + \ldots +
///     p\^n N(fl\^{n - 1} \mathbf{x})$$
///
/// where $p, f, l$ are persistence, frequency, and lacunarity, respectively,
/// $N$ is the noise function, and $\mathbf{x}$ is the vector of input
/// coordinates.
#[deriving(Clone)]
pub struct PinkNoise {
    /// The "seed" used to ensure reproducibility and variation in the output of
    /// the module.
    pub seed: uint,

    /// The scale of the noise. Setting this value is equivalent to scaling all
    /// input coordinates by the same value.
    pub frequency: f32,

    /// The apparent "roughness" of the noise. This value controls the amplitude
    /// falloff of the successive octaves, so that `0.5` will scale the first
    /// octave by `1.0`, the second by `0.5`, the third by `0.25`, and so on.
    pub persistence: f32,

    /// The frequency multiplier between successive octaves.
    pub lacunarity: f32,

    /// The number of octaves is the number of successive additive samples of
    /// the noise function this module will use to generate output. It is
    /// essentially a measure of the level of "detail" in the output.
    pub octaves: uint
}

impl PinkNoise {
    /// Create a new object with the seed `seed` and all parameters set to their
    /// default values.
    pub fn new(seed: uint) -> PinkNoise {
        PinkNoise { seed: seed, .. Default::default() }
    }
}

impl Default for PinkNoise {
    fn default() -> PinkNoise {
        PinkNoise {
            seed: 0, frequency: 1.0, persistence: 0.5,
            lacunarity: 2.0, octaves: 6
        }
    }
}

impl NoiseModule for PinkNoise {
    fn generate_2d(&self, v: Vector2<f32>) -> Result<f32, &str> {
        if self.octaves <= 1 {
            return Err("The number of octaves must be two or greater.");
        } else if self.octaves > 30 {
            return Err("The number of octaves must be less than 30.");
        }

        let mut result: f32 = 0.0;
        let mut sample = Vector2 {
            x: v.x * self.frequency, y: v.y * self.frequency
        };
        let mut persistence = 1.0;

        for octave in range(0, self.octaves) {
            result += persistence * snoise_2d(sample, self.seed + octave);
            sample = Vector2 {
                x: sample.x * self.lacunarity, y: sample.y * self.lacunarity
            };
            persistence *= self.persistence;
        }

        Ok(result * PINKNOISE_SCALE)
    }
}

impl Modifiable for PinkNoise {}

/// BillowNoise is quite smilar to PinkNoise, but uses the absolute value of the
/// noise function to create a more puffy, cloud-like appearance.
#[deriving(Clone)]
pub struct BillowNoise {
    /// The "seed" used to ensure reproducibility and variation in the output of
    /// the module.
    pub seed: uint,

    /// The scale of the noise. Setting this value is equivalent to scaling all
    /// input coordinates by the same value.
    pub frequency: f32,

    /// The apparent "roughness" of the noise. This value controls the amplitude
    /// falloff of the successive octaves, so that `0.5` will scale the first
    /// octave by `1.0`, the second by `0.5`, the third by `0.25`, and so on.
    pub persistence: f32,

    /// The frequency multiplier between successive octaves.
    pub lacunarity: f32,

    /// The number of octaves is the number of successive additive samples of
    /// the noise function this module will use to generate output. It is
    ///  essentially a measure of the level of "detail" in the output.
    pub octaves: uint,

    /// The offset from zero, used to reduce visual artifacts when using the
    /// absolute value function.
    pub offset: f32
}

impl BillowNoise {
    /// Create a new object with the seed `seed` and all parameters set to their
    /// default values.
    pub fn new(seed: uint) -> BillowNoise {
        BillowNoise { seed: seed, .. Default::default() }
    }
}

impl Default for BillowNoise {
    fn default() -> BillowNoise {
        BillowNoise {
            seed: 0, frequency: 1.0, persistence: 0.5,
            lacunarity: 2.0, offset: 0.2, octaves: 6
        }
    }
}

impl NoiseModule for BillowNoise {
    fn generate_2d(&self, v: Vector2<f32>) -> Result<f32, &str> {
        if self.octaves <= 1 {
            return Err("The number of octaves must be two or greater.");
        } else if self.octaves > 30 {
            return Err("The number of octaves must be less than 30.");
        }

        let mut result: f32 = 0.0;
        let mut sample = Vector2 {
            x: v.x * self.frequency, y: v.y * self.frequency
        };
        let mut persistence = 1.0;

        for octave in range(0, self.octaves) {
            result += persistence *
                (snoise_2d(sample, self.seed + octave) + self.offset).abs();
            sample = Vector2 {
                x: sample.x * self.lacunarity, y: sample.y * self.lacunarity
            };
            persistence *= self.persistence;
        }

        Ok(result * BILLOWNOISE_SCALE * 2.0 - 1.0)
    }
}

impl Modifiable for BillowNoise {}
