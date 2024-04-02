use crate::font::FontWeight;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

/// A helper structure that stores bytes into files in a given directory.
/// These files will contain the rasterized characters. They can be included with `include!`
/// afterwards. It names all files in a reproducible way. Each filename is influenced by the
/// character, the font weight, and the raster height. This way, new unicode ranges can be included
/// without invaliding old files, which would happen in the case of incremental numbers as file
/// name.
#[derive(Debug)]
pub struct BytesToFileOutsourcer {
    out_dir: &'static str,
}

impl BytesToFileOutsourcer {
    pub const fn new(out_dir: &'static str) -> Self {
        Self { out_dir }
    }

    /// Creates a file in the given directory with `bytes` as content.
    /// The `c` is relevant for the naming of the file.
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
    pub c: char,
    pub weight: FontWeight,
    pub height: u32,
}

impl Context {
    fn generate_filename(&self) -> String {
        format!(
            "0x{:x}_h{}_w{:?}.txt",
            self.c as u32,
            self.height,
            self.weight.name()
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
                c: 'a',
                weight: FontWeight::new(FontWeightName::Regular, false),
                height: 30,
            },
        );

        // 0x61: ascii of 'a'
        assert_eq!(
            "target/0x61_h30_wRegular.txt",
            path.as_os_str().to_str().unwrap()
        );
    }
}
