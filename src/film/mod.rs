use crate::{geometry::{point::{Point2, Point2f}, bounds::{Bounds2f, Bounds2i}}, config::RaytracerFloat};


#[derive(Copy, Clone)]
struct Pixel {
    pub xyz: [RaytracerFloat; 3],
}

impl Default for Pixel {
    fn default() -> Self {
        Self {
            xyz: [0.0; 3]
        }
    }
}

#[allow(unused)]
pub struct Film {
    pub resolution: Point2<usize>,
    pub diagonal: RaytracerFloat,
    pixels: Vec<Pixel>
}

impl Film {
    #[allow(unused)]
    pub fn new(width: usize, height: usize, diagonal: RaytracerFloat) -> Self {
        Self {
            resolution: Point2::<usize>::new(width, height),
            diagonal,
            pixels: vec![Pixel::default(); width * height],
        }
    }

    #[allow(unused)]
    pub fn get_physical_extent(&self) -> Bounds2f {
        let aspect = (self.resolution.y as RaytracerFloat) / (self.resolution.x as RaytracerFloat);
        let x= ((self.diagonal*self.diagonal)/(1.0 + aspect*aspect)).sqrt();
        let y = aspect * x;
        Bounds2f::new(Point2f::new(-x/2.0, -y/2.0), Point2f::new(x/2.0, y/2.0))
    }

    pub fn get_image_tile(&self) {
        todo!("get_image_tile not yet implemented ")
    }
}



// class Film {
//     public:
//       // Film Public Methods
//       Film(const Point2i &resolution, const Bounds2f &cropWindow,
//            std::unique_ptr<Filter> filter, Float diagonal,
//            const std::string &filename, Float scale,
//            Float maxSampleLuminance = Infinity);
//       Bounds2i GetSampleBounds() const;
//       Bounds2f GetPhysicalExtent() const;
//       std::unique_ptr<FilmTile> GetFilmTile(const Bounds2i &sampleBounds);
//       void MergeFilmTile(std::unique_ptr<FilmTile> tile);
//       void SetImage(const Spectrum *img) const;
//       void AddSplat(const Point2f &p, Spectrum v);
//       void WriteImage(Float splatScale = 1);
//       void Clear();
  
//       // Film Public Data
//       const Point2i fullResolution;
//       const Float diagonal;
//       std::unique_ptr<Filter> filter;
//       const std::string filename;
//       Bounds2i croppedPixelBounds;
  
//     private:
//       // Film Private Data
//       struct Pixel {
//           Pixel() { xyz[0] = xyz[1] = xyz[2] = filterWeightSum = 0; }
//           Float xyz[3];
//           Float filterWeightSum;
//           AtomicFloat splatXYZ[3];
//           Float pad;
//       };
//       std::unique_ptr<Pixel[]> pixels;
//       static PBRT_CONSTEXPR int filterTableWidth = 16;
//       Float filterTable[filterTableWidth * filterTableWidth];
//       std::mutex mutex;
//       const Float scale;
//       const Float maxSampleLuminance;
  
//       // Film Private Methods
//       Pixel &GetPixel(const Point2i &p) {
//           CHECK(InsideExclusive(p, croppedPixelBounds));
//           int width = croppedPixelBounds.pMax.x - croppedPixelBounds.pMin.x;
//           int offset = (p.x - croppedPixelBounds.pMin.x) +
//                        (p.y - croppedPixelBounds.pMin.y) * width;
//           return pixels[offset];
//       }
//   };


#[derive(Copy, Clone)]
struct FilmTilePixel<Spectrum>
where Spectrum: Copy + Clone {
    contrib_sum: Spectrum,
}

impl<Spectrum> Default for FilmTilePixel<Spectrum>
where Spectrum: Default + Copy + Clone {
    fn default() -> Self {
        Self {
            contrib_sum: Spectrum::default()
        }
    }
}

#[allow(unused)]
pub struct FilmTile<const WIDTH: usize, const HEIGHT: usize, Spectrum>
where Spectrum: Default + Copy + Clone {
    pixel_bounds: Bounds2i,
    max_sample_luminance: RaytracerFloat,
    pixels: [[FilmTilePixel<Spectrum>; HEIGHT]; WIDTH]
}

