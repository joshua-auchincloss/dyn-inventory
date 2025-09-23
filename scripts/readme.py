from pathlib import Path


README = Path(__file__).parent.parent / "README.md"
SRC = Path(__file__).parent.parent / "src/lib.rs"

def to_rs(ln: str):
    return ln.replace(
        "> [!TIP]", ""
    ).lstrip(
        "> "
    )

def get_readme():
    contents = README.read_text()
    out = ""
    for ln in contents.splitlines():
        out += "\n/// " + to_rs(ln)
    return out

def sub_readme(sub: str):
    contents = SRC.read_text().splitlines()
    (end, _) = next(filter(
        lambda v: "END OF README CONTENTS" in v[1], enumerate(contents)
    ))
    sub = sub.splitlines()
    contents = [
        contents[0],
        *sub,
        *contents[end:]
    ]
    SRC.write_text("\n".join(contents))
    
if __name__ == "__main__":
    readme = get_readme()

    sub_readme(readme)