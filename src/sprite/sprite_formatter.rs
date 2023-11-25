pub struct SpriteFormatter {
    current_indent: usize,
    has_value: bool,
    indent: &'static [u8],
    mode: SpritePrinterMode,
    check_key: bool,
    object_stack: u32,
}

enum SpritePrinterMode {
    Normal,
    Frames(u32),
    Compact(u32),
}

impl SpriteFormatter {
    pub fn new() -> Self {
        SpriteFormatter {
            current_indent: 0,
            has_value: false,
            indent: b"  ",
            mode: SpritePrinterMode::Normal,
            check_key: false,
            object_stack: 0,
        }
    }
}

use std::io;
impl serde_json::ser::Formatter for SpriteFormatter {
    #[inline]
    fn begin_array<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        self.current_indent += 1;
        self.has_value = false;

        if let SpritePrinterMode::Frames(v) = &mut self.mode {
            *v += 1;
        }

        writer.write_all(b"[")
    }

    #[inline]
    fn end_array<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        self.current_indent -= 1;

        if self.has_value {
            writer.write_all(b"\n").unwrap();
            indent(writer, self.current_indent, self.indent).unwrap();
        }

        if let SpritePrinterMode::Frames(v) = &mut self.mode {
            *v -= 1;
            if *v == 0 {
                self.mode = SpritePrinterMode::Normal;
            }
        }

        writer.write_all(b"]")
    }

    #[inline]
    fn begin_array_value<W>(&mut self, writer: &mut W, _first: bool) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        writer.write_all(b"\n").unwrap();
        indent(writer, self.current_indent, self.indent).unwrap();
        Ok(())
    }

    #[inline]
    fn end_array_value<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        writer.write_all(b",").unwrap();
        self.has_value = true;
        Ok(())
    }

    #[inline]
    fn begin_object<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        self.current_indent += 1;
        self.object_stack += 1;
        self.has_value = false;

        // increment our object count WITHIN the frames...
        if let SpritePrinterMode::Compact(v) = &mut self.mode {
            *v += 1;
        }
        writer.write_all(b"{")
    }

    #[inline]
    fn end_object<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        self.current_indent -= 1;
        self.object_stack -= 1;

        // increment our object count WITHIN the frames...
        match &mut self.mode {
            SpritePrinterMode::Normal => {
                if self.has_value {
                    writer.write_all(b"\n").unwrap();
                    indent(writer, self.current_indent, self.indent).unwrap();
                }
            }
            SpritePrinterMode::Frames(_v) => {}
            SpritePrinterMode::Compact(_v) => {
                self.mode = SpritePrinterMode::Normal;
            }
        }

        writer.write_all(b"}")
    }

    #[inline]
    fn begin_object_key<W>(&mut self, writer: &mut W, _first: bool) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        match self.mode {
            SpritePrinterMode::Normal => {
                writer.write_all(b"\n").unwrap();
                self.check_key = true;
                indent(writer, self.current_indent, self.indent)
            }
            SpritePrinterMode::Frames(_) => Ok(()),
            SpritePrinterMode::Compact(_) => Ok(()),
        }
    }

    fn write_string_fragment<W>(&mut self, writer: &mut W, fragment: &str) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        if self.check_key {
            match fragment {
                "frames" | "tracks" => {
                    self.mode = SpritePrinterMode::Frames(0);
                }
                "spriteId" | "events" | "moments" | "layers" => {
                    self.mode = SpritePrinterMode::Compact(0);
                }
                "parent" => {
                    if self.object_stack > 1 {
                        self.mode = SpritePrinterMode::Compact(0);
                    }
                }
                _ => {}
            }
        }

        writer.write_all(fragment.as_bytes())
    }

    fn end_object_key<W>(&mut self, _writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        self.check_key = false;

        Ok(())
    }

    #[inline]
    fn begin_object_value<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        writer.write_all(b":").unwrap();

        match self.mode {
            SpritePrinterMode::Normal => writer.write_all(b" "),
            SpritePrinterMode::Frames(v) | SpritePrinterMode::Compact(v) => {
                if v == 0 {
                    writer.write_all(b" ")
                } else {
                    Ok(())
                }
            }
        }
    }

    #[inline]
    fn end_object_value<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        self.has_value = true;
        writer.write_all(b",").unwrap();
        Ok(())
    }
}

fn indent<W>(wr: &mut W, n: usize, s: &[u8]) -> io::Result<()>
where
    W: ?Sized + io::Write,
{
    for _ in 0..n {
        wr.write_all(s).unwrap();
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    #[test]
    fn sprite_serialization() {
        let x = include_str!("../../../../Gms2/SwordAndField/sprites/spr_monster_sapling_blue_main_attack_jump_south/spr_monster_sapling_blue_main_attack_jump_south.yy");
        let json: crate::Sprite =
            serde_json::from_str(&crate::TrailingCommaUtility::clear_trailing_comma_once(x))
                .unwrap();

        let o = crate::serialize_file(&json);
        assert_eq!(x, o);
    }

    #[test]
    fn sprite_serialization2() {
        let x = include_str!("../../../../Gms2/SwordAndField/sprites/spr_manor_doorway2_spring/spr_manor_doorway2_spring.yy");
        let json: crate::Sprite =
            serde_json::from_str(&crate::TrailingCommaUtility::clear_trailing_comma_once(x))
                .unwrap();

        let o = crate::serialize_file(&json);
        assert_eq!(x, o);
    }
}
