use serde::Serialize;
use serde_json::ser::CompactFormatter;
use std::{any::TypeId, io};

/// Serializes a given Yy file.
#[cfg(target_os = "macos")]
pub fn serialize_file<T: Serialize + 'static>(value: &T) -> String {
    ser(value)
}

/// Serializes a given Yy file.
#[cfg(target_os = "windows")]
pub fn serialize_file(value: &Yyp) -> String {
    ser(value).replace('\n', "\r\n")
}

fn ser<T: Serialize + 'static>(value: &T) -> String {
    let mut writer = Vec::with_capacity(128);
    let formatter = Formatter {
        current_indent: 0,
        has_value: false,
        compact: 0,
    };
    if TypeId::of::<T>() == TypeId::of::<crate::Sprite>() {
        let mut ser = serde_json::ser::Serializer::with_formatter(
            &mut writer,
            SpriteFormatter {
                formatter,
                in_sequence: false,
                sequence_indent: 0,
            },
        );
        value.serialize(&mut ser).unwrap();
    } else {
        let mut ser = serde_json::ser::Serializer::with_formatter(&mut writer, formatter);
        value.serialize(&mut ser).unwrap();
    };
    unsafe {
        // We do not emit invalid UTF-8.
        String::from_utf8_unchecked(writer)
    }
}

#[derive(Debug)]
struct Formatter {
    current_indent: usize,
    has_value: bool,
    compact: usize,
}

impl Formatter {
    fn indent<W>(&self, wr: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        for _ in 0..self.current_indent {
            wr.write_all(b"  ")?;
        }

        Ok(())
    }

    fn use_compact(&self) -> bool {
        self.compact > 0
    }
}

impl serde_json::ser::Formatter for Formatter {
    #[inline]
    fn begin_array<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        self.current_indent += 1;
        self.has_value = false;
        self.compact += 1;
        writer.write_all(b"[")
    }

    #[inline]
    fn end_array<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        self.current_indent -= 1;
        self.compact -= 1;

        if self.has_value {
            writer.write_all(b",\n")?;
            self.indent(writer)?;
        }

        writer.write_all(b"]")
    }

    #[inline]
    fn begin_array_value<W>(&mut self, writer: &mut W, first: bool) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        if first {
            writer.write_all(b"\n")?;
        } else {
            writer.write_all(b",\n")?;
        }
        self.indent(writer)?;
        Ok(())
    }

    #[inline]
    fn end_array_value<W>(&mut self, _writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        self.has_value = true;

        Ok(())
    }

    #[inline]
    fn begin_object<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        self.has_value = false;
        if self.use_compact() {
            CompactFormatter.begin_object(writer)?;

            return Ok(());
        }

        self.current_indent += 1;
        writer.write_all(b"{")
    }

    #[inline]
    fn end_object<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        if self.use_compact() {
            writer.write_all(b",")?;
            CompactFormatter.end_object(writer)?;

            return Ok(());
        }

        self.current_indent -= 1;

        if self.has_value {
            writer.write_all(b",\n")?;
            self.indent(writer)?;
        }

        writer.write_all(b"}")?;
        Ok(())
    }

    #[inline]
    fn begin_object_key<W>(&mut self, writer: &mut W, first: bool) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        if self.use_compact() {
            CompactFormatter.begin_object_key(writer, first)?;

            return Ok(());
        }

        if first {
            writer.write_all(b"\n")?;
        } else {
            writer.write_all(b",\n")?;
        }
        self.indent(writer)?;

        Ok(())
    }

    fn end_object_key<W>(&mut self, _writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        Ok(())
    }

    #[inline]
    fn begin_object_value<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        if self.use_compact() {
            CompactFormatter.begin_object_value(writer)?;

            return Ok(());
        }
        writer.write_all(b": ")
    }

    #[inline]
    fn end_object_value<W>(&mut self, _writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        self.has_value = true;

        Ok(())
    }
}

struct SpriteFormatter {
    formatter: Formatter,
    in_sequence: bool,
    sequence_indent: usize,
}

