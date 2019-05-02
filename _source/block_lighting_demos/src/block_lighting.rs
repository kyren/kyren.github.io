pub trait BlockLightValue: Clone {
    fn dark() -> Self;

    fn spread(self, dest: Self, drop: f32) -> Self;
    fn subtract(self, drop: f32) -> Self;

    fn max_intensity(self) -> f32;
    fn min_intensity(self) -> f32;

    fn max(self, other: Self) -> Self;
}

#[derive(Clone, Copy)]
pub struct ScalarLightValue(pub f32);

impl BlockLightValue for ScalarLightValue {
    fn dark() -> Self {
        ScalarLightValue(0.0)
    }

    fn spread(self, dest: Self, drop: f32) -> Self {
        ScalarLightValue((self.0 - drop).max(dest.0))
    }

    fn subtract(self, drop: f32) -> Self {
        ScalarLightValue((self.0 - drop).max(0.0))
    }

    fn max_intensity(self) -> f32 {
        self.0
    }

    fn min_intensity(self) -> f32 {
        self.0
    }

    fn max(self, other: Self) -> Self {
        ScalarLightValue(self.0.max(other.0))
    }
}

#[derive(Clone, Copy)]
pub struct ColoredLightValue(pub f32, pub f32, pub f32);

impl BlockLightValue for ColoredLightValue {
    fn dark() -> Self {
        ColoredLightValue(0.0, 0.0, 0.0)
    }

    fn spread(self, dest: Self, drop: f32) -> Self {
        let max_channel = self.0.max(self.1.max(self.2));
        if max_channel <= 0.0 {
            dest
        } else {
            ColoredLightValue(
                dest.0.max(self.0 - self.0 * drop / max_channel),
                dest.1.max(self.1 - self.1 * drop / max_channel),
                dest.2.max(self.2 - self.2 * drop / max_channel),
            )
        }
    }

    fn subtract(self, drop: f32) -> Self {
        let max = self.0.max(self.1.max(self.2));
        if max <= 0.0 {
            self
        } else {
            ColoredLightValue(
                (self.0 - self.0 * drop / max).max(0.0),
                (self.1 - self.1 * drop / max).max(0.0),
                (self.2 - self.2 * drop / max).max(0.0),
            )
        }
    }

    fn max_intensity(self) -> f32 {
        self.0.max(self.1.max(self.2))
    }

    fn min_intensity(self) -> f32 {
        self.0.min(self.1.min(self.2))
    }

    fn max(self, other: Self) -> Self {
        ColoredLightValue(
            self.0.max(other.0),
            self.1.max(other.1),
            self.2.max(other.2),
        )
    }
}

#[derive(Clone)]
pub struct BlockLightCell<LV: BlockLightValue> {
    light: LV,
    is_obstacle: bool,
}

pub struct SpreadLight<LV: BlockLightValue> {
    position: (f32, f32),
    light: LV,
}

pub struct PointLight<LV: BlockLightValue> {
    position: (f32, f32),
    light: LV,
    beam: f32,
    beam_angle: f32,
    beam_ambience: f32,
}

pub struct BlockLightSettings {
    pub spread_passes: u8,
    pub spread_max_air: f32,
    pub spread_max_obstacle: f32,
    pub point_max_air: f32,
    pub point_max_obstacle: f32,
    pub point_obstacle_boost: f32,
}

pub struct BlockLightArray<LV: BlockLightValue> {
    settings: BlockLightSettings,
    spread_lights: Vec<SpreadLight<LV>>,
    point_lights: Vec<PointLight<LV>>,
    width: u32,
    height: u32,
    cells: Vec<BlockLightCell<LV>>,
}

type ColoredBlockLightArray = BlockLightArray<ColoredLightValue>;
type ScalarBlockLightArray = BlockLightArray<ScalarLightValue>;

impl<LV: BlockLightValue> BlockLightArray<LV> {
    pub fn new(settings: BlockLightSettings) -> BlockLightArray<LV> {
        assert!(settings.spread_passes > 0);
        assert!(settings.point_max_air >= settings.point_max_obstacle);
        assert!(settings.spread_max_air >= settings.spread_max_obstacle);

        BlockLightArray {
            settings,
            spread_lights: Vec::new(),
            point_lights: Vec::new(),
            width: 0,
            height: 0,
            cells: Vec::new(),
        }
    }

    /// The number of cells around an area to be lit where initial lighting and other light source
    /// data is required, based on BlockLightSettings (this is the maximum distance that light can
    /// travel, in cells, so anything more than this far beyond the lit area border cannot have an
    /// effect).
    pub fn border_cells(&self) -> u32 {
        self.settings
            .spread_max_air
            .max(self.settings.point_max_air)
            .ceil() as u32
    }

    /// Begin a new lighting calculation with the given array width and height.  Always clears all
    /// existing light and collision data.
    pub fn begin(&mut self, width: u32, height: u32) {
        assert!(width > 0 && height > 0);
        self.spread_lights.clear();
        self.point_lights.clear();
        self.cells.clear();
        self.cells.resize(
            width as usize * height as usize,
            BlockLightCell {
                light: LV::dark(),
                is_obstacle: false,
            },
        )
    }

    /// Position for lights is in index space, integer values lie on the corners of the grid, not
    /// the center.  Spread lights will have no effect if their position is outside of the array.
    pub fn add_spread_light(&mut self, spread_light: SpreadLight<LV>) {
        self.spread_lights.push(spread_light);
    }

    /// Position for lights is in index space, integer values lie on the corners of the grid, not
    /// the center.
    pub fn add_point_light(&mut self, point_light: PointLight<LV>) {
        self.point_lights.push(point_light);
    }

    /// Set whether a given cell obstructs light
    pub fn set_obstacle(&mut self, x: u32, y: u32, is_obstacle: bool) {
        self.cell_mut(x, y).is_obstacle = is_obstacle;
    }

    /// Return whether this cell is marked as obstructing light
    pub fn get_obstacle(&mut self, x: u32, y: u32) -> bool {
        self.cell(x, y).is_obstacle
    }

    /// Directly set the initial light value at this cell
    pub fn set_light(&mut self, x: u32, y: u32, light: LV) {
        self.cell_mut(x, y).light = light;
    }

    /// Get the light value for this cell, call after `calculate` to pull the final light data out.
    pub fn get_light(&self, x: u32, y: u32) -> &LV {
        &self.cell(x, y).light
    }

    /// Calculate lighting in the given sub-rect.  In order to properly perform lighting spread,
    /// there must be a border around the calculated region that is at least as large as the value
    /// returned by `border_cells`, and correct initial cell light values and light sources must be
    /// provided for the area within that border.
    pub fn calculate(xmin: u32, ymin: u32, width: u32, height: u32) {
        unimplemented!()
    }

    fn cell(&self, x: u32, y: u32) -> &BlockLightCell<LV> {
        &self.cells[y as usize * self.width as usize + x as usize]
    }

    fn cell_mut(&mut self, x: u32, y: u32) -> &mut BlockLightCell<LV> {
        &mut self.cells[y as usize * self.width as usize + x as usize]
    }
}
