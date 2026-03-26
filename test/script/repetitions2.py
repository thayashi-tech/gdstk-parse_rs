import pathlib
import gdstk


if __name__ == "__main__":
    # X-explicit repetition
    vline = gdstk.FlexPath([(3, 2), (3, 3.5)], 0.1, simple_path=True)
    vline.repetition = gdstk.Repetition(x_offsets=[0.2, 0.6, 1.4, 3.0])

    # Y-explicit repetition
    hline = gdstk.RobustPath((3, 2), 0.05, simple_path=True)
    hline.segment((6, 2))
    hline.repetition = gdstk.Repetition(y_offsets=[0.1, 0.3, 0.7, 1.5])

    # Create all copies
    vlines = vline.apply_repetition()
    hlines = hline.apply_repetition()

    # Include original elements for boolean operation
    vlines.append(vline)
    hlines.append(hline)

    result = gdstk.boolean(vlines, hlines, "or")

    main = gdstk.Cell("Main")
    main.add(*result)

    lib = gdstk.Library()
    lib.add(main, *main.dependencies(True))
    path = pathlib.Path(__file__).parent.parent.absolute() / "output"
    lib.write_oas(path / "transformation2.oas")