impl SpriteFormatter {
    fn use_compact(&self) -> bool {
        self.formatter.use_compact() || (self.in_sequence && self.sequence_indent > 1)
    }
}

impl serde_json::ser::Formatter for SpriteFormatter {
    #[inline]
    fn begin_array<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        self.formatter.begin_array(writer)
    }

    #[inline]
    fn end_array<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        self.formatter.end_array(writer)
    }

    #[inline]
    fn begin_array_value<W>(&mut self, writer: &mut W, first: bool) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        self.formatter.begin_array_value(writer, first)
    }

    #[inline]
    fn end_array_value<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        self.formatter.end_array_value(writer)
    }

    #[inline]
    fn begin_object<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        self.has_value = false;

        if self.in_sequence {
            self.sequence_indent += 1;
        }

        if self.use_compact() {
            CompactFormatter.begin_object(writer)?;

            return Ok(());
        }

        self.current_indent += 1;
        writer.write_all(b"{")
    }

    #[inline]
    fn end_object<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        writer.write_all(b",")?;

        if self.use_compact() {
            writer.write_all(b"}")?;

            if self.in_sequence {
                self.sequence_indent -= 1;
                if self.sequence_indent <= 1 {
                    self.in_sequence = false;
                }
            }

            return Ok(());
        }

        self.current_indent -= 1;

        if self.has_value {
            writer.write_all(b"\n")?;
            self.indent(writer)?;
        }

        writer.write_all(b"}")?;
        Ok(())
    }

    #[inline]
    fn begin_object_key<W>(&mut self, writer: &mut W, first: bool) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        if self.use_compact() {
            CompactFormatter.begin_object_key(writer, first)?;

            return Ok(());
        }

        if first {
            writer.write_all(b"\n")?;
        } else {
            writer.write_all(b",\n")?;
        }
        self.indent(writer)?;

        Ok(())
    }

    #[inline]
    fn begin_object_value<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        if self.use_compact() {
            CompactFormatter.begin_object_value(writer)?;

            return Ok(());
        }
        writer.write_all(b": ")
    }

    #[inline]
    fn end_object_value<W>(&mut self, _writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        self.has_value = true;

        Ok(())
    }

    fn write_string_fragment<W>(&mut self, writer: &mut W, fragment: &str) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        println!(
            "{} -- {}/ {}",
            fragment, self.current_indent, self.in_sequence
        );
        if fragment == "sequence" {
            self.in_sequence = true;
        }

        todo!("let's VIBE");

        writer.write_all(fragment.as_bytes())
    }
}

impl std::ops::Deref for SpriteFormatter {
    type Target = Formatter;

    fn deref(&self) -> &Self::Target {
        &self.formatter
    }
}

impl std::ops::DerefMut for SpriteFormatter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.formatter
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn yyp_serialization() {
        let x = include_str!("../../../Gms2/SwordAndField/FieldsOfMistria.yyp");
        let json: crate::Yyp =
            serde_json::from_str(&crate::TrailingCommaUtility::clear_trailing_comma_once(x))
                .unwrap();

        let o = serialize_file(&json);

        assert_eq!(x, o);
    }

    #[test]
    fn object_serialization() {
        let x = include_str!("../../../Gms2/SwordAndField/objects/Game/game.yy");
        let json: crate::Object =
            serde_json::from_str(&crate::TrailingCommaUtility::clear_trailing_comma_once(x))
                .unwrap();

        let o = serialize_file(&json);

        assert_eq!(x, o);
    }

    #[test]
    fn sprite_serialization() {
        let x = include_str!("../../../Gms2/SwordAndField/sprites/spr_monster_sapling_main_idle_north/spr_monster_sapling_main_idle_north.yy");
        let json: crate::Sprite =
            serde_json::from_str(&crate::TrailingCommaUtility::clear_trailing_comma_once(x))
                .unwrap();

        let o = serialize_file(&json);
        assert_eq!(x, o);
    }
}