impl<const WIDTH: usize, const HEIGHT: usize, Spectrum> FilmTile<WIDTH, HEIGHT, Spectrum>
where Spectrum: Default + Copy + Clone {
    #[allow(unused)]
    pub fn new(pixel_bounds: Bounds2i, max_sample_luminance: RaytracerFloat) -> Self {
        Self {
            max_sample_luminance,
            pixel_bounds,
            pixels: [[FilmTilePixel::<Spectrum>::default(); HEIGHT]; WIDTH]
        }
    }

    pub fn add_sample(&mut self, point: &Point2f, l: &Spectrum) {

    }

    // void AddSample(const Point2f &pFilm, Spectrum L,
    //     Float sampleWeight = 1.) {
    //     ProfilePhase _(Prof::AddFilmSample);
    //     if (L.y() > maxSampleLuminance)
    //     L *= maxSampleLuminance / L.y();
    //     // Compute sample's raster bounds
    //     Point2f pFilmDiscrete = pFilm - Vector2f(0.5f, 0.5f);
    //     Point2i p0 = (Point2i)Ceil(pFilmDiscrete - filterRadius);
    //     Point2i p1 =
    //     (Point2i)Floor(pFilmDiscrete + filterRadius) + Point2i(1, 1);
    //     p0 = Max(p0, pixelBounds.pMin);
    //     p1 = Min(p1, pixelBounds.pMax);

    //     // Loop over filter support and add sample to pixel arrays

    //     // Precompute $x$ and $y$ filter table offsets
    //     int *ifx = ALLOCA(int, p1.x - p0.x);
    //     for (int x = p0.x; x < p1.x; ++x) {
    //     Float fx = std::abs((x - pFilmDiscrete.x) * invFilterRadius.x *
    //                         filterTableSize);
    //     ifx[x - p0.x] = std::min((int)std::floor(fx), filterTableSize - 1);
    //     }
    //     int *ify = ALLOCA(int, p1.y - p0.y);
    //     for (int y = p0.y; y < p1.y; ++y) {
    //     Float fy = std::abs((y - pFilmDiscrete.y) * invFilterRadius.y *
    //                         filterTableSize);
    //     ify[y - p0.y] = std::min((int)std::floor(fy), filterTableSize - 1);
    //     }
    //     for (int y = p0.y; y < p1.y; ++y) {
    //     for (int x = p0.x; x < p1.x; ++x) {
    //         // Evaluate filter value at $(x,y)$ pixel
    //         int offset = ify[y - p0.y] * filterTableSize + ifx[x - p0.x];
    //         Float filterWeight = filterTable[offset];

    //         // Update pixel values with filtered sample contribution
    //         FilmTilePixel &pixel = GetPixel(Point2i(x, y));
    //         pixel.contribSum += L * sampleWeight * filterWeight;
    //         pixel.filterWeightSum += filterWeight;
    //     }
    //     }
    //     }
}

// class FilmTile {
//     public:
//       // FilmTile Public Methods
//       FilmTile(const Bounds2i &pixelBounds, const Vector2f &filterRadius,
//                const Float *filterTable, int filterTableSize,
//                Float maxSampleLuminance)
//           : pixelBounds(pixelBounds),
//             filterRadius(filterRadius),
//             invFilterRadius(1 / filterRadius.x, 1 / filterRadius.y),
//             filterTable(filterTable),
//             filterTableSize(filterTableSize),
//             maxSampleLuminance(maxSampleLuminance) {
//           pixels = std::vector<FilmTilePixel>(std::max(0, pixelBounds.Area()));
//       }
//       void AddSample(const Point2f &pFilm, Spectrum L,
//                      Float sampleWeight = 1.) {
//           ProfilePhase _(Prof::AddFilmSample);
//           if (L.y() > maxSampleLuminance)
//               L *= maxSampleLuminance / L.y();
//           // Compute sample's raster bounds
//           Point2f pFilmDiscrete = pFilm - Vector2f(0.5f, 0.5f);
//           Point2i p0 = (Point2i)Ceil(pFilmDiscrete - filterRadius);
//           Point2i p1 =
//               (Point2i)Floor(pFilmDiscrete + filterRadius) + Point2i(1, 1);
//           p0 = Max(p0, pixelBounds.pMin);
//           p1 = Min(p1, pixelBounds.pMax);
  
//           // Loop over filter support and add sample to pixel arrays
  
//           // Precompute $x$ and $y$ filter table offsets
//           int *ifx = ALLOCA(int, p1.x - p0.x);
//           for (int x = p0.x; x < p1.x; ++x) {
//               Float fx = std::abs((x - pFilmDiscrete.x) * invFilterRadius.x *
//                                   filterTableSize);
//               ifx[x - p0.x] = std::min((int)std::floor(fx), filterTableSize - 1);
//           }
//           int *ify = ALLOCA(int, p1.y - p0.y);
//           for (int y = p0.y; y < p1.y; ++y) {
//               Float fy = std::abs((y - pFilmDiscrete.y) * invFilterRadius.y *
//                                   filterTableSize);
//               ify[y - p0.y] = std::min((int)std::floor(fy), filterTableSize - 1);
//           }
//           for (int y = p0.y; y < p1.y; ++y) {
//               for (int x = p0.x; x < p1.x; ++x) {
//                   // Evaluate filter value at $(x,y)$ pixel
//                   int offset = ify[y - p0.y] * filterTableSize + ifx[x - p0.x];
//                   Float filterWeight = filterTable[offset];
  
//                   // Update pixel values with filtered sample contribution
//                   FilmTilePixel &pixel = GetPixel(Point2i(x, y));
//                   pixel.contribSum += L * sampleWeight * filterWeight;
//                   pixel.filterWeightSum += filterWeight;
//               }
//           }
//       }
//       FilmTilePixel &GetPixel(const Point2i &p) {
//           CHECK(InsideExclusive(p, pixelBounds));
//           int width = pixelBounds.pMax.x - pixelBounds.pMin.x;
//           int offset =
//               (p.x - pixelBounds.pMin.x) + (p.y - pixelBounds.pMin.y) * width;
//           return pixels[offset];
//       }
//       const FilmTilePixel &GetPixel(const Point2i &p) const {
//           CHECK(InsideExclusive(p, pixelBounds));
//           int width = pixelBounds.pMax.x - pixelBounds.pMin.x;
//           int offset =
//               (p.x - pixelBounds.pMin.x) + (p.y - pixelBounds.pMin.y) * width;
//           return pixels[offset];
//       }
//       Bounds2i GetPixelBounds() const { return pixelBounds; }
  
//     private:
//       // FilmTile Private Data
//       const Bounds2i pixelBounds;
//       const Vector2f filterRadius, invFilterRadius;
//       const Float *filterTable;
//       const int filterTableSize;
//       std::vector<FilmTilePixel> pixels;
//       const Float maxSampleLuminance;
//       friend class Film;
//   };