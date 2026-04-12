import pathlib
import gdstk


if __name__ == "__main__":
    path = pathlib.Path(__file__).parent.parent.absolute() / "testdata"

    unit = gdstk.Cell("Unit")
    unit.add(gdstk.cross((0, 0), 1, 0.2))

    main = gdstk.Cell("Main")

    # Create repeating pattern using references
    d = 2
    ref1 = gdstk.Reference(unit, columns=11, rows=6, spacing=(d, d * 3**0.5))
    ref2 = gdstk.Reference(
        unit, (d / 2, d * 3**0.5 / 2), columns=10, rows=5, spacing=(d, d * 3**0.5)
    )
    main.add(ref1, ref2)
    main.flatten()

    hole = gdstk.text("PY", 8 * d, (0.5 * d, 0), layer=1)
    for pol in main.polygons:
        if gdstk.any_inside(pol.points, hole):
            main.remove(pol)

    main.add(*hole)

    gdstk.Library().add(main).write_oas(path / "pos_filtering.oas")
