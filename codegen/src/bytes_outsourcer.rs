use crate::font::FontWeight;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

/// A helper structure that stores bytes into files in a given directory.
///
/// These files contain the rasterized glyphs of one unicode range for a given
/// font weight and raster height as raw binary blob. They can be included with
/// `include_bytes!` afterwards. It names all files in a reproducible way. Each
/// filename is influenced by the font weight, the raster height, and the
/// unicode range. This way, new unicode ranges can be included without
/// invalidating old files, which would happen in the case of incremental
/// numbers as file name.
#[derive(Debug)]
pub struct BytesToFileOutsourcer {
    out_dir: &'static str,
}

impl BytesToFileOutsourcer {
    pub const fn new(out_dir: &'static str) -> Self {
        Self { out_dir }
    }

    /// Creates a file in the given directory with `bytes` as content.
    /// The `ctx` is relevant for the naming of the file.
    pub fn outsource_bytes(&mut self, bytes: &[u8], ctx: Context) -> PathBuf {
        let path = self.generate_path(ctx);

        let mut file = File::options()
            .create(true)
            .truncate(true)
            .write(true)
            .open(path.as_path())
            .unwrap();
        file.write_all(bytes).unwrap();
        path
    }

    fn generate_path(&self, ctx: Context) -> PathBuf {
        let mut buf = PathBuf::new();
        buf.push(self.out_dir);
        buf.push(ctx.generate_filename());
        buf
    }
}

/// Context object needed to uniquely name files in a reproducible way.
#[derive(Debug, Copy, Clone)]
pub struct Context {
    pub weight: FontWeight,
    pub height: u32,
    /// Feature name of the unicode range, such as `unicode-basic-latin`.
    pub unicode_range: &'static str,
}

impl Context {
    fn generate_filename(&self) -> String {
        format!(
            "{}_size_{}_{}.bin",
            self.weight.mod_name(),
            self.height,
            self.unicode_range
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::bytes_outsourcer::{BytesToFileOutsourcer, Context};
    use crate::font::{FontWeight, FontWeightName};

    #[test]
    fn test_bytes_outsourcer() {
        let mut outsourcer = BytesToFileOutsourcer::new("target");
        let path = outsourcer.outsource_bytes(
            b"hello world",
            Context {
                weight: FontWeight::new(FontWeightName::Regular, false),
                height: 30,
                unicode_range: "unicode-basic-latin",
            },
        );

        assert_eq!(
            "target/regular_size_30_unicode-basic-latin.bin",
            path.as_os_str().to_str().unwrap()
        );
        assert_eq!(std::fs::read(path).unwrap(), b"hello world");
    }
}
