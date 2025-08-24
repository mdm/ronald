#!/usr/bin/env -S uv run --script
from dataclasses import dataclass
import os


@dataclass
class Key:
    name: str
    x: int
    y: int
    width: int
    labels: list[str]


keys = [
    Key(name="Escape", x=0, y=0, width=100, labels=["ESC"]),
    Key(
        name="Key1",
        x=100,
        y=0,
        width=100,
        labels=[
            "!",
            "1",
        ],
    ),
    Key(
        name="Key2",
        x=200,
        y=0,
        width=100,
        labels=[
            '"',
            "2",
        ],
    ),
    Key(
        name="Key3",
        x=300,
        y=0,
        width=100,
        labels=[
            "#",
            "3",
        ],
    ),
    Key(
        name="Key4",
        x=400,
        y=0,
        width=100,
        labels=[
            "$",
            "4",
        ],
    ),
    Key(
        name="Key5",
        x=500,
        y=0,
        width=100,
        labels=[
            "%",
            "5",
        ],
    ),
    Key(
        name="Key6",
        x=600,
        y=0,
        width=100,
        labels=[
            "&amp;",
            "6",
        ],
    ),
    Key(
        name="Key7",
        x=700,
        y=0,
        width=100,
        labels=[
            "'",
            "7",
        ],
    ),
    Key(
        name="Key8",
        x=800,
        y=0,
        width=100,
        labels=[
            "(",
            "8",
        ],
    ),
    Key(
        name="Key9",
        x=900,
        y=0,
        width=100,
        labels=[
            ")",
            "9",
        ],
    ),
    Key(
        name="Key0",
        x=1000,
        y=0,
        width=100,
        labels=[
            "_",
            "0",
        ],
    ),
    Key(
        name="Minus",
        x=1100,
        y=0,
        width=100,
        labels=[
            "=",
            "-",
        ],
    ),
    Key(
        name="Caret",
        x=1200,
        y=0,
        width=100,
        labels=[
            "£",
            "↑",
        ],
    ),
    Key(name="Clear", x=1300, y=0, width=100, labels=["CLR"]),
    Key(name="Delete", x=1400, y=0, width=100, labels=["DEL"]),
    Key(name="Numpad7", x=1500, y=0, width=100, labels=["F7"]),
    Key(name="Numpad8", x=1600, y=0, width=100, labels=["F8"]),
    Key(name="Numpad9", x=1700, y=0, width=100, labels=["F9"]),
    Key(name="JoystickUp", x=2000, y=0, width=100, labels=["⇧"]),
    Key(name="Tab", x=0, y=100, width=125, labels=["TAB"]),
    Key(name="Q", x=125, y=100, width=100, labels=["Q"]),
    Key(name="W", x=225, y=100, width=100, labels=["W"]),
    Key(name="E", x=325, y=100, width=100, labels=["E"]),
    Key(name="R", x=425, y=100, width=100, labels=["R"]),
    Key(name="T", x=525, y=100, width=100, labels=["T"]),
    Key(name="Y", x=625, y=100, width=100, labels=["Y"]),
    Key(name="U", x=725, y=100, width=100, labels=["U"]),
    Key(name="I", x=825, y=100, width=100, labels=["I"]),
    Key(name="O", x=925, y=100, width=100, labels=["O"]),
    Key(name="P", x=1025, y=100, width=100, labels=["P"]),
    Key(
        name="At",
        x=1125,
        y=100,
        width=100,
        labels=[
            "|",
            "@",
        ],
    ),
    Key(
        name="BracketLeft",
        x=1225,
        y=100,
        width=100,
        labels=[
            "{",
            "[",
        ],
    ),
    Key(name="Enter", x=1325, y=100, width=175, labels=["RETURN"]),
    Key(name="Numpad4", x=1500, y=100, width=100, labels=["F4"]),
    Key(name="Numpad5", x=1600, y=100, width=100, labels=["F5"]),
    Key(name="Numpad6", x=1700, y=100, width=100, labels=["F6"]),
    Key(name="JoystickLeft", x=1900, y=100, width=100, labels=["⇦"]),
    Key(name="JoystickIcon", x=2000, y=100, width=100, labels=["🕹"]),
    Key(name="JoystickRight", x=2100, y=100, width=100, labels=["⇨"]),
    Key(
        name="CapsLock",
        x=0,
        y=200,
        width=150,
        labels=[
            "CAPS",
            "LOCK",
        ],
    ),
    Key(name="A", x=150, y=200, width=100, labels=["A"]),
    Key(name="S", x=250, y=200, width=100, labels=["S"]),
    Key(name="D", x=350, y=200, width=100, labels=["D"]),
    Key(name="F", x=450, y=200, width=100, labels=["F"]),
    Key(name="G", x=550, y=200, width=100, labels=["G"]),
    Key(name="H", x=650, y=200, width=100, labels=["H"]),
    Key(name="J", x=750, y=200, width=100, labels=["J"]),
    Key(name="K", x=850, y=200, width=100, labels=["K"]),
    Key(name="L", x=950, y=200, width=100, labels=["L"]),
    Key(
        name="Colon",
        x=1050,
        y=200,
        width=100,
        labels=[
            "*",
            ":",
        ],
    ),
    Key(
        name="Semicolon",
        x=1150,
        y=200,
        width=100,
        labels=[
            "+",
            ";",
        ],
    ),
    Key(
        name="BracketRight",
        x=1250,
        y=200,
        width=100,
        labels=[
            "}",
            "]",
        ],
    ),
    Key(name="Numpad1", x=1500, y=200, width=100, labels=["F1"]),
    Key(name="Numpad2", x=1600, y=200, width=100, labels=["F2"]),
    Key(name="Numpad3", x=1700, y=200, width=100, labels=["F3"]),
    Key(name="JoystickDown", x=2000, y=200, width=100, labels=["⇩"]),
    Key(name="ShiftLeft", x=0, y=300, width=200, labels=["SHIFT"]),
    Key(name="Z", x=200, y=300, width=100, labels=["Z"]),
    Key(name="X", x=300, y=300, width=100, labels=["X"]),
    Key(name="C", x=400, y=300, width=100, labels=["C"]),
    Key(name="V", x=500, y=300, width=100, labels=["V"]),
    Key(name="B", x=600, y=300, width=100, labels=["B"]),
    Key(name="N", x=700, y=300, width=100, labels=["N"]),
    Key(name="M", x=800, y=300, width=100, labels=["M"]),
    Key(
        name="Comma",
        x=900,
        y=300,
        width=100,
        labels=[
            "&lt;",
            ",",
        ],
    ),
    Key(
        name="Period",
        x=1000,
        y=300,
        width=100,
        labels=[
            "&gt;",
            ".",
        ],
    ),
    Key(
        name="Slash",
        x=1100,
        y=300,
        width=100,
        labels=[
            "?",
            "/",
        ],
    ),
    Key(
        name="Backslash",
        x=1200,
        y=300,
        width=100,
        labels=[
            "`",
            "\\",
        ],
    ),
    Key(name="ShiftRight", x=1300, y=300, width=200, labels=["SHIFT"]),
    Key(name="Numpad0", x=1500, y=300, width=100, labels=["F0"]),
    Key(name="ArrowUp", x=1600, y=300, width=100, labels=["⇧"]),
    Key(name="NumpadPeriod", x=1700, y=300, width=100, labels=["."]),
    Key(name="JoystickFire1", x=1900, y=300, width=100, labels=["FIRE 1"]),
    Key(name="JoystickFire2", x=2000, y=300, width=100, labels=["FIRE 2"]),
    Key(name="JoystickFire3", x=2100, y=300, width=100, labels=["FIRE 3"]),
    Key(name="Control", x=0, y=400, width=200, labels=["CONTROL"]),
    Key(name="Copy", x=200, y=400, width=175, labels=["COPY"]),
    # Key(name="Space", x=375, y=400, width=800, labels=[""]),
    Key(name="NumpadEnter", x=1175, y=400, width=325, labels=["ENTER"]),
    Key(name="ArrowLeft", x=1500, y=400, width=100, labels=["⇦"]),
    Key(name="ArrowDown", x=1600, y=400, width=100, labels=["⇩"]),
    Key(name="ArrowRight", x=1700, y=400, width=100, labels=["⇨"]),
]


