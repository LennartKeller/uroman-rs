#!/usr/bin/env python3
"""
Example usage of the uroman-rs Python bindings.

This demonstrates how to use the uroman_rs library from Python to romanize
text in various writing systems.
"""

from uroman_rs import Uroman, romanize

def main():
    print("=== Uroman Python Bindings Example ===\n")

    # Using the convenience function
    print("1. Using the convenience function:")
    result = romanize("こんにちは")
    print(f"   Input:  こんにちは")
    print(f"   Output: {result}\n")

    # Creating a Uroman instance
    print("2. Using the Uroman class:")
    uroman = Uroman()

    # Japanese
    text = "こんにちは、世界！"
    result = uroman.romanize(text)
    print(f"   Japanese:  {text}")
    print(f"   Result:    {result}\n")

    # Arabic
    text = "مرحبا بالعالم"
    result = uroman.romanize(text, lcode="ara")
    print(f"   Arabic:    {text}")
    print(f"   Result:    {result}\n")

    # Chinese
    text = "你好世界"
    result = uroman.romanize(text, lcode="zho")
    print(f"   Chinese:   {text}")
    print(f"   Result:    {result}\n")

    # Hindi
    text = "नमस्ते दुनिया"
    result = uroman.romanize(text, lcode="hin")
    print(f"   Hindi:     {text}")
    print(f"   Result:    {result}\n")

    # Greek
    text = "Γειά σου Κόσμε"
    result = uroman.romanize(text)
    print(f"   Greek:     {text}")
    print(f"   Result:    {result}\n")

    # Russian
    text = "Привет, мир!"
    result = uroman.romanize(text)
    print(f"   Russian:   {text}")
    print(f"   Result:    {result}\n")

    # Using edges format to get detailed information
    print("3. Using edges format for detailed output:")
    text = "こんにちは"
    edges = uroman.romanize(text, format="edges")
    print(f"   Input: {text}")
    print(f"   Edges:")
    for edge in edges:
        print(f"     {edge}")
    print()

    # Multi-line romanization
    print("4. Multi-line romanization:")
    multiline = """こんにちは
مرحبا
Привет"""
    result = uroman.romanize_text(multiline)
    print(f"   Input:\n{multiline}")
    print(f"   Output:\n{result}")

if __name__ == "__main__":
    main()
