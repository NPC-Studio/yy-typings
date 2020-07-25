# Yy-Typings

This is a library was created for the development of [Fields of Mistria](https://twitter.com/FieldsofMistria), a farming RPG with *tons* of Sprites, by NPC Studio. [Jack Spira](https://twitter.com/sanbox_irl) wrote the first version of this tool and maintains it. This tool was created to support an Aseprite -> GMS2 pipeline tool. That tool is not public. Using this tool, one should be able to generate their own pipeline without difficulty.

***This crate only supports Gms2, and only supports Gms2 2.3 and above***.

This repository has a pair: [the Yy-Boss](https://github.com/NPC-Studio/yy-boss), which provides active Yyp handling as a library and as a binary over stdin/stdout, abstracting over Gms2's native types to allow users to dynamically create resources (and analyze existing resources) without handling the Gms2 Yy files directly.

This crate is composed of typings for the `.yyp` and various `.yy` files. The following typings have been created (and are actively supported):

- [x] YYP
- [x] Sprites
- [ ] Tilesets
- [ ] Sounds
- [ ] Paths
- [ ] Scripts
- [ ] Shaders
- [ ] Fonts
- [ ] Timelines
- [x] Objects
- [ ] Rooms
- [ ] Notes
- [ ] Included Files
- [ ] Extensions
- [x] Options*
- [x] Configurations

***Options are not directly user accessible, but are provided through other structures like TextureGroups. The Options file itself is very non-spec, and it difficult to get a cohesive picture of.**

## The Future Development of this Crate

This course of this crate will be developed as it is needed for further tools for *Fields of Mistria*. Eventually, all the Gms2 Yy files will be typed, though no guarentee is made of the maintainer's time or any particular deadline. If users would like to help, PRs are always welcome!
