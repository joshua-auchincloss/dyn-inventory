from pathlib import Path


README = Path(__file__).parent.parent / "README.md"
SRC = Path(__file__).parent.parent / "src/lib.rs"

def trim_prefix(s: str, pre: str):
    if s.startswith(pre):
        s = s[len(pre):]
    return s

def to_rs(ln: str):
    ln = ln.replace(
        "> [!TIP]", ""
    )
    ln = trim_prefix(
        ln, ">"
    )
    return ln

def get_readme():
    contents = README.read_text()
    out = ""
    for ln in contents.splitlines():
        out += "\n/// " + to_rs(ln)
    return out

def find_idx_of(haystack: list[str], needle: str)-> int:
    (idx, _) = next(filter(
        lambda v: needle in v[1], enumerate(haystack)
    ))
    return idx

def sub_readme(sub: str):
    contents = SRC.read_text().splitlines()
    start = find_idx_of(contents, "START OF README CONTENTS")
    end = find_idx_of(contents, "END OF README CONTENTS")
    sub = sub.splitlines()
    contents = [
        *contents[:start+1],
        *sub,
        *contents[end:]
    ]
    SRC.write_text("\n".join(contents))
    
if __name__ == "__main__":
    readme = get_readme()

    sub_readme(readme)