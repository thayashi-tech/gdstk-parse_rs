import pathlib
import gdstk
import numpy


if __name__ == "__main__":
    d = 2
    f = gdstk.text("F", 8 * d, (0.5 * d, 0), layer=1)
    ref = gdstk.Cell("ref")
    ref.add(*f)

    f2 = gdstk.text("F", 8 * d, (0.5 * d, 0), layer=2)
    ref2 = gdstk.Cell("ref2")
    ref2.add(*f2)

    f3 = gdstk.text("F", 8 * d, (0.5 * d, 0), layer=30)
    ref3 = gdstk.Cell("ref3")
    ref3.add(*f3)

    f4 = gdstk.text("F", 8 * d, (0.5 * d, 0), layer=30)
    ref4 = gdstk.Cell("ref4")
    ref4.add(*f4)

    refs = gdstk.Cell("refs")
    refs.add(gdstk.Reference(ref))
    refs.add(gdstk.Reference(ref2, x_reflection=True))
    refs.add(gdstk.Reference(ref, rotation=numpy.pi))
    refs.add(gdstk.Reference(ref2, rotation=numpy.pi, x_reflection=True))

    refs.add(
        gdstk.Reference(
            ref3,
            rotation=numpy.pi / 2,
            x_reflection=True,
            origin=(8, 8),
            magnification=0.5,
        )
    )

    main = gdstk.Cell("Main")
    main.add(
        gdstk.Reference(
            refs,
            columns=6,
            rows=4,
            spacing=(60, 60),
        )
    )

    lib = gdstk.Library()
    lib.add(main, *main.dependencies(True))
    path = pathlib.Path(__file__).parent.parent.absolute() / "testdata"
    lib.write_oas(path / "figure_transform.oas")
