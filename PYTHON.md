# Python Bindings for uroman-rs

This document describes how to use the Python bindings for the uroman-rs universal romanization library.

## Installation

### Install via Github with `pip`

```bash
pip install git+https://https://github.com/LennartKeller/uroman-rs.git
```

## Quick Start

### Simple usage

```python
import uroman_rs

# Convenience function
result = uroman_rs.romanize("こんにちは")
print(result)  # "konnichiha"

# With language code
result = uroman_rs.romanize("مرحبا", lcode="ara")
print(result)  # "mrhba"
```

### Using the Uroman class

```python
from uroman_rs import Uroman

# Create a Uroman instance
rom = Uroman()

# Romanize text
result = rom.romanize("你好世界", lcode="zho")
print(result)  # "nihaoshijie"
```

## API Reference

### Functions

#### `romanize(text, lcode=None, format="str")`

Convenience function to romanize text without creating a Uroman instance.

**Parameters:**
- `text` (str): The text to romanize
- `lcode` (str, optional): ISO 639-3 language code (e.g., 'jpn', 'ara', 'zho')
- `format` (str, optional): Output format - 'str', 'edges', 'alts', or 'lattice'. Defaults to 'str'

**Returns:** String or list of Edge objects, depending on format

### Classes

#### `Uroman`

The main romanization class.

##### Methods

###### `romanize(text, lcode=None, format="str")`

Romanizes a given string.

**Parameters:**
- `text` (str): The text to romanize
- `lcode` (str, optional): ISO 639-3 language code
- `format` (str, optional): Output format

**Returns:** String or list of Edge objects

**Example:**
```python
uroman = Uroman()
result = uroman.romanize("こんにちは")
print(result)  # "kon'nichiha"
```

###### `romanize_escaped(text, lcode=None, format="str")`

Romanizes text with Unicode escape sequences decoded first.

**Parameters:**
- `text` (str): Text with possible \\uXXXX escape sequences
- `lcode` (str, optional): ISO 639-3 language code
- `format` (str, optional): Output format

**Returns:** String or list of Edge objects

###### `romanize_text(text, lcode=None, format="str", decode_unicode=False)`

Romanizes multi-line text, preserving line breaks.

**Parameters:**
- `text` (str): Multi-line text to romanize
- `lcode` (str, optional): ISO 639-3 language code
- `format` (str, optional): Output format
- `decode_unicode` (bool, optional): Whether to decode Unicode escapes

**Returns:** String with romanized text, newlines preserved

#### `Edge`

Represents a romanization edge with position and text information.

**Attributes:**
- `start` (int): Start position in the original text
- `end` (int): End position in the original text
- `text` (str): Romanized text
- `edge_type` (str): Type of the edge
- `is_numeric` (bool): Whether the edge represents a number
- `value` (float, optional): Numeric value if applicable
- `orig_text` (str, optional): Original text for numeric edges

## Output Formats

### `"str"` (default)

Returns a simple string with the romanized text.

```python
uroman.romanize("こんにちは")
# "kon'nichiha"
```

### `"edges"`

Returns a list of Edge objects representing the best romanization path.

```python
edges = uroman.romanize("こんにちは", format="edges")
for edge in edges:
    print(f"[{edge.start}:{edge.end}] {edge.text}")
```

### `"alts"`

Returns edges with alternative romanizations included.

### `"lattice"`

Returns all possible romanization edges (useful for advanced processing).

## Language Codes

uroman supports ISO 639-3 language codes. Some common codes:

- `jpn`: Japanese
- `ara`: Arabic
- `zho`: Chinese
- `hin`: Hindi
- `rus`: Russian
- `ell`: Greek
- `heb`: Hebrew
- `tha`: Thai

If no language code is provided, uroman will attempt to romanize the text automatically.


## Building Python Bindings

### Prerequisites

- Rust toolchain (stable)
- `uv` for Python dependency and build management


### Building the Python package

```bash
uv build
```

### Testing the bindings

```bash
pytest
```

To get print out some (crude) timing information:

```bash
pytest -s
```

To print out romanized data:

```bash
DEBUG=1 pytest -s
```
