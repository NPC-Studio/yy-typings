use serde_json::ser::CompactFormatter;

use crate::formatter::Formatter;
use std::io;

pub(crate) struct SpriteFormatter {
    pub formatter: Formatter,
    pub in_sequence: bool,
    pub sequence_indent: usize,
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
        if self.has_value {
            writer.write_all(b",")?;
        }

        let compacted = self.use_compact();

        if self.in_sequence {
            self.sequence_indent -= 1;
            if self.sequence_indent == 0 {
                self.in_sequence = false;
            }
        }

        if compacted {
            writer.write_all(b"}")?;

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
            "{} -- indent={}, in_sequence={}, sequence_indent={}",
            fragment, self.current_indent, self.in_sequence, self.sequence_indent
        );
        if fragment == "sequence" {
            self.in_sequence = true;
        }

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
    use pretty_assertions::assert_eq;

    #[test]
    fn sprite_serialization() {
        let x = include_str!("../../../../Gms2/SwordAndField/sprites/spr_monster_sapling_main_idle_north/spr_monster_sapling_main_idle_north.yy");
        let json: crate::Sprite =
            serde_json::from_str(&crate::TrailingCommaUtility::clear_trailing_comma_once(x))
                .unwrap();

        let o = crate::serialize_file(&json);

        println!("{}", o);

        assert_eq!(x, o);
    }
}
