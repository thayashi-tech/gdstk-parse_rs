import numpy
import gdstk
import pathlib


if __name__ == "__main__":
    main = gdstk.Cell("Main")

    # Create a path
    path = gdstk.RobustPath((0, 0), 0.5)
    path.segment((8, 0))
    path.interpolation(
        [(2, -4), (-2, -6), (-5, -8), (-4, -12)],
        angles=[0, None, None, None, -numpy.pi / 4],
        relative=True,
    )
    path.segment((3, -3), relative=True)
    main.add(path)

    # Major and minor markers
    major = gdstk.regular_polygon((0, 0), 0.5, 6, layer=1)
    minor = gdstk.rectangle((-0.1, -0.5), (0.1, 0.5), layer=1)
    for s in range(path.size):
        # A major marker is added at the start of each path section
        m = major.copy()
        m.translate(path.position(s))
        main.add(m)
        for u in numpy.linspace(0, 1, 5)[1:-1]:
            # Each section receives 3 equally-spaced minor markers
            # rotated to be aligned to the path direction
            m = minor.copy()
            grad = path.gradient(s + u)
            m.rotate(numpy.arctan2(grad[1], grad[0]))
            m.translate(path.position(s + u))
            main.add(m)
    # Add a major marker at the end of the path
    major.translate(path.position(path.size))
    main.add(major)

    lib = gdstk.Library()
    lib.add(main, *main.dependencies(True))
    path = pathlib.Path(__file__).parent.parent.absolute() / "testdata"
    lib.write_oas(path / "points_along_path.oas")
