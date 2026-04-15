#!/bin/env python
import subprocess
from pathlib import Path

RELEASE = "release"
O2I = f"../target/{RELEASE}/examples/oasis_to_image"
CUTOUT = f"../target/{RELEASE}/examples/cutout"
DATADIR = "testdata"
OUTPUT = "output"


def test_image_to_oasis_area():
    areas = [
        [100, 50, 200, 80],
        [200, 50, 300, 80],
        [100, 80, 200, 110],
        [200, 80, 300, 110],
    ]
    input_file = f"{DATADIR}/figure_transform.oas"
    input = Path(input_file).stem
    for i, area in enumerate(areas):
        cmd = [
            O2I,
            input_file,
            "--cell-bounds",
            "--polygon-bounds",
            "--top-cell",
            "0",
            "--area",
            ",".join(map(str, area)),
            "-o",
            f"{OUTPUT}/io2_{input}_{i}.png",
        ]
        print(" ".join(cmd))
        subprocess.run(cmd)


def test_image_to_oasis_inner(filename: str, topcell: int):
    input_file = f"{DATADIR}/{filename}"
    input = Path(input_file).stem
    cmd = [
        O2I,
        input_file,
        "--cell-bounds",
        "--polygon-bounds",
        "--top-cell",
        f"{topcell}",
        "-o",
        f"{OUTPUT}/io2_{input}_all.png",
    ]
    print(" ".join(cmd))
    subprocess.run(cmd)


def test_image_to_oasis():
    # all
    test_image_to_oasis_inner("figure_transform.oas", 0)
    test_image_to_oasis_inner("photonics.oas", 1)
    test_image_to_oasis_inner("layout.oas", 0)
    test_image_to_oasis_inner("points_along_path.oas", 0)
    test_image_to_oasis_inner("transformation.oas", 0)
    test_image_to_oasis_inner("transformation2.oas", 0)
    test_image_to_oasis_inner("connection_pads.oas", 0)


def test_cutout_area(do_clip: bool = False):
    areas = [
        [100, 50, 200, 80],
        [200, 50, 300, 80],
        [100, 80, 200, 110],
        [200, 80, 300, 110],
    ]
    input_file = f"{DATADIR}/figure_transform.oas"
    input = Path(input_file).stem
    clip = "clip" if do_clip else "extract"
    for i, area in enumerate(areas):
        name = f"cutout_{input}_{i}_{clip}"
        clip_data = f"{OUTPUT}/{name}.oas"
        cmd = [
            CUTOUT,
            input_file,
            "--top-cell",
            "0",
            "--area",
            ",".join(map(str, area)),
            "-o",
            clip_data,
        ]
        if do_clip:
            cmd.append("--clip")

        print(" ".join(cmd))
        subprocess.run(cmd)

        cmd = [
            O2I,
            clip_data,
            "--area",
            ",".join(map(str, area)),
            "-o",
            f"{OUTPUT}/{name}.png",
        ]
        print(" ".join(cmd))
        subprocess.run(cmd)


test_image_to_oasis()
test_image_to_oasis_area()
test_cutout_area(False)
test_cutout_area(True)
