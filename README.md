# lv_cxx_bindgen
A C++ binding generator for LVGL. Written in Rust (oh the irony)

> This is still work-in-progress, it doesn't even generate any sources for
> now, but it's in the works.

This generator uses `tree-sitter` to efficiently construct an AST of all the headers
in LVGL source code, that will later be analyzed, and used as a basis for all
autogenerated C++ code. Also, it supports different C++ target versions, like
C++11, C++20 etc.

## Usage

`lv_cxx_bindgen` requires a config file, by default named `lv_cxx_bindgen.toml` and
looked for in CWD, but can be specified in the `-c` argument.

> TODO: Finish writing this section after CLI API is finished

## Configuration

All configuration is done in the `lv_cxx_bindgen.toml` file. An example file
looks like this:

```toml
[input]
cwd = "res/src/lvgl"
files = [ "lvgl.h" ]
auto_scan = true

[generation]
target = "c++20"
root_namespace = "lv"
classes = [
    "obj",
    { ident = "img", inherits = ["obj"] },
    { ident = "anim", inherits = ["obj"] }
]
namespaces = [
    "tick",
    "mem"
]
```

### Input

**`cwd`** - current working directory. Used when resolving input file paths.

**`files`** - input files, which are then parsed for functions. If `auto_scan` is enabled,
it will also scan the input files for #include directives, and adds those files to the list
of input files.

### Generation

**`target`** - target C++ version, by default C++20. All the differences between different
targets and generated output:

- C++11
    - Functions that accept arrays in arguments have those arguments converted from pointers
    to `std::vector` or `std::array`, depending on configuration
    - Functions that accept function pointers in arguments have those arguments converted
    to `std::function`, but as an overload, so there are options for `std::function`, and
    normal function pointers
- C++14 & C++17 don't have any changes, but it's still a good idea to set the target to your
C++ version, so that in future updates it will not break
- C++20
    - Now there is no header file in the output, only a `.cppm` file, which is a C++ module,
    that can be imported with `import lvgl;`

**`root_namespace`** - namespace that houses all generated functions, classes etc.

> TODO: document `classes` and `namespaces` when those will be implemented

## Process

In short, the whole process can be simplified to 3 main steps:

- Parsing
- Grouping
- Generation

The first step is responsible for getting the raw source code, parsing it into
an AST, and extracting all functions/constants/typedefs/etc into a list.

The second step then groups all items inside that list in, well, groups, for
example "this group of functions should result in a class", or "this group
of functions should result in a namespace" etc.

And the best for last, the third step consists of actually converting allat into
actual C++ code. That shit is done manually, for plain efficiency and also ease
of understanding what is actually going on. Also, an additional `clang-format`
run can be named a "three and a half" step, because it's part of codegen, but not
part of the actual generator.

So, a full process can be described like this:

- Extraction
    - `pcpp` run over headers for `#ifdef` macro expansion
    - Source AST parsing w/ `tree-sitter`
    - Function list extraction
    - Function list simplification (removal of singular void args, etc)
- Grouping
    - In namespaces
    - In classes
    - Function arguments transformation for more idiomatic C++
- Generation
    - Conversion of groups into output AST
    - AST to source code generation
    - clang-format run over generated code
