import gdstk
import pathlib
import numpy


def grating(
    period, fill_frac=0.5, length=20, width=25, layer=0, datatype=0, cell_name="Grating"
):
    """
    Straight grating:

    Args:
        period: Grating period.
        fill_frac: Filling fraction of the teeth (wrt period).
        length: Length of the grating.
        width: Width of the grating.
        layer: GDSII layer number
        datatype: GDSII data type number

    Return:
        gdstk.Cell
    """
    result = gdstk.Cell(cell_name)
    x = width / 2
    w = period * fill_frac
    result.add(
        gdstk.rectangle(
            (-x, y * period), (x, y * period + w), layer=layer, datatype=datatype
        )
        for y in range(int(length / period))
    )
    return result


if __name__ == "__main__":
    lib = gdstk.Library()

    length = 20
    grat1 = grating(3.5, length=length, layer=1, cell_name="Grating 1")
    grat2 = grating(3.0, length=length, layer=1, cell_name="Grating 2")
    lib.add(grat1, grat2)

    main = lib.new_cell("Main")
    main.add(gdstk.rectangle((0, -10), (150, 10)))
    main.add(gdstk.Reference(grat1, (length, 0), rotation=numpy.pi / 2))
    main.add(gdstk.Reference(grat2, (150 - length, 0), rotation=-numpy.pi / 2))

    path = pathlib.Path(__file__).parent.parent.absolute() / "output"
    lib.write_gds(path / "pcell.gds")