def main():
    for i, key in enumerate(keys):
        print(f"Processing key: {key.name} ({i + 1}/{len(keys)})")
        output_filename = f"../assets/keys/{key.name}.partial.svg"
        try:
            os.remove(output_filename)
        except FileNotFoundError:
            pass

        svg = '<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 2200 500">'
        x = key.x + key.width // 2
        if len(key.labels) == 1:
            if key.name in ["JoystickFire1", "JoystickFire2", "JoystickFire3"]:
                size = 25
            elif key.name in ["JoystickIcon"]:
                size = 70
            else:
                size = 30

            y = key.y + 50
            svg += f'<text x="{x}" y="{y}" font-family="Open Sans" font-size="{size}" fill="white" stroke="white" text-anchor="middle" dominant-baseline="middle">{key.labels[0]}</text>'
        else:
            size = 30
            y = key.y + 35
            svg += f'<text x="{x}" y="{y}" font-family="Open Sans" font-size="{size}" fill="white" stroke="white" text-anchor="middle" dominant-baseline="middle">{key.labels[0]}</text>'
            y = key.y + 70
            svg += f'<text x="{x}" y="{y}" font-family="Open Sans" font-size="{size}" fill="white" stroke="white" text-anchor="middle" dominant-baseline="middle">{key.labels[1]}</text>'

        svg += "</svg>"
        with open("temp_in.svg", "wb") as f:
            f.write(svg.encode("utf-8"))

        # call inkscape to convert the SVG to a path
        os.system(
            'inkscape --actions "select-by-element:text;object-to-path;" temp_in.svg -o temp_out.svg'
        )

        with open("temp_out.svg", "rb") as file_in:
            with open(output_filename, "wb") as file_out:
                extract = False
                for line in file_in:
                    if b"<path" in line:
                        extract = True
                    if b"</g>" in line:
                        extract = False

                    if extract:
                        file_out.write(line)

        try:
            os.remove("temp_in.svg")
            os.remove("temp_out.svg")
        except FileNotFoundError:
            pass


if __name__ == "__main__":
    main()
