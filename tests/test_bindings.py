import os
import math
from pathlib import Path
from timeit import default_timer as timer
import subprocess

DATA_FILE = Path(__file__).parent / "test" / "multi-script.txt"


def time_call(func, *args, **kwargs):
    start = timer()
    result = func(*args, **kwargs)
    end = timer()
    return result, end - start


def avg(times):
    return sum(times) / len(times) if times else 0


def stddev(times, mean):
    return math.sqrt(sum((t - mean) ** 2 for t in times) / len(times)) if times else 0


def call_uroman_rs(s: str) -> str:
    result = subprocess.run(
        ["uroman-rs", s],
        input=s.encode("utf-8"),
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
    )
    if result.returncode != 0:
        raise RuntimeError(f"uroman-rs failed: {result.stderr.decode('utf-8')}")
    return result.stdout.decode("utf-8").rstrip("\n")


def test_convenience_function():
    from uroman_rs import romanize

    result = romanize("こんにちは")
    assert result == "konnichiha", f"Expected 'konnichiha', got '{result}'"


def test_uroman_class():
    from uroman_rs import Uroman

    uroman = Uroman()

    # Test Japanese
    result = uroman.romanize("こんにちは")
    assert result == "konnichiha", f"Expected 'konnichiha', got '{result}'"

    # Test Arabic with language code
    result = uroman.romanize("مرحبا", lcode="ara")
    assert result == "mrhba", f"Expected 'mrhba', got '{result}'"

    # Test Greek
    result = uroman.romanize("Γειά")
    assert result == "Geia", f"Expected 'Geia', got '{result}'"


def test_edges_format():
    from uroman_rs import Uroman

    uroman = Uroman()
    edges = uroman.romanize("こん", format="edges")

    assert isinstance(edges, list), "Edges should be a list"
    assert len(edges) > 0, "Should have at least one edge"
    assert hasattr(edges[0], "text"), "Edge should have 'text' attribute"
    assert hasattr(edges[0], "start"), "Edge should have 'start' attribute"
    assert hasattr(edges[0], "end"), "Edge should have 'end' attribute"


def test_multiline():
    from uroman_rs import Uroman

    uroman = Uroman()
    text = "こんにちは\nmrحba"
    result = uroman.romanize_text(text)
    lines = result.strip().split("\n")
    assert len(lines) == 2, f"Expected 2 lines, got {len(lines)}"


def test_comparison():
    import uroman as ur
    import uroman_rs

    uroman = uroman_rs.Uroman()
    uroman_orig = ur.Uroman()

    times_rs, times_orig, times_rs_shell = [], [], []
    with DATA_FILE.open("r", encoding="utf-8") as f:
        for line in f:
            text = line.strip()

            out_rs, time_rs = time_call(uroman.romanize, text)
            out_orig, time_orig = time_call(uroman_orig.romanize_string, text)
            out_rs_shell, time_rs_shell = time_call(call_uroman_rs, text)

            if int(os.getenv("DEBUG", 0)):
                print(out_rs)
                print(out_orig)
                print(out_rs_shell)

            assert out_rs == out_orig == out_rs_shell

            times_rs.append(time_rs)
            times_orig.append(time_orig)
            times_rs_shell.append(time_rs_shell)

    print("Timing results over", len(times_rs), "lines:\n")
    print("uroman_rs average time:", avg(times_rs))
    print("uroman average time:", avg(times_orig))
    print("uroman_rs (shell) average time:", avg(times_rs_shell))
    print()
    print("uroman_rs stddev time:", stddev(times_rs, avg(times_rs)))
    print("uroman stddev time:", stddev(times_orig, avg(times_orig)))
    print("uroman_rs (shell) stddev time:", stddev(times_rs_shell, avg(times_rs_shell)))
